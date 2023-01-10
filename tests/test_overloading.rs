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

#[overloading::overloading(overloaded)]
fn overloaded<T: Fn() -> i32>(t: T) -> i32 {
    t()
}

#[test]
fn test() {
    let res = overloaded("123".to_owned());
    assert_eq!(res, 123);

    let res = overloaded();
    assert_eq!(res, 114514);

    let res = overloaded(|| 1919810);
    assert_eq!(res, 1919810);
}
