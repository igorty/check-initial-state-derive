use check_initial_state_derive::CheckInitialState;
use std::fmt::{Debug, Display};

type AnOption<T> = Option<T>;

/// Ensures that `CheckInitialState` macro properly handles cases when struct is
/// supplied with generic parameters and lifetimes.
#[test]
fn check_generics() {
	Struct {
		option: None,
		option2: &None::<String>,
		_string: "value",
		_string2: "other",
		_vector: vec![Box::new("string")],
	}
		.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct<'a, 'b, T: Display + ?Sized, E>
	where E: Debug {
	option: AnOption<String>,
	option2: &'b Option<E>,
	#[ignore_field]
	_string: &'a str,
	#[ignore_field]
	_string2: &'static str,
	#[ignore_field]
	_vector: Vec<Box<T>>,
}
