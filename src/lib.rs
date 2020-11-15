//! In order to initialize complex structures Rust uses so&#x2011;called
//! [builder pattern](https://en.wikipedia.org/wiki/Builder_pattern). This
//! procedural macro provides possibility to check that a given builder struct
//! has all fields set to `None` on creation. Utilizing this macro:
//! * Eliminates human factor when, say for debugging purposes, some `Option`
//! fields of a builder pattern were explicitly initialized with `Some`, and are
//! to be committed in such erroneous state.
//! * Allows not to take care about newly added fields' initial state check to
//! be explicitly covered in the corresponding unit test.
//!
//! There may be a need to ignore some fields of a builder struct, so they can
//! have any value initially. Such fields should be annotated with
//! `ignore_field` attribute. The same requirement applies to fields which are
//! not `Option` at all, if such are present.
//! # Example
//! ```
//! # use check_initial_state_derive::CheckInitialState;
//! #
//! #[derive(CheckInitialState)]
//! struct Builder {
//!     // Only this, non-annotated with `ignore_field`, field is going to be
//!     // checked for its value to be equal to None
//!     option: Option<i32>,
//!     // Non-Option fields should be explicitly annotated
//!     #[ignore_field]
//!     integer: i32,
//!     // If an Option field is expected to have Some value initially, it must
//!     // be explicitly annotated as well. Otherwise call to
//!     // `check_initial_state()` panics
//!     #[ignore_field]
//!     option2: Option<i32>,
//! }
//!
//! impl Builder {
//!     fn new() -> Self {
//!         Self { option: None, integer: 10, option2: Some(10) }
//!     }
//! }
//!
//! fn main() {
//!     // `check_initial_state()` method is created by `CheckInitialState`
//!     // procedural macro
//!     Builder::new().check_initial_state()
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Error, Field, Fields};

/// Fields which are annotated with this attribute are ignored by
/// `CheckInitialState` proc macro check.
const IGNORE_FIELD_ATTRIBUTE: &'static str = "ignore_field";

/// Creates `check_initial_state()` method which is intended to check all
/// `Option` fields to have `None` at the time of the mentioned method call. If
/// any of optional fields are `Some` instead, the mentioned method panics
/// providing the name of such non&#x2011;empty field.
///
/// *Notes.*
/// * Proc macros cannot definitely determine fields types. Fields which are not
/// `Option` must be explicitly annotated with `ignore_field` attribute. The
/// same approach may be used to exclude some `Option` fields from the check.
/// * This procedural macro expects to call `std::option::Option::is_some()` on
/// each non&#x2011;annotated field of the provided struct. Since
/// `proc_macro_derive` receives nothing but a struct in the form of a token
/// tree, a user of this macro is going to observe error, similar to
/// ```text
/// error[E0308]: mismatched types
///   --> $DIR/struct_with_other_fields_not_annotated.rs:10:10
///    |
/// 10 | #[derive(CheckInitialState)]
///    |          ^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found `i32`
///    |
///    = note: expected reference `&std::option::Option<_>`
///               found reference `&i32`
///    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
/// ```
/// , if a given non&#x2011;annotated with `ignore_field` field is not an
/// `Option`.
#[proc_macro_derive(CheckInitialState, attributes(ignore_field))]
pub fn check_initial_state_derive(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let idents = {
		let data = match extract_struct(&ast) {
			Ok(data) => data,
			Err(err) => return err,
		};
		let fields = match fetch_fields(&data, &ast) {
			Ok(fields) => fields,
			Err(err) => return err,
		};
		fields.iter()
			// Filter out fields with `ignore_field` attribute
			.filter(|field| {
				return field.attrs.iter()
					.filter(|attribute| attribute.path.segments.first().is_some())
					.find(|attribute| {
						return attribute.path.segments.first().unwrap()
							.ident.to_string() == IGNORE_FIELD_ATTRIBUTE;
					})
					.is_none();
			})
			.map(|field| {
				return match field.ident.as_ref() {
					Some(field) => field,
					None => {
						panic!(
							"Unexpected implementation error occurred. Reason: Field `{:?}` is \
								expected to have name while it does not",
							field,
						);
					},
				};
			})
	};
	let struct_name = &ast.ident;
	let (leading_generics, trailing_generics, where_clause) = &ast.generics.split_for_impl();
	let impl_header = if where_clause.is_some() {
		let where_clause = where_clause.unwrap();
		quote! { impl #leading_generics #struct_name #trailing_generics #where_clause }
	} else {
		quote! { impl #leading_generics #struct_name #trailing_generics }
	};
	let field_checks = idents.map(|ident| {
		let error_message = format!("Field `{}` has Some value instead of None", ident);
		return quote! {
			if ::std::option::Option::is_some(&self.#ident) {
				panic!(#error_message);
			};
		};
	});
	let result = quote! {
		#impl_header {
			/// Checks all `Option` fields to have `None` at the time of this
			/// method call. Is expected to be used for testing purposes.
			/// # Panics
			/// Any of `self` fields, which are not annotated with
			/// `ignore_field`, are `Some`. Panic message will contain the name
			/// of an `Option` field which has some value.
			fn check_initial_state(&self) {
				#(#field_checks)*
			}
		}
	};
	return result.into();
}

/// Fetches struct from `input`. Parsing error is returned in case of data
/// structure for this procedural macro is other than a struct with named
/// fields.
fn extract_struct(input: &DeriveInput) -> Result<&DataStruct, TokenStream> {
	return match &input.data {
		Data::Struct(data_struct) => Ok(data_struct),
		_ => {
			let message = Error::new_spanned(
				&input,
				"`CheckInitialState` procedural macro is allowed for structs with named fields \
					only",
			)
				.to_compile_error()
				.into();
			Err(message)
		},
	};
}

/// Retrieves fields contained in `data_struct`. Parsing error is returned, if
/// `data_struct` is not of expected type.
/// # Parameters
/// * `data_struct` &ndash; Expected to be a struct with named fields.
/// * `input` &ndash; Is entire abstract syntax tree provided for this
/// procedural macro. May be used to form a syntax error when `data_struct` is
/// other than expected.
fn fetch_fields<'a>(data_struct: &'a DataStruct, input: &DeriveInput)
	-> Result<&'a Punctuated<Field, Comma>, TokenStream> {
	return match &data_struct.fields {
		Fields::Named(named_fields) => Ok(&named_fields.named),
		Fields::Unit => {
			let message = Error::new_spanned(
				input,
				"`CheckInitialState` procedural macro is no allowed for unit structs",
			)
				.to_compile_error()
				.into();
			Err(message)
		},
		fields => {
			let message = Error::new_spanned(
				fields,
				"`CheckInitialState` procedural macro is allowed for structs with named fields \
					only",
			)
				.to_compile_error()
				.into();
			Err(message)
		},
	};
}
