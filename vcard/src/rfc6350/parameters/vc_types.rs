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
#[derive(Debug)]
pub enum VCardType {
    Base(BaseType),
    Tel(TelType),
    XName(String),
}

impl fmt::Display for VCardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VCardType::Base(base) => base.fmt(f),
            VCardType::Tel(tel) => tel.fmt(f),
            VCardType::XName(x_name) => write!(f, "{}", x_name),
        }
    }
}

#[vcard_property_type("TYPE")]
pub struct TypeParam {
    types: Vec<VCardType>,
}

impl TypeParam {
    pub fn new() -> Self {
        Self { types: vec![] }
    }

    pub fn push(&mut self, vc_type: VCardType) {
        self.types.push(vc_type)
    }

    pub fn push_base(&mut self, base: BaseType) {
        self.push(VCardType::Base(base))
    }

    pub fn push_tel(&mut self, tel: TelType) {
        self.push(VCardType::Tel(tel))
    }

    pub fn push_x_name(&mut self, x_name: &str) {
        let mut x = String::new();
        let m = x_name.split_ascii_whitespace();
        for s in m {
            x.push_str(s)
        }
        if x.len() > 0 {
            self.types.push(VCardType::XName(x.to_uppercase()));
        }
    }
}

impl VCardParam for TypeParam {
    fn format_param(&self) -> String {
        let mut out = String::from("");
        for i in 0..self.types.len() {
            let value = &self.types[i].to_string();
            if i == 0 {
                out.push_str(value);
            } else {
                out.push_str(&format!(",{}", value));
            }
        }
        if out.len() > 0 {
            format!(";{}=\"{}\"", Self::get_value_type(), out)
        } else {
            "".into()
        }
    }
}
