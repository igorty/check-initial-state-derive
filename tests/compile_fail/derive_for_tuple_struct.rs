use check_initial_state_derive::CheckInitialState;

fn main() {}

#[derive(CheckInitialState)]
struct Tuple (i32, Option<i32>);
