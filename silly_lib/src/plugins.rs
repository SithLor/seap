macro_rules! link_to_c_function {
    ($binary_name:ident, $function_name:ident, $arg1:ty, $arg2:ty, $return:ty) => {
        #[link(name = stringify!($binary_name))]
        extern "C" {
            fn $function_name(arg1: $arg1, arg2: $arg2) -> $return;
        }
    };
}
