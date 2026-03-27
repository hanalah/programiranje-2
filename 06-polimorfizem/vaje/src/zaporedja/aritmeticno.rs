use super::Zaporedje;

struct AritmeticnoZaporedje<T> {
    ime: String,
    a0: T,
    d: T,
    index: u64,
}

impl<T> AritmeticnoZaporedje<T> {
    fn new(ime: &str, a0: T, d: T) -> Self {
        AritmeticnoZaporedje {ime: ime.to_string(), a0, d, index: 0 }
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

impl<T> Zaporedje<T> for AritmeticnoZaporedje<T> {
    fn name(&self) -> &str {
        todo!()
    }

    fn start(&self) -> T {
        todo!()
    }

    fn k_th(&self, k: u64) -> T {
        todo!()
    }

    fn contains(&self, value: &T) -> bool {
        todo!()
    }
}