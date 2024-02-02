pub fn js_args_to_utf8_vec(ctx: &JSContext, args: &[JSValue]) -> Vec<u8> {
    args.iter()
        .map(|arg| arg.to_string(ctx).unwrap().into_bytes())
        .flatten()
        .collect()
}