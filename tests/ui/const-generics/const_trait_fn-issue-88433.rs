//@ build-pass

#![feature(const_trait_impl, effects)] //~ WARN the feature `effects` is incomplete

#[const_trait]
trait Func<T> {
    type Output;

    fn call_once(self, arg: T) -> Self::Output;
}


struct Closure;

impl const Func<&usize> for Closure {
    type Output = usize;

    fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
}

enum Bug<T = [(); Closure.call_once(&0) ]> {
    V(T),
}

fn main() {}
