#[macro_export]
macro_rules! err {
    ($($args:tt)*) => ({
        let description = format!($($args)*);
        elaiki_api::utils::errors::Error::new(description);
    })
}
