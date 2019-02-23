#![feature(generators, generator_trait, try_from, never_type)]

use eff::*;

#[derive(Debug)]
struct Foo(usize);
impl Effect for Foo {
    type Output = usize;
}

#[derive(Debug)]
struct Bar;
impl Effect for Bar {
    type Output = String;
}

mod effects {
    #[derive(Debug)]
    pub struct Baz;
    impl super::Effect for Baz {
        type Output = !;
    }
}

#[test]
fn test_example() {
    #[eff(Foo, Bar, effects::Baz)]
    fn effectful_computation() -> char {
        let i1 = perform!(Foo(1));
        let i2 = perform!(Foo(4));
        let s = perform!(Bar);
        if i1 + i2 >= s.len() {
            perform!(effects::Baz);
        }
        s.chars().nth(i1 + i2).unwrap()
    }

    let e = effectful_computation();
    let result = e
        .handle(|Foo(x)| {
            static move || {
                println!("foo");
                resume!(x + 1)
            }
        })
        .handle(|Bar| {
            static move || {
                println!("bar");
                resume!("Hello, World!".into())
            }
        })
        .handle(|effects::Baz| {
            static move || {
                if false {
                    yield unreachable!()
                }
                println!("baz");
                'x'
            }
        })
        .run(|x| x)
        .unwrap();

    assert_eq!(result, "Hello, World!".chars().nth(7).unwrap());
}
