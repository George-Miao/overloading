# Overloading

[<img alt="github" src="https://img.shields.io/badge/github-overloading-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/George-Miao/overloading)
[<img alt="crates.io" src="https://img.shields.io/crates/v/overloading.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/overloading)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-overloading-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/overloading)

A POC crate that utilizes `Fn*` traits to implement partial overloading. Caveat: only parameters can be overloaded, not return types.

## TLDR

```rust
#![feature(unboxed_closures)]
#![feature(fn_traits)]

use overloading::overloading;

#[overloading]
fn overloaded(abc: String) -> i32 {
    abc.parse().unwrap()
}

#[overloading(overloaded)]
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
```

Expanded code:

```rust
#[allow(non_camel_case_types)]
struct overloaded;
impl std::ops::FnOnce<(String,)> for overloaded {
    type Output = i32;
    extern "rust-call" fn call_once(self, (abc,): (String,)) -> Self::Output {
        abc.parse().unwrap()
    }
}
impl std::ops::FnMut<(String,)> for overloaded {
    extern "rust-call" fn call_mut(&mut self, (abc,): (String,)) -> Self::Output {
        abc.parse().unwrap()
    }
}
impl std::ops::Fn<(String,)> for overloaded {
    extern "rust-call" fn call(&self, (abc,): (String,)) -> Self::Output {
        abc.parse().unwrap()
    }
}
impl std::ops::FnOnce<()> for overloaded {
    type Output = i32;
    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        114514
    }
}
impl std::ops::FnMut<()> for overloaded {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        114514
    }
}
impl std::ops::Fn<()> for overloaded {
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        114514
    }
}
```
