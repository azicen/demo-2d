#[macro_export]
macro_rules! err {
    ($($args:tt)*) => ({
        let description = format!($($args)*);
        $crate::utils::errors::Error::new(description)
    })
}
