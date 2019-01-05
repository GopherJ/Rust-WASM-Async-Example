#![feature(async_await, await_macro, futures_api, try_blocks, type_ascription)]

#[macro_use]
extern crate cfg_if;

use futures01::future::Future;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{
    compat::backward::Compat,
    sleep::sleep,
};

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

#[wasm_bindgen(module = "../helper")]
extern "C" {
    pub fn foo();
}

pub async fn run() -> Result<(), JsValue> {
    try {
        web_sys::console::log_1(&"In Rust".into());

        set_panic_hook();

        let window = web_sys::window().expect("should have a Window");
        let document = window.document().expect("should have a Document");

        await!(sleep(1000))?;

        foo();

        let p: web_sys::Node = document.create_element("p")?.into();
        p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

        let body = document.body().expect("should have a body");
        let body: &web_sys::Node = body.as_ref();
        body.append_child(&p)?;

        ()
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(js_name = run)]
pub fn run_js() -> Promise {
    let future = Compat::new(run());
    let future = future.map(|_| JsValue::UNDEFINED);
    future_to_promise(future)
}
