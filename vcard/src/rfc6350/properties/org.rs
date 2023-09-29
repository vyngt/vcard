use super::super::values::Organization;
use crate::common::{VCardProperty, VCardValue};

pub struct OrganizationProperty {
    organizations: Vec<Organization>,
}

impl OrganizationProperty {
    pub fn new() -> Self {
        Self {
            organizations: vec![],
        }
    }

    pub fn add(&mut self, organization: Organization) {
        self.organizations.push(organization);
    }
}

impl VCardProperty for OrganizationProperty {
    fn to_content(&self) -> String {
        let mut output = String::from("");
        for organization in &self.organizations {
            output.push_str(&organization.format_value());
        }
        output
    }
}
