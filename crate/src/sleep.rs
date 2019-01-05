use std::future::Future;
use futures01::Future as Future01;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

use crate::compat::{
    backward::Compat,
    forward::IntoAwaitable,
};

#[wasm_bindgen(module = "../helper")]
extern "C" {
    fn _sleep(millis: u32) -> Promise;
}

pub fn sleep(millis: u32) -> impl Future<Output=Result<(), JsValue>> {
    JsFuture::from(_sleep(millis))
        .map(|_| ())
        .map_err(|_| unreachable!())
        .into_awaitable()
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: u32) -> Promise {
    let future = Compat::new(sleep(millis));
    let future = future.map(|_| JsValue::UNDEFINED);
    future_to_promise(future)
}
