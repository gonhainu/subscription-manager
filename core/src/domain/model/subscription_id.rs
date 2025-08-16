use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SubscriptionId(uuid::Uuid);

impl SubscriptionId {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    pub fn raw(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for SubscriptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for SubscriptionId {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}

impl From<uuid::Uuid> for SubscriptionId {
    fn from(u: uuid::Uuid) -> Self {
        Self(u)
    }
}

impl std::fmt::Display for SubscriptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .as_simple()
                .encode_lower(&mut uuid::Uuid::encode_buffer())
        )
    }
}

impl From<SubscriptionId> for String {
    fn from(id: SubscriptionId) -> Self {
        id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_id_from_str() {
        let id_string = SubscriptionId::new().to_string();
        let id = SubscriptionId::from_str(&id_string);
        assert!(id.is_ok());
    }

    #[test]
    fn test_subscription_id_invalid() {
        let id = SubscriptionId::from_str("invalid-uuid");
        assert!(id.is_err());
    }
}
