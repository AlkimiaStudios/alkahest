/// Profile the given block of code.
///
/// This macro is only active when the `profiling` feature is enabled. When the
/// `profiling` feature is enabled, this macro will print the time it took to
/// execute the given block of code, along with the name of the profile.
#[macro_export]
macro_rules! profile {
    ($name:ident, $content:block) => {
        #[cfg(feature = "profiling")]
        {
            use std::time::Instant;
            use crate::info;

            let start = Instant::now();
            $content
            let end = Instant::now();
            info!("Profile {}: {}us", stringify!($name), (end - start).as_micros());
        }
        #[cfg(not(feature = "profiling"))]
        {
            $content
        }
    };
}
