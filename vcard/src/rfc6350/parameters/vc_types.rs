use std::fmt;

/// ref: https://www.rfc-editor.org/rfc/rfc6350#section-5.6
#[derive(Debug)]
pub enum BaseVCType {
    WORK,
    HOME,
}

impl fmt::Display for BaseVCType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
/// ref: https://www.rfc-editor.org/rfc/rfc6350#section-6.4.1
///
/// MUST NOT be used with a property other than TEL.
#[derive(Debug)]
pub enum TelVCType {
    TEXT,
    VOICE,
    FAX,
    CELL,
    VIDEO,
    PAGER,
    TEXTPHONE,
}

impl fmt::Display for TelVCType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ref: https://www.rfc-editor.org/rfc/rfc6350#section-5.6
#[derive(Debug)]
pub enum VCardType {
    Base(BaseVCType),
    Tel(TelVCType),
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

pub struct VCTypeParam {
    value: VCardType,
}

impl VCTypeParam {
    pub fn new(vc_type: VCardType) -> Self {
        Self { value: vc_type }
    }

    pub fn set(&mut self, vc_type: VCardType) {
        self.value = vc_type;
    }

    pub fn from_x_name(x_name: &str) -> Self {
        let mut x = String::new();
        let m = x_name.split_ascii_whitespace();
        for s in m {
            x.push_str(s)
        }
        Self {
            value: VCardType::XName(x.to_uppercase()),
        }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

pub struct VCardTypeParams {
    types: Vec<VCTypeParam>,
}

impl VCardTypeParams {
    pub fn new() -> Self {
        Self { types: vec![] }
    }

    pub fn push(&mut self, vc_type: VCardType) {
        self.types.push(VCTypeParam::new(vc_type))
    }

    pub fn push_base(&mut self, base: BaseVCType) {
        self.push(VCardType::Base(base))
    }

    pub fn push_tel(&mut self, tel: TelVCType) {
        self.push(VCardType::Tel(tel))
    }

    pub fn push_x_name(&mut self, x_name: &str) {
        self.types.push(VCTypeParam::from_x_name(x_name));
    }

    pub fn to_value(&self) -> String {
        let mut out = String::from("");
        for i in 0..self.types.len() {
            let value = &self.types[i].to_string();
            if i == 0 {
                out.push_str(value);
            } else {
                out.push_str(&format!(",{}", value));
            }
        }
        out
    }
}
