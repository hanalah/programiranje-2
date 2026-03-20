//enunm Ocena {
//    A,
//    B,
//    C
//}
//
//struct Ucenec {
//    ime: String
//}
//
//type par = (u32, u32);
//
//struct Par(u32, u32);
//

struct AritmeticnoZaporedje {
    a0: i64,
    d: i64,
    index: u64
}

/*
- trenutni člen
- vsota prvih nekaj členov
- naslednji člen
- prejšni člen
*/

impl AritmeticnoZaporedje {
    fn new(a0: i64, d: i64) -> Self {
        AritmeticnoZaporedje { a0, d, index: 0 }
    }

    fn next(&mut self) -> i64 {
        self.index += 1;
        self.a0 + (self.index as i64 - 1) * self.d
    }

    fn n_th(&self, n: u64) -> i64 {
        self.a0 + (n as i64) * self.d
    }

    fn reset(&mut self) {
        self.index = 0
    }

    fn current(&self) -> i64 {
        self.a0 + (self.index as i64) * self.d
    }

    fn sum(&mut self, n: u64) -> i64 {
        let tmp_index = self.index;
        let mut vsota = 0; 
        for _ in 0..n {
            vsota += self.next()
        }
        self.index = tmp_index;
        vsota
    }

    fn vsota(&self, other: &Self) -> Self {
        Self::new(self.a0 + other.a0, self.d + other.d)
    }

}

//aritmeticno zapiredje ni zaprto za množenje

//še za geometrijsko zaporedje


enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

fn main() {
    let izraz1 = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)),
            BinOperacija::Times,
            Box::new(Izraz::Konstanta(3)),
        ))
    )
}
