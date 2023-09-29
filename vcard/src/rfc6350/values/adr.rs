use super::super::parameters::{PrefParam, TypeParam, VCardType, ValueParam};
use crate::common::VCardValue;
use vcard_derive::vcard_property_type;

#[vcard_property_type("ADR")]
pub struct Address {
    street: ValueParam,
    locality: ValueParam,
    region: ValueParam,
    code: ValueParam,
    country: ValueParam,
    type_param: TypeParam,
    pref_param: PrefParam,
}

impl Address {
    pub fn new() -> Self {
        Self {
            street: ValueParam::new(),
            locality: ValueParam::new(),
            region: ValueParam::new(),
            code: ValueParam::new(),
            country: ValueParam::new(),
            type_param: TypeParam::new(),
            pref_param: PrefParam::new(),
        }
    }

    /// #### Set Street
    /// e.g., 123 Main Street
    pub fn street(mut self, street: &str) -> Self {
        self.street.set(street);
        self
    }

    /// #### Set Locality
    /// e.g., city
    pub fn locality(mut self, locality: &str) -> Self {
        self.locality.set(locality);
        self
    }

    /// #### Set Locality
    /// e.g., state or province
    pub fn region(mut self, region: &str) -> Self {
        self.region.set(region);
        self
    }

    /// #### Set Postal code
    /// e.g., 91921-1234
    pub fn code(mut self, code: &str) -> Self {
        self.code.set(code);
        self
    }

    /// #### Set Country
    /// e.g., Vietnam, U.S.A
    pub fn country(mut self, country: &str) -> Self {
        self.country.set(country);
        self
    }

    pub fn add_type(mut self, vc_type: VCardType) -> Self {
        let tp: TypeParam = self.type_param;
        self.type_param = tp.add(vc_type);
        self
    }

    pub fn set_prefer(mut self, preferred: u8) -> Self {
        self.pref_param.set(preferred).unwrap();
        self
    }

    /// First two components should be empty
    ///
    /// Ref: `https://datatracker.ietf.org/doc/html/rfc6350#section-6.3.1`
    fn format_adr(&self) -> String {
        format!(
            ";;{};{};{};{};{}",
            self.street, self.locality, self.region, self.code, self.country
        )
    }
}

impl VCardValue for Address {
    fn format_value(&self) -> String {
        let value = self.format_adr();
        if value.len() > 6 {
            format!(
                "{}{}{}:{}\n",
                Self::get_value_type(),
                self.pref_param,
                self.type_param,
                value
            )
        } else {
            "".into()
        }
    }
}
