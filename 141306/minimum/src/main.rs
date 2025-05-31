#![allow(dead_code)]

use ring::*;
use std::hint::black_box;
mod ring {
    #[derive(Clone, Copy, Debug)]
    pub struct Bar(pub &'static Foo);

    pub static BAR0: Bar = Bar(&FOO0);
    pub static BAR1: Bar = Bar(&FOO1);
    pub static BAR2: Bar = Bar(&FOO2);
    pub static BAR3: Bar = Bar(&FOO3);

    #[derive(Debug)]
    pub struct Foo {
        pub f: fn(),
        pub id: i32,
    }

    fn f() {}

    pub static FOO0: Foo = Foo { f, id: 0 };
    pub static FOO1: Foo = Foo { f, id: 1 };
    pub static FOO2: Foo = Foo { f, id: 2 };
    pub static FOO3: Foo = Foo { f, id: 3 };
}

#[no_mangle]
#[inline(never)]
fn bad(foo: &Foo) -> Bar {
    if foo.id == FOO0.id {
        BAR0
    } else if foo.id == FOO1.id {
        BAR1
    } else if foo.id == FOO2.id {
        BAR2
    } else if foo.id == FOO3.id {
        BAR3
    } else {
        unreachable!()
    }
}

fn main() {
    dbg!(bad(black_box(&FOO0)));
    dbg!(bad(black_box(&FOO1)));
    dbg!(bad(black_box(&FOO2)));
    dbg!(bad(black_box(&FOO3)));
}

