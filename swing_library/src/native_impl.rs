use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setFrameName")]
    pub fn set_frame_name(s: &str);
}

use rjvm_core::{Context, Error, NativeMethod, Value};

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/awt/Frame.initName.(Ljava/lang/String;)V", init_name),
    ];

    context.register_native_mappings(mappings);
}

fn init_name(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let name_object = args[0].object().unwrap();

    let name_array = name_object.get_field(0).object().unwrap();
    let name_bytes = name_array.get_array_data();

    let mut name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let character = value.get().int() as u16;
        name_data.push(character);
    }

    let frame_name = String::from_utf16_lossy(&name_data);
    set_frame_name(&frame_name);

    Ok(None)
}
