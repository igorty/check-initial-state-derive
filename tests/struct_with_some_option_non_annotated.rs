use check_initial_state_derive::CheckInitialState;

/// Checks case when struct contains `Option` fields, which are not annotated
/// with `ignore_field`, and have `Some` value.
#[test]
#[should_panic(expected = "Field `option` has Some value instead of None")]
fn struct_with_some_option_non_annotated() {
	Struct { _integer: 10, option: Some("string".to_owned()) }.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {
	#[ignore_field]
	_integer: i32,
	option: Option<String>,
}
