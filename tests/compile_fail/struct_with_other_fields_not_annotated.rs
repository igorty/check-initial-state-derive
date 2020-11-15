use check_initial_state_derive::CheckInitialState;

/// Checks case when struct contains non&#x2011;`Option` fields which are not
/// annotated with `ignore_field`.
fn main() {
	Struct { option: Some(10), integer: 10 }.check_initial_state();
}

#[derive(CheckInitialState)]
struct Struct {
	#[ignore_field]
	option: Option<i32>,
	integer: i32,
}
