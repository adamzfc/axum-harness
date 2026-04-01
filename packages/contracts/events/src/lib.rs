//! contracts/events — Domain event payload types.

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

/// Tenant created event.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, TS)]
#[ts(export, export_to = "events/")]
pub struct TenantCreated {
    pub tenant_id: String,
    pub owner_sub: String,
}

/// Tenant member added event.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, TS)]
#[ts(export, export_to = "events/")]
pub struct TenantMemberAdded {
    pub tenant_id: String,
    pub user_sub: String,
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_tenant_created() {
        TenantCreated::export().unwrap();
    }

    #[test]
    fn export_tenant_member_added() {
        TenantMemberAdded::export().unwrap();
    }
}
