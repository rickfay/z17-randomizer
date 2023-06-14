///
#[macro_export]
macro_rules! string_constants {
    (
        $(#[$attr:meta])*
        $type:ident {
            $($variant:ident,)+
        }
    ) => {
        $(#[$attr])*
        pub struct $type;

        $(#[$attr])*
        impl $type {
            $(pub const $variant: &'static str = stringify!($variant);)+
        }
    }
}
