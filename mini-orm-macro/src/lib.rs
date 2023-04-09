#[macro_export]
macro_rules! iden {
    ($($x: ident => $y: literal,)+) => {
        #[derive(Debug, Eq, PartialEq)]
        pub enum Iden {
            $($x,)+
        }

        impl sea_query::Iden for Iden {
            fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                write!(
                    s,
                    "{}",
                    match self {
                        $(Self::$x => $y,)+
                    }
                )
                .unwrap();
            }
        }
    };
}
