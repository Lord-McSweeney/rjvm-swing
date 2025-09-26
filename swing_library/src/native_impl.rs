use rjvm_core::{Context, Error, JvmString, MethodDescriptor, NativeMethod, Value};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// We MUST ensure that the closures are not dropped!
thread_local! {
    static ALL_CLOSURES: Mutex<Vec<Closure<dyn FnMut()>>> = Mutex::new(Vec::new());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setInterval")]
    fn js__set_interval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;

    #[wasm_bindgen(js_name = "startPaint")]
    fn js__start_paint();

    #[wasm_bindgen(js_name = "flushPaint")]
    fn js__flush_paint();

    #[wasm_bindgen(js_name = "setFrameName")]
    fn js__set_frame_name(s: &str);

    #[wasm_bindgen(js_name = "drawLine")]
    fn js__draw_line(x1: i32, y1: i32, x2: i32, y2: i32);

    #[wasm_bindgen(js_name = "fillRect")]
    fn js__fill_rect(x: i32, y: i32, width: i32, height: i32);

    #[wasm_bindgen(js_name = "setColor")]
    fn js__set_color(r: u8, g: u8, b: u8, a: u8);

    #[wasm_bindgen(js_name = "translate")]
    fn js__translate(x: i32, y: i32);

    #[wasm_bindgen(js_name = "drawString")]
    fn js__draw_string(string: &str, x: i32, y: i32);

    #[wasm_bindgen(js_name = "setFont")]
    fn js__set_font(string: &str, size: i32, modifiers: &str);

    // FIXME don't duplicate this across `web` and `swing_library`
    #[wasm_bindgen(js_name = "appendText")]
    fn js__output_to_err(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn js__debug(info: &str);
}

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/awt/Frame.initName.(Ljava/lang/String;)V", init_name),
        ("java/awt/Component.startPaint.()V", start_paint),
        ("java/awt/Component.flushPaint.()V", flush_paint),
        ("javax/swing/Timer.internalStartTimer.(ILjava/awt/event/ActionListener;)V", internal_start_timer),
        ("java/awt/CRC2DGraphics.drawLine.(IIII)V", draw_line),
        ("java/awt/CRC2DGraphics.fillRect.(IIII)V", fill_rect),
        ("java/awt/CRC2DGraphics.setColor.(Ljava/awt/Color;)V", set_color),
        ("java/awt/CRC2DGraphics.translate.(II)V", translate),
        ("java/awt/CRC2DGraphics.drawString.(Ljava/lang/String;II)V", draw_string),
        ("java/awt/CRC2DGraphics.internalSetFont.(Ljava/lang/String;II)V", internal_set_font),
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
    js__set_frame_name(&frame_name);

    Ok(None)
}

fn start_paint(_context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    js__start_paint();

    Ok(None)
}

fn flush_paint(_context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    js__flush_paint();

    Ok(None)
}

fn internal_start_timer(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let delay = args[0].int() as u32;
    let listener = args[1].object().unwrap();

    // SAFETY: `listener` is stored in the Timer object so it's safe to use it
    // in the closure (it won't get collected)

    // Lookup actionPerformed method
    let func_name = JvmString::new(context.gc_ctx, "actionPerformed".to_string());
    let func_descriptor_name = JvmString::new(
        context.gc_ctx,
        "(Ljava/awt/event/ActionEvent;)V".to_string(),
    );

    let func_descriptor = MethodDescriptor::from_string(context.gc_ctx, func_descriptor_name)
        .expect("Valid descriptor");

    let listener_class = listener.class();
    let listener_vtable = listener_class.instance_method_vtable();

    let method_idx = listener_vtable.lookup((func_name, func_descriptor));
    let action_performed_method = method_idx
        .map(|m| listener_vtable.get_element(m))
        .expect("ActionListener objects should have actionPerformed function");

    let closure = Closure::new(move || {
        if let Err(error) = action_performed_method.exec(
            context,
            &[Value::Object(Some(listener)), Value::Object(None)],
        ) {
            js__output_to_err(&format!(
                "Error while running timer callback: {:?}\n",
                error
            ));
        }
    });

    js__set_interval(&closure, delay);

    ALL_CLOSURES.with(|m| m.lock().unwrap().push(closure));

    Ok(None)
}

fn draw_line(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let x1 = args[1].int();
    let y1 = args[2].int();
    let x2 = args[3].int();
    let y2 = args[4].int();

    js__draw_line(x1, y1, x2, y2);

    Ok(None)
}

fn fill_rect(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let x = args[1].int();
    let y = args[2].int();
    let width = args[3].int();
    let height = args[4].int();

    js__fill_rect(x, y, width, height);

    Ok(None)
}

fn set_color(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let color = args[1].object();

    // The docs don't say what happens when calling `setColor(null)`, but
    // `SunGraphics2D` just ignores the call
    if let Some(color) = color {
        let r = color.get_field(0).int() as u8;
        let g = color.get_field(1).int() as u8;
        let b = color.get_field(2).int() as u8;
        let a = color.get_field(3).int() as u8;

        js__set_color(r, g, b, a);
    }

    Ok(None)
}

fn translate(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let x = args[1].int();
    let y = args[2].int();

    js__translate(x, y);

    Ok(None)
}

fn draw_string(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let string_obj = args[1].object().unwrap();

    let chars = string_obj.get_field(0).object().unwrap();
    let chars = chars.get_array_data();
    let chars = chars
        .iter()
        .map(|c| c.get().int() as u16)
        .collect::<Vec<_>>();

    let string = String::from_utf16_lossy(&chars);

    let x = args[2].int();
    let y = args[3].int();

    js__draw_string(&string, x, y);

    Ok(None)
}

fn internal_set_font(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let string_obj = args[1].object().unwrap();

    let chars = string_obj.get_field(0).object().unwrap();
    let chars = chars.get_array_data();
    let chars = chars
        .iter()
        .map(|c| c.get().int() as u16)
        .collect::<Vec<_>>();

    let font_name = String::from_utf16_lossy(&chars);

    let size = args[2].int();
    let style = args[3].int();

    let modifiers = match style {
        1 => "bold ",
        2 => "italic ",
        3 => "italic bold ",
        _ => "",
    };

    js__set_font(&font_name, size, modifiers);

    Ok(None)
}
