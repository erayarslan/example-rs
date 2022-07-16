#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::ffi::CStr;

mod bindings;

struct HelloWorldFuture {}

impl Future for HelloWorldFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let c_str: &CStr = unsafe { CStr::from_ptr(bindings::hello_world()) };
        let str_slice: &str = c_str.to_str().unwrap();
        let str_buf: String = str_slice.to_owned();

        Poll::Ready(str_buf)
    }
}

pub fn hello_world() -> impl Future<Output=String> {
    HelloWorldFuture {}
}
