use check_initial_state_derive::CheckInitialState;

/// Checks case when struct contains `Option` fields only.
#[test]
fn struct_with_optional_fields_only() {
	Struct { option: None, option2: None }.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {
	option: Option<String>,
	option2: Option<i32>,
}
