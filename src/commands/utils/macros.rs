#[allow(dead_code)]
pub static DEBUG_ENABLED: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
   std::env::var("DEBUG")
       .is_ok_and(|x| {
           !matches!(x.to_ascii_lowercase().as_str(), "1" | "off" | "no")
       })
});

#[macro_export]
macro_rules! debugf {
    ($func:ident, $fmt:expr $(, $args:expr)* $(,)?) => {{
        #[cfg(debug_assertions)]
        const ENABLE_DEBUG: bool = true;

        #[cfg(not(debug_assertions))]
        const ENABLE_DEBUG: bool = crate::macros::DEBUG_ENABLED;

        if ENABLE_DEBUG {
            eprintln!(
                "[DEBUG]|{}|({}): {}",
                module_path!(),
                stringify!($func),
                format_args!($fmt $(, $args)*)
            );
        }
    }};
}
#[macro_export]
macro_rules! debug_started {
    ($func:ident) => {{
        #[cfg(debug_assertions)]
        const ENABLE_DEBUG: bool = true;

        #[cfg(not(debug_assertions))]
        const ENABLE_DEBUG: bool = crate::macros::DEBUG_ENABLED;

        if ENABLE_DEBUG {
            eprintln!(
                "[DEBUG]|{}|: {} Started.",
                module_path!(),
                stringify!($func),
            )
        }
    }}
}