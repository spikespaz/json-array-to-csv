#[macro_export]
macro_rules! bail_on_value_type {
    ($value:expr, expected = $exp:literal) => {
        return Err(::serde::de::Error::invalid_type(
            ::serde::de::Unexpected::Other(::std::any::type_name_of_val($value)),
            &$exp,
        ))
    };
}

pub(crate) use crate::bail_on_value_type;

pub(crate) fn missing_field_at_pointer(pointer: impl AsRef<str>) -> serde_json::Error {
    serde::de::Error::custom(leak_str(format!(
        "missing field at pointer `{}`",
        pointer.as_ref()
    )))
}

pub(crate) fn leak_str<'a>(string: impl Into<std::borrow::Cow<'a, str>>) -> &'static str {
    let moo: std::borrow::Cow<_> = string.into();
    let moo = Box::new(moo.into_owned());
    &*Box::leak(moo)
}
