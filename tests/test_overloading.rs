#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate overloading;

#[overloading::overloading]
fn overloaded(abc: String) -> i32 {
    abc.parse().unwrap()
}

#[overloading::overloading(overloaded)]
fn overloaded() -> i32 {
    114514
}

#[test]
fn test() {
    let res = overloaded("123".to_owned());
    assert_eq!(res, 123);

    let res = overloaded();
    assert_eq!(res, 114514);
}
