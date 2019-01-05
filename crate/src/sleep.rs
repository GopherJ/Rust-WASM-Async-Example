use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../helper")]
extern "C" {
    fn _sleep(millis: u32) -> js_sys::Promise;
}

pub async fn sleep(millis: u32) -> Result<(), JsValue> {
    use crate::js_compat::promise_to_future;

    await!(promise_to_future(_sleep(millis)))?;
    Ok(())
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: u32) -> js_sys::Promise {
    use crate::js_compat::future_to_promise;

    future_to_promise(async move {
        await!(sleep(millis))?;
        Ok(JsValue::UNDEFINED)
    })
}
