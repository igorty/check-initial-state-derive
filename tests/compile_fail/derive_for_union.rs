use check_initial_state_derive::CheckInitialState;

fn main() {}

#[derive(CheckInitialState)]
union Union {
	integer: i32,
}
