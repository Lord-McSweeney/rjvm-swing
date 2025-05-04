use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setFrameName")]
    fn set_frame_name(s: &str);

    #[wasm_bindgen(js_name = "setInterval")]
    fn set_interval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
}

use rjvm_core::{Context, Error, JvmString, MethodDescriptor, NativeMethod, Value};

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/awt/Frame.initName.(Ljava/lang/String;)V", init_name),
        ("javax/swing/Timer.internalStartTimer.(ILjava/awt/event/ActionListener;)V", internal_start_timer),
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

fn internal_start_timer(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let delay = args[0].int() as u32;
    let listener = args[1].object().unwrap();

    // SAFETY: `listener` is stored in the Timer object so it's safe to use it
    // in the closure (it won't get collected)

    let closure = Closure::new(move || {
        // Call actionPerformed method
        let func_name = JvmString::new(context.gc_ctx, "actionPerformed".to_string());
        let func_descriptor_name = JvmString::new(
            context.gc_ctx,
            "(Ljava/awt/event/ActionEvent;)V".to_string(),
        );

        let func_descriptor = MethodDescriptor::from_string(context.gc_ctx, func_descriptor_name)
            .expect("Valid descriptor");

        let listener_class = listener.class();

        let method_idx = listener_class
            .instance_method_vtable()
            .lookup((func_name, func_descriptor));

        if let Some(method_idx) = method_idx {
            let method = listener_class.instance_methods()[method_idx];
            let _ = method.exec(
                context,
                &[Value::Object(Some(listener)), Value::Object(None)],
            );
        } else {
            panic!("ActionListener objects should have actionPerformed function");
        }
    });

    set_interval(&closure, delay);

    // :p
    std::mem::forget(closure);

    Ok(None)
}
