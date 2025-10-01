use wasm_bindgen::prelude::*;

mod loader_backend;
mod native_impl;
mod runner;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn dbg(s: &str);

    #[wasm_bindgen(js_name = "appendText")]
    pub fn output(s: &str);

    #[wasm_bindgen(js_name = "appendText")]
    pub fn output_to_err(s: &str);
}

#[wasm_bindgen(js_name = "fileLoaded")]
pub fn file_loaded(name: &str, data: &[u8], args: Vec<String>) {
    let is_jar = name.ends_with(".jar");

    output("rjvm ");
    if is_jar {
        output("--jar ");
    }
    output(name);
    for arg in &args {
        output(" ");
        output(arg);
    }
    output("\n");

    runner::run_file(data, args, is_jar);
    // output("$ ");
}

#[wasm_bindgen(js_name = "setPanicHook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = "onMouseMove")]
pub fn on_mouse_move(x: i32, y: i32) {
    runner::on_mouse_move(x, y)
}

#[wasm_bindgen(js_name = "onMouseDown")]
pub fn on_mouse_down(x: i32, y: i32) {
    runner::on_mouse_down(x, y)
}

#[wasm_bindgen(js_name = "onMouseUp")]
pub fn on_mouse_up(x: i32, y: i32) {
    runner::on_mouse_up(x, y)
}

#[wasm_bindgen(js_name = "onKeyDown")]
pub fn on_key_down(code: i32) {
    runner::on_key_down(code)
}

#[wasm_bindgen(js_name = "onKeyUp")]
pub fn on_key_up(code: i32) {
    runner::on_key_up(code)
}
