mod bool_attribute;
mod path_attribute;
mod string_literal_attribute;
mod integer_literal_attribute;
mod expr_method_call;

//function for every macro attribute type
//from deriveInput is used for attributes on the struct
//from field is used for attributes on the fields of the struct

pub use bool_attribute::from_field as bool_attr_from_field;

pub use path_attribute::from_derive_input as path_attr_from_derive_input;
pub use path_attribute::from_field as path_attr_from_field;

pub use string_literal_attribute::from_field as string_lit_attr_from_field;

pub use integer_literal_attribute::from_field as int_lit_attr_from_field;

pub use expr_method_call::from_derive_input as expr_method_call_from_derive_input;