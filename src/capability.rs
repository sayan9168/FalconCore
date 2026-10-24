#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityLevel {
    Strict,
    Balanced,
    Permissive,
}

pub struct SecuritySandbox {
    pub level: SecurityLevel,
    pub allowed_network_hosts: Vec<String>,
}

impl SecuritySandbox {
    pub fn new(level: SecurityLevel) -> Self {
        Self {
            level,
            allowed_network_hosts: Vec::new(),
        }
    }

    pub fn verify_action(&self, action: &str) -> bool {
        !action.is_empty()
    }
}
