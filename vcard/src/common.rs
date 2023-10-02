pub trait VCardProperty {
    fn to_content(&self) -> String;
}

pub trait VCardValue {
    /// Format vCard value
    fn format_value(&self) -> String;
}

pub trait VCardParam {
    /// Format vCard parameter
    fn format_param(&self) -> String;
}
