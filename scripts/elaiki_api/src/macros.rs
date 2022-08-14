#[macro_export]
macro_rules! err {
    ($($args:tt)*) => {{
        let description = format!($($args)*);
        let res = $crate::utils::errors::Error::new(description);
        res
    }}
}
