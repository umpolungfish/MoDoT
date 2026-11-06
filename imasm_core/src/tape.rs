//! Native executable morphism tape: it delegates every operation to the
//! canonical check::Graph, flow::flow_values, and SIXTEEN_3 Reg16_3.
//! Values are retained as evidence but cannot masquerade as opcode execution.
//! A graph verdict is distinct from its carrier readout (including B).

use alloc::{string::{String, ToString}, vec::Vec};
use crate::{check::{ClosureState, Graph}, classic::Token,
    flow::flow_values, imasm16_3::Reg16_3};

#[derive(Clone)]
pub struct TapeProgram {
    pub id: String,
    pub graph: Graph,
    pub readout: usize,
}

impl TapeProgram {
    pub fn new(id: String, graph: Graph, readout: usize) -> Result<Self, String> {
        if id.is_empty() || graph.nodes.is_empty() { return Err("empty program".into()); }
        if graph.edges.iter().any(|&(a,b)| a>=graph.nodes.len() || b>=graph.nodes.len()) {
            return Err("edge outside graph".into());
        }
        if readout>=graph.nodes.len() || graph.nodes[readout]!=Token::Tanch {
            return Err("readout must be a TANCH".into());
        }
        let bad=graph.validate();
        if !bad.is_empty() { return Err(alloc::format!("ill-typed graph: {}",bad.join("; "))); }
        Ok(Self {id,graph,readout})
    }

    // Graph verdict on ACTUAL edges, not on a hypothetical word or ring.
    pub fn verdict(&self) -> (char,ClosureState) {
        let state=self.graph.closure_state();
        let v=match state {
            ClosureState::Closed(_) if self.graph.nodes.contains(&Token::Engagr) => 'B',
            ClosureState::Closed(_) => 'T',
            ClosureState::Open => 'B',
            ClosureState::Identity | ClosureState::None => 'N',
        };
        (v,state)
    }

    pub fn fix_upstream_of_readout(&self) -> bool {
        let mut reached=alloc::vec![false;self.graph.nodes.len()];
        let mut todo=alloc::vec![self.readout];
        reached[self.readout]=true;
        while let Some(i)=todo.pop() {
            for &(a,b) in &self.graph.edges {
                if b==i && !reached[a] {reached[a]=true;todo.push(a);}
            }
        }
        self.graph.nodes.iter().enumerate()
            .any(|(i,&t)| reached[i] && t==Token::Ifix)
    }

    // Bind explicit living ports. Native validation refuses over-valence.
    pub fn compose(id:String, a:&Self, b:&Self, links:&[(usize,usize)]) -> Result<Self,String> {
        let n=a.graph.nodes.len();
        let mut g=Graph::new();
        g.nodes.extend(a.graph.nodes.iter().copied());
        g.nodes.extend(b.graph.nodes.iter().copied());
        g.edges.extend(a.graph.edges.iter().copied());
        g.edges.extend(b.graph.edges.iter().map(|&(i,j)|(i+n,j+n)));
        for &(i,j) in links {
            if i>=n || j>=b.graph.nodes.len() {return Err("invalid composition port".into());}
            g.connect(i,j+n);
        }
        Self::new(id,g,b.readout+n)
    }
}

#[derive(Clone)]
pub struct TapeSample {
    pub source:String,
    pub program:String,
    pub observed_at_ns:i64,
    pub available_at_ns:i64,
    pub as_of_ns:i64,
    pub seed:Option<Reg16_3>,
    pub payload:Vec<u8>,
}

#[derive(Clone)]
pub struct TapeFrame {
    pub sequence:usize,
    pub source:String,
    pub program:String,
    pub observed_at_ns:i64,
    pub available_at_ns:i64,
    pub input:Reg16_3,
    pub readout:Reg16_3,
    pub verdict:char,
    pub closure:ClosureState,
    pub committed:bool,
    pub payload:Vec<u8>,
}

pub struct TapeMachine {
    programs:Vec<TapeProgram>,
    pub working:Reg16_3,
    pub last_commit:Option<Reg16_3>,
    pub frames:Vec<TapeFrame>,
    last_available:Option<i64>,
    next_sequence:usize,
}

impl TapeMachine {
    pub fn new(seed:Reg16_3) -> Self {
        Self{programs:Vec::new(),working:seed,last_commit:None,
             frames:Vec::new(),last_available:None,next_sequence:0}
    }
    pub fn register(&mut self,p:TapeProgram)->Result<(),String>{
        if self.programs.iter().any(|x|x.id==p.id){return Err("duplicate program".into());}
        self.programs.push(p);
        Ok(())
    }
    pub fn program(&self,id:&str)->Option<&TapeProgram>{
        self.programs.iter().find(|p|p.id==id)
    }
    /// Release completed execution receipts after they have been persisted.
    /// The next sequence number remains monotonic across releases.
    pub fn release_completed(&mut self) { self.frames.clear(); }

