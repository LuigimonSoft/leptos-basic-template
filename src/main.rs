#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_basic_template::App;
use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn wasm_start() {
  main();
}

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(|| view! { <App /> });
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
  println!("This example only works when compiled to WebAssembly.");
}
