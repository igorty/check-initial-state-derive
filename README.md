In order to initialize complex structures Rust uses so&#x2011;called
[builder pattern](https://en.wikipedia.org/wiki/Builder_pattern). This
procedural macro provides possibility to check that a given builder struct
has all fields set to `None` on creation. Utilizing this macro:
* Eliminates human factor when, say for debugging purposes, some `Option`
fields of a builder pattern were explicitly initialized with `Some`, and are
to be committed in such erroneous state.
* Allows not to take care about newly added fields' initial state check to
be explicitly covered in the corresponding unit test.

There may be a need to ignore some fields of a builder struct, so they can
have any value initially. Such fields should be annotated with
`ignore_field` attribute. The same requirement applies to fields which are
not `Option` at all, if such are present.
# Example
```rust
#[derive(CheckInitialState)]
struct Builder {
    // Only this, non-annotated with `ignore_field`, field is going to be
    // checked for its value to be equal to None
    option: Option<i32>,
    // Non-Option fields should be explicitly annotated
    #[ignore_field]
    integer: i32,
    // If an Option field is expected to have Some value initially, it must
    // be explicitly annotated as well. Otherwise call to
    // `check_initial_state()` panics
    #[ignore_field]
    option2: Option<i32>,
}

impl Builder {
    fn new() -> Self {
        Self { option: None, integer: 10, option2: Some(10) }
    }
}

fn main() {
    // `check_initial_state()` method is created by `CheckInitialState`
    // procedural macro
    Builder::new().check_initial_state()
}
```
