use check_initial_state_derive::CheckInitialState;

/// Checks case when struct does not contain fields at all.
#[test]
fn struct_with_no_fields() {
	Struct{}.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {}
