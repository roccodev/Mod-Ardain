macro_rules! ffi_fn {
    ($fn:expr, $($sig:tt) +) => {
        (::std::mem::transmute::<_, extern "C" fn$($sig) +>($fn))
    };
}

macro_rules! offset_fn {
    ($platform:expr, $offset:expr, $($sig:tt) +) => {
        (::std::mem::transmute::<_, extern "C" fn$($sig) +>($offset.as_fn($platform)))
    };
}

macro_rules! c_str {
    ($st:expr) => {
        concat!($st, '\0').as_ptr() as *const ::skyline::libc::c_char
    };
}

/// Converts a string slice to a borrowed [`std::ffi::CStr`].
///
/// This is a fast, no-allocation conversion, and its behaviour is defined
/// provided the following condition is met:
///     * The string does not contain null bytes. (Anywhere, it also can't
///     be null-terminated)
macro_rules! c_str_ref {
    ($st:expr) => {
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($st, '\0').as_bytes()) }
    };
}
