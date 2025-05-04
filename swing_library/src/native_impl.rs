use rjvm_core::{Context, Error, NativeMethod, Object, Value};

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
    ];

    context.register_native_mappings(mappings);
}
