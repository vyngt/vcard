use super::super::values::Address;
use crate::common::{VCardProperty, VCardValue};

pub struct AddressProperty {
    addresses: Vec<Address>,
}

impl AddressProperty {
    pub fn new() -> Self {
        Self { addresses: vec![] }
    }

    pub fn add(&mut self, address: Address) {
        self.addresses.push(address);
    }
}

impl VCardProperty for AddressProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for address in &self.addresses {
            output.push_str(&address.format_value());
        }
        output
    }
}
