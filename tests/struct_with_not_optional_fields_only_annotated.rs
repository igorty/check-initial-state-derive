use check_initial_state_derive::CheckInitialState;

/// Checks case when struct contains fields with some other types in addition to
/// the `Option` ones. Fields with other types are annotated with
/// `ignore_field`.
#[test]
fn struct_with_not_optional_fields_only_annotated() {
	Struct { option: None, _other: "other".to_owned(), option2: None, _other2: 10 }
		.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {
	option: Option<String>,
	#[ignore_field]
	_other: String,
	option2: Option<u8>,
	#[ignore_field]
	_other2: usize,
}
