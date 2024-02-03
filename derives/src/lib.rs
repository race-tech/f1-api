use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod filter_validation;

/// Derive the `FilterValidation` trait for a struct.
/// This trait is used to validate that the filter is valid (e.g non overlapping parameters are used)
/// this works by using the `#[validate]` attribute on the fields of the struct.
/// It will only work on structs with `Option<T>` fields.
///
/// It will generate an implementation of the `FilterValidation` trait for the struct.
///
/// # Attributes
/// - `#[validation(skip)]` - Skip this field when validating the filter
/// - `#[validation(unique)]` - Check that this field is unique in the filter
/// - `#[validation(compatible(...))]` - Check that this field is compatible with other fields in the filter
///
/// # Example
/// ```ignore
/// #[derive(FilterValidation)]
/// pub struct DriverFilter {
///     #[validation(skip)]
///     pub limit: Option<Limit>,
///     #[validation(skip)]
///     pub page: Option<Page>,
///     #[validation(unique)]
///     pub driver_number: Option<DriverNumber>,
///     #[validation(unique)]
///     pub driver_ref: Option<DriverRef>,
///     #[validation(compatible(circuit, grid, result, year, round))]
///     pub constructor: Option<ConstructorName>,
///     pub circuit: Option<Circuit>,
///     pub grid: Option<Grid>,
///     pub result: Option<RaceResult>,
///     pub year: Option<Year>,
///     pub round: Option<Round>,
/// }
/// ```

#[proc_macro_derive(FilterValidation, attributes(validation))]
pub fn derive_filter_validation(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    filter_validation::expand(ast).into()
}
