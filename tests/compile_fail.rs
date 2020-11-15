/// Manages tests which are expected to fail while procedural macro expansion.
#[test]
fn compile_fail() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/compile_fail/struct_with_other_fields_not_annotated.rs");
	t.compile_fail("tests/compile_fail/derive_for_enum.rs");
	t.compile_fail("tests/compile_fail/derive_for_union.rs");
	t.compile_fail("tests/compile_fail/derive_for_tuple_struct.rs");
	t.compile_fail("tests/compile_fail/derive_for_unit_struct.rs");
}
