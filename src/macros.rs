macro_rules! strong_string {
    ( $t:ident ) => {
        #[derive(Clone, Deserialize)]
        pub struct $t(pub String);

        impl fmt::Display for $t {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(&self.0)
            }
        }

        impl Strong<String> for $t {
            fn get(&self) -> &String {
                &self.0
            }
        }
    };
}

pub(crate) use strong_string;
