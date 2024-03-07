macro_rules! strong_alias {
    ( $n:ident, $t:ident ) => {
        strong_alias!( $n, $t, );
    };
    ( $n:ident, $t:ident, $($derivs:ident),* ) => {
        #[derive(Clone, Deserialize, $($derivs),*)]
        pub struct $n(pub $t);

        impl fmt::Display for $n {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(&self.0.to_string())
            }
        }

        impl $n {
            #[allow(dead_code)]
            pub fn unchecked(value: impl Into<$t>) -> $n {
                $n(value.into())
            }
        }

        impl Strong<$t> for $n {
            fn get(self) -> $t {
                self.0
            }

            fn get_ref(&self) -> &$t {
                &self.0
            }
        }
    };
}

pub(crate) use strong_alias;


#[cfg(test)]
mod tests {
    use core::fmt;
    use serde::Deserialize;

    use crate::strong::Strong;

    strong_alias!(MyString, String);

    #[test]
    fn test_my_string() {
        let a = MyString::unchecked("abc");

        assert_eq!(a.get_ref(), "abc");
        assert_eq!(a.get(), "abc");
    }

    #[test]
    fn test_fmt_my_string() {
        let a = MyString::unchecked("abc");

        assert_eq!(format!("{a}"), "abc");
    }

    strong_alias!(MyI64, i64);

    #[test]
    fn test_my_i64() {
        let a = MyI64::unchecked(42);

        assert_eq!(a.get(), 42);
    }
}
