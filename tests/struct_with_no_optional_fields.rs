use check_initial_state_derive::CheckInitialState;

/// Checks case when struct does not contain `Option` fields at all. Other
/// fields are annotated with `ignore_field`.
#[test]
fn struct_with_no_optional_fields() {
	Struct { _string: "string".to_owned(), _integer: 10 }.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {
	#[ignore_field]
	_string: String,
	#[ignore_field]
	_integer: i32,
}