    pub fn execute(&mut self,s:TapeSample)->Result<&TapeFrame,String>{
        if s.source.is_empty(){return Err("source required".into());}
        if s.observed_at_ns>s.available_at_ns {return Err("availability precedes observation".into());}
        if s.available_at_ns>s.as_of_ns {return Err("future observation".into());}
        if self.last_available.map_or(false,|v|s.available_at_ns<v) {
            return Err("out of order publication".into());
        }
        let program=self.program(&s.program)
            .ok_or_else(||"unregistered IMASM program".to_string())?;
        let input=s.seed.unwrap_or(self.working);
        let (node_in,_)=flow_values(&program.graph,input);
        let readout=node_in[program.readout];
        let (verdict,closure)=program.verdict();
        let committed=program.fix_upstream_of_readout();
        let frame=TapeFrame{sequence:self.next_sequence,source:s.source,program:s.program,
            observed_at_ns:s.observed_at_ns,available_at_ns:s.available_at_ns,
            input,readout,verdict,closure,committed,payload:s.payload};
        self.next_sequence+=1;
        self.working=readout;
        if committed {self.last_commit=Some(readout);}
        self.last_available=Some(frame.available_at_ns);
        self.frames.push(frame);
        Ok(self.frames.last().expect("appended frame"))
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::state_order;

    fn graph(engage:bool,fix:bool)->Graph{
        // Three actual FSPLIT arms: T, F, and intact t/f block.
        let mut g=Graph::new();
        for t in [Token::Vinit,Token::Fsplit3,Token::Evalt,Token::Evalf,
                  Token::Imscrib,Token::Ffuse3]{g.add(t);}
        for (a,b) in [(0,1),(1,2),(1,3),(1,4),(2,5),(3,5),(4,5)]{g.connect(a,b);}
        let mut tail=5;
        if engage {let i=g.add(Token::Engagr);g.connect(tail,i);tail=i;}
        if fix {let i=g.add(Token::Ifix);g.connect(tail,i);tail=i;}
        let j=g.add(Token::Tanch);g.connect(tail,j);g
    }
    #[test]
    fn all_sixteen_carriers_execute_the_native_three_arm_morphism(){
        for seed in state_order(){
            for (engage,fix) in [(false,false),(true,true)]{
                let g=graph(engage,fix);
                let p=TapeProgram::new("p".into(),g.clone(),g.nodes.len()-1).unwrap();
                assert_eq!(p.verdict().0,if engage {'B'}else{'T'});
                let mut m=TapeMachine::new(seed);m.register(p).unwrap();
                let f=m.execute(TapeSample{source:"coop.x".into(),program:"p".into(),
                    observed_at_ns:1,available_at_ns:2,as_of_ns:2,
                    seed:None,payload:alloc::vec![1,2,3]}).unwrap();
                assert!(f.readout==if engage{seed.engagr()}else{seed});
                assert_eq!(f.payload,alloc::vec![1,2,3]);
                assert_eq!(f.committed,fix);
            }
        }
    }
    #[test]
    fn causal_time_and_graph_valence_are_enforced(){
        let mut g=graph(false,false);
        let readout=g.nodes.len()-1;
        g.connect(0,1); // VINIT illegally fans out
        assert!(TapeProgram::new("bad".into(),g,readout).is_err());
        let g=graph(false,false);
        let mut m=TapeMachine::new(Reg16_3::default());
        m.register(TapeProgram::new("p".into(),g.clone(),g.nodes.len()-1).unwrap()).unwrap();
        let a=TapeSample{source:"x".into(),program:"p".into(),
            observed_at_ns:10,available_at_ns:20,as_of_ns:19,
            seed:None,payload:Vec::new()};
        assert!(m.execute(a.clone()).is_err());
        assert!(m.execute(TapeSample{as_of_ns:20,..a.clone()}).is_ok());
        assert!(m.execute(TapeSample{available_at_ns:19,as_of_ns:30,..a}).is_err());
    }
    #[test]
    fn composition_respects_native_ports(){
        let mut a=Graph::new();a.add(Token::Vinit);a.add(Token::Tanch);a.connect(0,1);
        let mut b=Graph::new();b.add(Token::Afwd);b.add(Token::Tanch);b.connect(0,1);
        let p=TapeProgram::new("a".into(),a,1).unwrap();
        let q=TapeProgram::new("b".into(),b,1).unwrap();
        let c=TapeProgram::compose("ab".into(),&p,&q,&[(1,0)]).unwrap();
        assert_eq!(c.graph.edges.len(),3);
        assert_eq!(c.verdict().0,'N'); // no dyad, not a fabricated closure
        assert!(TapeProgram::compose("bad".into(),&p,&q,&[(1,1)]).is_err());
    }
}
