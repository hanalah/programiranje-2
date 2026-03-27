mod aritmeticno;

use aritmeticno::*;

trait Zaporedje<T> {
    fn name(&self) -> &str;
    fn start(&self) -> T;
    fn k_th(&self, k: u64) -> T;
    fn contains(&self, value: &T) -> bool;
}