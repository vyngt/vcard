use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("BDAY")]
pub struct Birthday {}
