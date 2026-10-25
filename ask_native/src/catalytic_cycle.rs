//! The native catalytic working/return strokes, shared with numerical carriers.
//! Ordinal scales are supplied by the native catalog interpreter, not copied here.
pub type Tuple = [Option<u8>; 12];

pub fn transfer(donor:&Tuple,acceptor:&Tuple,maxima:&[u8;12])
    ->Result<(Tuple,Tuple),&'static str>{
    let d=donor[11].ok_or("donor has no winding coordinate")?;
    let a=acceptor[11].ok_or("acceptor has no winding coordinate")?;
    if d==0{return Err("donor winding is empty");}
    if a>=maxima[11]{return Err("acceptor winding is saturated");}
    let mut donor=*donor;let mut acceptor=*acceptor;
    donor[11]=Some(d-1);acceptor[11]=Some(a+1);Ok((donor,acceptor))
}
pub fn imprint(catalyst:&Tuple,substrate:&mut Tuple,maxima:&[u8;12])->Option<&'static str>{
    let mut best:Option<(usize,usize,&'static str,f32)>=None;
    for (a,b,label) in [(1,9,"T↔H"),(2,10,"R↔S")]{
        if let (Some(x),Some(y))=(catalyst[a],catalyst[b]){
            let norm=|v:u8,i:usize|if maxima[i]==0{0.0}else{v as f32/maxima[i] as f32};
            let delta=norm(x,a)-norm(y,b);
            if best.map_or(true,|(_,_,_,old)|delta.abs()>old.abs()){best=Some((a,b,label,delta));}
        }
    }
    let (a,b,label,delta)=best?;if delta==0.0{return None;}
    let direction=if delta>0.0{1i16}else{-1};
    for (at,step) in [(a,direction),(b,-direction)]{
        if let Some(v)=substrate[at]{substrate[at]=Some((v as i16+step).rem_euclid(maxima[at] as i16+1) as u8);}
    }
    Some(label)
}
pub fn working(catalyst:&Tuple,substrate:&Tuple,maxima:&[u8;12])
    ->Result<(Tuple,Tuple,&'static str,Option<&'static str>),&'static str>{
    let (spent,mut product,direction)=match transfer(catalyst,substrate,maxima){
        Ok((c,s))=>(c,s,"reductive"),
        Err(_)=>{let (s,c)=transfer(substrate,catalyst,maxima)?;(c,s,"oxidative")}
    };
    let pair=imprint(catalyst,&mut product,maxima);Ok((spent,product,direction,pair))
}
pub fn restore(spent:&Tuple,direction:&str,maxima:&[u8;12])->Result<Tuple,&'static str>{
    let w=spent[11].ok_or("spent catalyst has no winding coordinate")?;
    let restored=match direction{
        "reductive"=>w.checked_add(1).filter(|&v|v<=maxima[11]),
        "oxidative"=>w.checked_sub(1),_=>return Err("unknown catalytic direction")
    }.ok_or("terminal carrier exchange exceeds the winding boundary")?;
    let mut returned=*spent;returned[11]=Some(restored);Ok(returned)
}
