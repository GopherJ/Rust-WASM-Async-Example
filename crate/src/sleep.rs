use std::future::Future;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../helper")]
extern "C" {
    fn _sleep(millis: u32) -> js_sys::Promise;
}

pub fn sleep(millis: u32) -> impl Future<Output=Result<(), JsValue>> {
    use futures01::future::Future;
    use wasm_bindgen_futures::JsFuture;
    use crate::compat::forward::IntoAwaitable;

    JsFuture::from(_sleep(millis))
        .map(|_| ())
        .map_err(|_| unreachable!())
        .into_awaitable()
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: u32) -> js_sys::Promise {
    use futures01::future::Future;
    use crate::compat::backward::Compat;
    use wasm_bindgen_futures::future_to_promise;

    let future = Compat::new(sleep(millis));
    let future = future.map(|_| JsValue::UNDEFINED);
    future_to_promise(future)
}
