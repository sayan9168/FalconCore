use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Capability { NetworkScan, FilesystemRead, FilesystemWrite, ProcessSpawn }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityPolicy { allowed: BTreeSet<Capability>, limits: BTreeMap<Capability, u64> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError { Denied(Capability), LimitExceeded(Capability) }

impl CapabilityPolicy {
    pub fn deny_all() -> Self { Self { allowed: BTreeSet::new(), limits: BTreeMap::new() } }
    pub fn allow(mut self, capability: Capability) -> Self { self.allowed.insert(capability); self }
    pub fn limit(mut self, capability: Capability, max_uses: u64) -> Self { self.limits.insert(capability, max_uses); self }
    pub fn allows(&self, capability: &Capability) -> bool { self.allowed.contains(capability) }
    pub fn authorize(&self, capability: &Capability, uses: u64) -> Result<(), CapabilityError> {
        if !self.allows(capability) { return Err(CapabilityError::Denied(capability.clone())); }
        if let Some(limit) = self.limits.get(capability) { if uses >= *limit { return Err(CapabilityError::LimitExceeded(capability.clone())); } }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CapabilityRegistry { policy: CapabilityPolicy, usage: BTreeMap<Capability, u64> }

impl CapabilityRegistry {
    pub fn new(policy: CapabilityPolicy) -> Self { Self { policy, usage: BTreeMap::new() } }
    pub fn request(&mut self, capability: Capability) -> Result<(), CapabilityError> {
        let used = self.usage.get(&capability).copied().unwrap_or(0);
        self.policy.authorize(&capability, used)?;
        self.usage.insert(capability, used + 1);
        Ok(())
    }
    pub fn request_many<I>(&mut self, capabilities: I) -> Result<(), CapabilityError>
    where I: IntoIterator<Item = Capability> { for capability in capabilities { self.request(capability)?; } Ok(()) }
    pub fn usage(&self, capability: &Capability) -> u64 { self.usage.get(capability).copied().unwrap_or(0) }
    pub fn policy_allows(&self, capability: &Capability) -> bool { self.policy.allows(capability) }
    pub fn snapshot(&self) -> BTreeMap<Capability, u64> { self.usage.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn deny_by_default() {
        let mut r = CapabilityRegistry::new(CapabilityPolicy::deny_all());
        assert!(matches!(r.request(Capability::NetworkScan), Err(CapabilityError::Denied(Capability::NetworkScan))));
    }
    #[test] fn enforce_usage_limit() {
        let p = CapabilityPolicy::deny_all().allow(Capability::NetworkScan).limit(Capability::NetworkScan, 2);
        let mut r = CapabilityRegistry::new(p);
        assert!(r.request(Capability::NetworkScan).is_ok());
        assert!(r.request(Capability::NetworkScan).is_ok());
        assert!(matches!(r.request(Capability::NetworkScan), Err(CapabilityError::LimitExceeded(Capability::NetworkScan))));
    }
    #[test] fn batch_requests_are_policy_checked() {
        let p = CapabilityPolicy::deny_all().allow(Capability::FilesystemRead);
        let mut r = CapabilityRegistry::new(p);
        assert!(r.request_many([Capability::FilesystemRead]).is_ok());
        assert_eq!(r.usage(&Capability::FilesystemRead), 1);
        assert!(!r.policy_allows(&Capability::ProcessSpawn));
    }
}