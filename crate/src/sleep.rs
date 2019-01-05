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
    use crate::js_compat::future_to_promise;

    future_to_promise(async move {
        await!(sleep(millis))?;
        Ok(JsValue::UNDEFINED)
    })
}
