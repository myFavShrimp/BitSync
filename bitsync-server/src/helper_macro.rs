macro_rules! route {
    ($name:ident => $route:literal, ($($arg_prefix:tt $arg:ident : $arg_type:ty),*)) => {
        pub struct $name;
        impl $name {
            pub fn handler_route() -> String {
                format!($route, $( $crate::helper_macro::route_parameter_format!($arg_prefix $arg) ),*)
            }

            pub fn route_path($($arg: $arg_type),*) -> String {
                format!($route, $($arg),*)
            }
        }
    };
    ($name:ident => $route:literal) => {
        pub struct $name;
        impl $name {
            pub fn handler_route() -> String {
                String::from($route)
            }

            pub fn route_path() -> String {
                String::from($route)
            }
        }
    };
}

macro_rules! route_parameter_format {
    (* $arg:ident) => {
        format_args!("*{}", stringify!($arg))
    };
    (: $arg:ident) => {
        format_args!(":{}", stringify!($arg))
    };
}

pub(crate) use {route, route_parameter_format};

#[cfg(test)]
mod test {
    route!(SomeRoute => "/path/{}/with/{}", (: something: String, *parameter: u8));

    #[test]
    fn handler_route() {
        assert_eq!(
            SomeRoute::handler_route(),
            "/path/:something/with/*parameter"
        )
    }

    #[test]
    fn route_path() {
        assert_eq!(
            SomeRoute::route_path(String::from("test"), 13),
            "/path/test/with/13"
        )
    }
}
