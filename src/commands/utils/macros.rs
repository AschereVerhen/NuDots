#[macro_export]
macro_rules! debugf {
    ($func:ident, $fmt:expr $(, $args:expr)* $(,)?) => {{
        #[cfg(debug_assertions)]
        {
            eprintln!(
                "[DEBUG]|{}|({}): {}",
                module_path!(),
                stringify!($func),
                format!($fmt $(, $args)*)
            );
        }

        #[cfg(not(debug_assertions))]
        {
            let debug_env_on: bool = ::std::env::var("DEBUG").is_ok_and(|x| {
                x.parse::<u8>().map(|v| v != 0).unwrap_or(false)
            });

            if debug_env_on {
                eprintln!(
                    "[DEBUG]|{}|({}): {}",
                    module_path!(),
                    stringify!($func),
                    format!($fmt $(, $args)*)
                );
            }
        }
    }};
}
