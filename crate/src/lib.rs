#![feature(async_await, await_macro, futures_api, type_ascription)]

#![deny(dead_code, unused_imports)]

#[macro_use]
extern crate cfg_if;

use wasm_bindgen::prelude::*;

use crate::sleep::sleep;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

mod compat;
mod sleep;

#[wasm_bindgen(module = "../../js/helper")]
extern "C" {
    pub fn greet();
}

pub async fn run() -> Result<(), JsValue> {
    web_sys::console::log_1(&"In Rust".into());

    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    await!(sleep(1000))?;

    greet();

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;

    Ok(())
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(js_name = run)]
pub fn run_js() -> js_sys::Promise {
    use crate::compat::future_to_promise;
    use futures03::future::FutureExt;

    future_to_promise(async move {
        await!(run())?;
        Ok(JsValue::UNDEFINED)
    }.boxed())
}
