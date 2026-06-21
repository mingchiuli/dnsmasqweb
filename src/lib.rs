#![recursion_limit = "256"]

pub mod api_types;
pub mod config;
pub mod error;

#[cfg(feature = "ssr")]
pub mod dnsmasq;
#[cfg(feature = "ssr")]
pub mod server;
#[cfg(feature = "ssr")]
pub mod storage;

#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod app;
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod i18n;
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod server_fns;
#[cfg(any(feature = "hydrate", feature = "ssr"))]
pub mod ui;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
