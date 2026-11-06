//! Streaming JSONL entrypoint for executable IMASM tape.
//!
//! Each register command defines an actual graph (opcode nodes and explicit
//! edges). Each sample executes it on the native SIXTEEN_3 carrier, checks
//! publication times, and emits the full readout plus the original payload.
//! No source-name -> opcode classification is invented.
//!
//! Usage:
//!   cargo run --manifest-path imasm_core/Cargo.toml --bin imasm_tape < tape.jsonl
//!
//! Supported records:
//!   {"kind":"register","id":"p","nodes":["VINIT", ... ],
//!    "edges":[[0,1],...],"readout":8}
//!   {"kind":"compose","id":"ab","left":"a","right":"b",
//!    "links":[[1,0]]}
//!   {"kind":"sample","source":"coop.x","program":"p",
//!    "observed_at_ns":1,"available_at_ns":2,"as_of_ns":2,
//!    "seed":"TF","payload":{"x":1.25}}
//!
//! A file of successful samples is a reproducible, executable morphism tape.

use imasm_core::{
    check::{ClosureState, Graph},
    classic::Token,
    imasm16_3::Reg16_3,
    tape::{TapeMachine, TapeProgram, TapeSample},
};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

fn required<'a>(v: &'a Value, name: &str) -> Result<&'a str, String> {
    v.get(name).and_then(Value::as_str)
        .ok_or_else(|| format!("{name}: expected string"))
}
fn int(v: &Value, name: &str) -> Result<i64, String> {
    v.get(name).and_then(Value::as_i64)
        .ok_or_else(|| format!("{name}: expected signed integer nanoseconds"))
}
fn list_edges(v: &Value, key: &str) -> Result<Vec<(usize,usize)>, String> {
    let entries=v.get(key).and_then(Value::as_array)
        .ok_or_else(||format!("{key}: expected pairs of node indices"))?;
    let mut result=Vec::with_capacity(entries.len());
    for edge in entries {
        let pair=edge.as_array()
            .ok_or_else(||"edge must be an index pair".to_string())?;
        if pair.len()!=2 {return Err("edge must have exactly two ends".into());}
        let a=pair[0].as_u64()
            .ok_or_else(||"negative or noninteger node index".to_string())? as usize;
        let b=pair[1].as_u64()
            .ok_or_else(||"negative or noninteger node index".to_string())? as usize;
        result.push((a,b));
    }
    Ok(result)
}
fn register(v:&Value, m:&mut TapeMachine)->Result<Value,String>{
    let id=required(v,"id")?.to_string();
    let mut g=Graph::new();
    let nodes=v.get("nodes").and_then(Value::as_array)
        .ok_or_else(||"nodes array required".to_string())?;
    for n in nodes {
        let token=Token::parse(n.as_str().ok_or("opcode must be a string")?)
            .ok_or_else(||format!("unknown IMASM opcode: {n}"))?;
        g.add(token);
    }
    for (a,b) in list_edges(v,"edges")? {g.connect(a,b);}
    let readout=v.get("readout").and_then(Value::as_u64)
        .ok_or_else(||"TANCH readout index required".to_string())? as usize;
    let p=TapeProgram::new(id.clone(),g,readout)?;
    let (verdict,closure)=p.verdict();
    m.register(p)?;
    Ok(json!({"kind":"registered","id":id,
        "verdict":verdict.to_string(),
        "closure":closure_name(closure),"native":true}))
}
fn closure_name(c:ClosureState)->String {
    match c {
        ClosureState::Closed(n)=>format!("closed:{n}"),
        ClosureState::Open=>"open".into(),
        ClosureState::Identity=>"identity".into(),
        ClosureState::None=>"none".into(),
    }
}
fn compose(v:&Value,m:&mut TapeMachine)->Result<Value,String>{
    let id=required(v,"id")?.to_string();
    let a=m.program(required(v,"left")?)
        .ok_or_else(||"unregistered left operand".to_string())?;
    let b=m.program(required(v,"right")?)
        .ok_or_else(||"unregistered right operand".to_string())?;
    let links=list_edges(v,"links")?;
    let p=TapeProgram::compose(id.clone(),a,b,&links)?;
    let (verdict,closure)=p.verdict();
    m.register(p)?;
    Ok(json!({"kind":"composed","id":id,
        "verdict":verdict.to_string(),
        "closure":closure_name(closure),"native":true}))
}
fn sample(v:&Value,m:&mut TapeMachine)->Result<Value,String>{
    let seed=match v.get("seed") {
        Some(x) if !x.is_null() => Some(Reg16_3::from_name(
            x.as_str().ok_or("seed must name a SIXTEEN_3 carrier")?)
            .ok_or_else(||"unknown SIXTEEN_3 carrier name".to_string())?),
        _ => None,
    };
    let payload=v.get("payload").cloned().unwrap_or(Value::Null);
    let input=TapeSample{
        source:required(v,"source")?.to_string(),
        program:required(v,"program")?.to_string(),
        observed_at_ns:int(v,"observed_at_ns")?,
        available_at_ns:int(v,"available_at_ns")?,
        as_of_ns:int(v,"as_of_ns")?,
        seed,
        payload:serde_json::to_vec(&payload).map_err(|e|e.to_string())?,
    };
    let f=m.execute(input)?;
    let evidence:Value=serde_json::from_slice(&f.payload)
        .map_err(|e|e.to_string())?;
    Ok(json!({"kind":"executed","sequence":f.sequence,
        "source":f.source,"program":f.program,
        "observed_at_ns":f.observed_at_ns,
        "available_at_ns":f.available_at_ns,
        "input":f.input.name(),"readout":f.readout.name(),
        "verdict":f.verdict.to_string(),
        "closure":closure_name(f.closure),
        "committed":f.committed,
        "payload":evidence,"native":true}))
}
fn process(v:&Value,m:&mut TapeMachine)->Result<Value,String>{
    match required(v,"kind")? {
        "register"=>register(v,m),
        "compose"=>compose(v,m),
        "sample"=>sample(v,m),
        other=>Err(format!("unknown tape command: {other}")),
    }
}
fn main(){
    let stdin=io::stdin();
    let mut stdout=io::stdout().lock();
    // N is only the starting state; callers may supply any of 16 seeds.
    let mut machine=TapeMachine::new(Reg16_3::default());
    for (line_number,line) in stdin.lock().lines().enumerate(){
        let line=match line {
            Ok(s)=>s,Err(e)=>{
                eprintln!("input line {}: {}",line_number+1,e);
                std::process::exit(2);
            },
        };
        if line.trim().is_empty(){continue;}
        let result=serde_json::from_str::<Value>(&line)
            .map_err(|e|e.to_string())
            .and_then(|v|process(&v,&mut machine));
        let response=match result {
            Ok(value)=>value,
            Err(msg)=>{
                let _=writeln!(stdout,"{}",json!({"kind":"error",
                    "line":line_number+1,"error":msg}));
                let _=stdout.flush();
                std::process::exit(2);
            },
        };
        if writeln!(stdout,"{response}").is_err() || stdout.flush().is_err() {
            std::process::exit(3);
        }
        // Durable receipt is now in the output stream; release in-memory
        // execution history without restarting the carrier or sequence.
        machine.release_completed();
    }
}
