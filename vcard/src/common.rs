pub trait VCardProperty {
    fn to_content(&self) -> String;
}

pub trait VCardValue {
    fn format_value(&self) -> String;
}

pub trait VCardParam {
    fn format_param(&self) -> String;
}
