use creusot_std::prelude::*;

#[logic(indirect)]
pub fn const_true<T>(_: T) -> bool {
    true
}

#[logic(indirect)]
#[ensures(const_true(x))]
pub fn id<T>(x: T) -> T {
    x
}

pub mod opaque {
    use super::*;

    #[logic(open(self))]
    pub fn f(_: Int) -> Int {
        0
    }

    #[logic]
    #[ensures(
        #[trigger(result)]
        f(2 * x + 1) == f(2 * x - 1)
    )]
    #[trusted]
    pub fn f_def(x: Int) {}
}

use opaque::*;

#[logic]
#[requires(true)]
pub fn test() {
    let t = |_: Int| {};

    #[trigger(t[i])]
    |i: Int| {
        f_def(2 * i);
        f_def(2 * i + 1);
    };

    id(t[0]);
    id(t[1]);
    id(t[2]);
    id(t[3]);
    id(t[4]);
    id(t[5]);
    proof_assert!(f(23) == f(-1));
}
