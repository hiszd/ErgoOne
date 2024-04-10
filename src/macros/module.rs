#[allow(unused_macros)]
#[macro_export]
macro_rules! ct_idents {
    () => {0usize};
    ($_head:ident $($tail:ident)*) => {1usize + ct_idents!($($tail)*)};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! modules {
    ($($m:ident),+) => {
        const MODULES: [&str;ct_idents!($($m)*)] = [$(stringify!($m),)*];

        enum Modules {
        $($m,)*
        }
    }
}
