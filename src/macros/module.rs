#[allow(unused_macros)]
#[macro_export]
macro_rules! modules {
    ($($m:ident),+) => {
        enum Modules {
        $($m,)*
        }
    }
}
