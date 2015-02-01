use std::error::{Error};

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct WrongType {
    pub path: String,
    pub detail: String
}
impl_err!(WrongType, "wrong_type", "Type of the value is wrong", +detail);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MultipleOf {
    pub path: String
}
impl_err!(MultipleOf, "multiple_of", "Wrong number of the value");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Maximum {
    pub path: String
}
impl_err!(Maximum, "maximum", "Maximum condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Minimum {
    pub path: String
}
impl_err!(Minimum, "minimum", "Minimum condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxLength {
    pub path: String
}
impl_err!(MaxLength, "max_length", "MaxLength condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MinLength {
    pub path: String
}
impl_err!(MinLength, "min_length", "MinLength condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Pattern {
    pub path: String
}
impl_err!(Pattern, "pattern", "Pattern condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxItems {
    pub path: String
}
impl_err!(MaxItems, "max_items", "MaxItems condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MinItems {
    pub path: String
}
impl_err!(MinItems, "min_items", "MinItems condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct UniqueItems {
    pub path: String
}
impl_err!(UniqueItems, "unique_items", "UniqueItems condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Items {
    pub path: String,
    pub detail: String
}
impl_err!(Items, "items", "Items condition is not met", +detail);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MaxProperties {
    pub path: String
}
impl_err!(MaxProperties, "max_properties", "MaxProperties condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct MinProperties {
    pub path: String
}
impl_err!(MinProperties, "min_properties", "MinProperties condition is not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Required {
    pub path: String
}
impl_err!(Required, "required", "This property is required");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Properties {
    pub path: String,
    pub detail: String
}
impl_err!(Properties, "properties", "Property conditions are not met", +detail);

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct Enum {
    pub path: String
}
impl_err!(Enum, "enum", "Enum conditions are not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct AnyOf {
    pub path: String,
    pub states: Vec<super::validators::ValidationState>
}
impl_err!(AnyOf, "any_of", "AnyOf conditions are not met");

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct OneOf {
    pub path: String,
    pub states: Vec<super::validators::ValidationState>
}
impl_err!(OneOf, "one_of", "OneOf conditions are not met");

