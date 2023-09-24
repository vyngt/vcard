pub trait VCardProperty {
    fn to_content(&self) -> String;
}

pub trait VCardValue {
    fn format_value(&self) -> String;
}
