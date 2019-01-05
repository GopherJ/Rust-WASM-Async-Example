use std::future::Future;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    JsFuture,
    future_to_promise as _future_to_promise,
};

use crate::compat::{
    backward::Compat,
    forward::IntoAwaitable,
};

pub fn future_to_promise<F>(future: F) -> Promise
        where F: Future<Output=Result<JsValue, JsValue>> + 'static {
    let future = Compat::new(future);
    _future_to_promise(future)
}

pub fn promise_to_future(promise: Promise) -> impl Future<Output=Result<JsValue, JsValue>> {
    JsFuture::from(promise).into_awaitable()
}
