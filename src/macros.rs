macro_rules! enum_map {
	(
        $NAME:ident: $TRAIT:ident; $FUNCTION:ident -> $TYPE:ty {
            $(
                $VARIANT:ident => $OUT:expr
            ),*
        }
    ) => (
        #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
        pub enum $NAME {
            $($VARIANT,)*
        }

        impl $TRAIT for $NAME {
        	fn $FUNCTION(&self) -> $TYPE {
        		match *self {
        		    $(
        		        $NAME::$VARIANT => $OUT,
        		    )*
        		}
        	}
        }
    );

	(
        $NAME:ident: $TRAIT:ident; $FUNCTION:ident {
            $(
                $VARIANT:ident => $OUT:expr
            ),*
        }
    ) => (
        enum_map! {
			$NAME: $TRAIT; $FUNCTION -> &str {
	            $(
	                $VARIANT => $OUT
	            ),*
	        }
		}
    );
}

macro_rules! schema_pattern_type {
	(
		$NAME:ident, $REGEX:expr
	) => (
		#[derive(Debug)]
		pub struct $NAME(String);

		impl SchemaStringType for $NAME {
			fn value(&self) -> &str {
				&self.0
			}
		}

		// TODO: Put this in some other trait
		impl SchemaRegexInput for $NAME {
			fn from_str(val: &str) -> Result<$NAME, AQIError> {
				lazy_static! {
					static ref RE: Regex = Regex::new($REGEX).unwrap();
				}
				match RE.is_match(val) {
					true => Ok($NAME(val.to_string())),
					_ => Err(AQIError::RegexError)
				}
			}
		}
	)
}

macro_rules! schema_string_tuple_struct {
	(
		$NAME:ident
	) => (
		pub struct $NAME(pub String);

		impl<'a> SchemaStringType for $NAME {
			fn value(&self) -> &str {
				&self.0
			}
		}
	)
}
