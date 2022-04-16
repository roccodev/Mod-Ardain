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
