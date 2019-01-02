#![feature(try_blocks, type_ascription)]

#[macro_use]
extern crate cfg_if;

use futures::future::{Future, ok, FutureResult};

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

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

#[wasm_bindgen(module = "../helper")]
extern "C" {
    pub fn foo();
}

pub fn run() -> impl Future<Item=(), Error=JsValue> {
    ok({
        web_sys::console::log_1(&"In Rust".into());

        set_panic_hook();
        ()
    })
    .and_then(|_| {
        use self::sleep::sleep;

        sleep(1000)
    })
    .and_then(|_| FutureResult::from(try {
        foo();

        let window = web_sys::window().expect("should have a Window");
        let document = window.document().expect("should have a Document");

        let p: web_sys::Node = document.create_element("p")?.into();
        p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

        let body = document.body().expect("should have a body");
        let body: &web_sys::Node = body.as_ref();
        body.append_child(&p)?;

        ()
    }: Result<(), JsValue>))
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(js_name = run)]
pub fn run_js() -> Promise {
    let future = run().map(|_| JsValue::UNDEFINED);
    future_to_promise(future)
}

mod sleep {
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
}
