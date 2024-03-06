macro_rules! strong_alias {
    ( $n:ident, $t:ident ) => {
        #[derive(Clone, Deserialize)]
        pub struct $n(pub $t);

        impl fmt::Display for $n {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(&self.0.to_string())
            }
        }

        impl Strong<$t> for $n {
            fn get(&self) -> &$t {
                &self.0
            }
        }
    };
}

pub(crate) use strong_alias;
