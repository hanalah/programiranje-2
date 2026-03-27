use std::ops::*;
use std::fmt::*;

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz<T> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
    Spremenjlivka(String),
}

impl<T> Izraz<T> {
    fn konst(v:T) -> Self {
        Izraz::Konstanta(v)
    } 

    fn spr(ime: &str) -> Self {
        Izraz::Spremenjlivka(ime.to_string())
    }
    
    fn collect(&self) -> u32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Spremenjlivka(_) => 1,
            Izraz::Operacija(l,
                _,
                d) => l.collect() + d.collect(),
        }
    }  
}

impl<T> Izraz<T> 
where 
    T : Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone
{
    fn eval(&self) -> T {
        match self {
            Izraz::Konstanta(v) => v.clone(),
            Izraz::Spremenjlivka(_) => todo!(),
            Izraz::Operacija(l,
                bin_operacija,
                d) => {
                    let lv = l.eval();
                    let dv = d.eval();
                    match bin_operacija {
                        BinOperacija::Plus => lv + dv,
                        BinOperacija::Minus => lv - dv,
                        BinOperacija::Times => lv * dv,
                    }
                }
        }
    }
}

impl<T: Display> Izraz<T> {
    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(v) => v.to_string(),
            Izraz::Spremenjlivka(s) => s.to_string(),
            Izraz::Operacija(l,
                bin_operacija,
                d) => {
                    let lv = l.izpis();
                    let dv = d.izpis();
                    let opi = match bin_operacija {
                        BinOperacija::Plus => "+", //format!("({} + {})", lv, dv), //lv + &String::from(" + ") + &dv,
                        BinOperacija::Minus => "-",
                        BinOperacija::Times => "*",
                    };
                    format!("({} {} {})", lv, opi, dv)
            }
        }
    }  
}

impl<T: Display> Display for Izraz<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Izraz: {}", self.izpis())
    }
}