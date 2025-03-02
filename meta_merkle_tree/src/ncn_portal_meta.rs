use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct NcnPortalMeta {
    /// Rate Limiting
    rate_limiting: u64,
}
