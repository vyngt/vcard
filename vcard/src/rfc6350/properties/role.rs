use super::super::values::Role;
use crate::common::{VCardProperty, VCardValue};

pub struct RoleProperty {
    roles: Vec<Role>,
}

impl RoleProperty {
    pub fn new() -> Self {
        Self { roles: vec![] }
    }

    pub fn add(&mut self, role: Role) {
        self.roles.push(role);
    }
}

impl VCardProperty for RoleProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for role in &self.roles {
            output.push_str(&role.format_value());
        }
        output
    }
}
