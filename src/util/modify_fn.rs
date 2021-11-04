use crate::Arg;

/// Adds default_value which is obtained via
/// Default and ToString traits
pub fn default_value_t<T>(arg: Arg) -> Arg
where
    T: Default + ToString
{
    let s = make_static_str(<T as Default>::default());
    arg.default_value(s).required(false)
}

fn make_static_str<T>(t: T) -> &'static str
where
    T: ToString
{
    Box::leak(t.to_string().into_boxed_str())
}
