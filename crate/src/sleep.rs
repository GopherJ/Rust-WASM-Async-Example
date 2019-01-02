use futures::Future;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

#[wasm_bindgen(module = "../helper")]
extern "C" {
    fn _sleep(millis: u32) -> Promise;
}

pub fn sleep(millis: u32) -> impl Future<Item=(), Error=JsValue> {
    JsFuture::from(_sleep(millis))
        .map(|_| ())
        .map_err(|_| unreachable!())
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: u32) -> Promise {
    let future = sleep(millis).map(|_| JsValue::UNDEFINED);
    future_to_promise(future)
}
