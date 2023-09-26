use crate::common::VCardParam;
use std::fmt;
use vcard_derive::vcard_property_type;

/// ref: `https://www.rfc-editor.org/rfc/rfc6350#section-5.6`
#[derive(Debug)]
pub enum BaseType {
    WORK,
    HOME,
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
/// ref: `https://www.rfc-editor.org/rfc/rfc6350#section-6.4.1`
///
/// MUST NOT be used with a property other than TEL.
#[derive(Debug)]
pub enum TelType {
    TEXT,
    VOICE,
    FAX,
    CELL,
    VIDEO,
    PAGER,
    TEXTPHONE,
}

impl fmt::Display for TelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ref: `https://www.rfc-editor.org/rfc/rfc6350#section-5.6`
pub enum VCardType {
    Base(BaseType),
    Tel(TelType),
    XName(String),
}

#[vcard_property_type("TYPE")]
pub struct TypeParam {
    types: Vec<VCardType>,
}

impl TypeParam {
    pub fn new() -> Self {
        Self { types: vec![] }
    }

    pub fn add(mut self, vc_type: VCardType) -> Self {
        self.types.push(vc_type);
        self
    }

    pub fn add_base(self, base: BaseType) -> Self {
        self.add(VCardType::Base(base))
    }

    pub fn add_tel(self, tel: TelType) -> Self {
        self.add(VCardType::Tel(tel))
    }

    /// Note
    pub fn add_x_name(self, x_name: &str) -> Self {
        match Self::format_x_name(x_name) {
            Some(xn) => self.add(VCardType::XName(xn.to_uppercase())),
            None => self,
        }
    }

    fn format_x_name(x_name: &str) -> Option<String> {
        let mut x = String::new();
        let m = x_name.split_ascii_whitespace();
        for s in m {
            x.push_str(s)
        }
        if x.len() > 0 {
            Some(x)
        } else {
            None
        }
    }
}

impl VCardParam for TypeParam {
    fn format_param(&self) -> String {
        let mut out = vec![];
        for i in 0..self.types.len() {
            let type_str = match &self.types[i] {
                VCardType::Base(base) => base.to_string(),
                VCardType::Tel(tel) => tel.to_string(),
                VCardType::XName(xn) => match Self::format_x_name(xn) {
                    Some(xn) => xn.to_uppercase(),
                    None => "".into(),
                },
            };

            if type_str.len() > 0 {
                out.push(type_str);
            }
        }

        if out.len() > 0 {
            format!(";{}={}", Self::get_value_type(), out.join(","))
        } else {
            "".into()
        }
    }
}
