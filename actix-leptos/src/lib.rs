mod app;

cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use app::App;
        use leptos::{log, view};
        use log::Level;

        #[cfg(feature = "hydrate")]
        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            _ = console_log::init_with_level(Level::Debug);

            log!("Hydrating...");

            leptos::mount_to_body(|cx| view! { cx, <App/> });
        }
    }
}
