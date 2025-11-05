mod app;

use app::*;
use leptos::prelude::*;
include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
