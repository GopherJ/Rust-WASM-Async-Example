use wasm_bindgen::prelude::*;

pub async fn sleep(millis: i32) -> Result<(), JsValue> {
    use crate::compat::promise_to_future;

    let promise = js_sys::Promise::new(&mut move |resolve, _| {
        let window = web_sys::window().expect("should have a Window");
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve, millis
        );
    });

    await!(promise_to_future(promise))?;
    Ok(())
}

#[wasm_bindgen(js_name = sleep)]
pub fn sleep_js(millis: i32) -> js_sys::Promise {
    use crate::compat::future_to_promise;
    use futures03::future::FutureExt;

    future_to_promise(async move {
        await!(sleep(millis))?;
        Ok(JsValue::UNDEFINED)
    }.boxed())
}
