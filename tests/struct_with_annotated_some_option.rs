use check_initial_state_derive::CheckInitialState;

/// Checks case when struct contains `Option` fields with `Some` value, but
/// annotated with `ignore_field`.
#[test]
fn struct_with_annotated_some_option() {
	Struct { option: None, _option2: Some(10) }.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {
	option: Option<String>,
	#[ignore_field]
	_option2: Option<i32>,
}
