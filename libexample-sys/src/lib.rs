#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::pin::Pin;
use futures_util::future::{self, Either};
use std::task::{Context, Poll};
use std::future::Future;
use libc::c_char;
use std::ffi::CStr;
use std::str;

mod bindings;

struct HelloWorldFuture {}

impl Future for HelloWorldFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let c_buf: *const c_char = unsafe { bindings::hello_world() };
        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
        let str_slice: &str = c_str.to_str().unwrap();
        let str_buf: String = str_slice.to_owned();

        Poll::Ready(str_buf)
    }
}

pub fn hello_world() -> impl Future<Output=String> {
    if true {
        Either::Left(HelloWorldFuture {})
    } else {
        Either::Right(future::ready(String::from("You Fucked")))
    }
}
