use async_trait::async_trait;
use crate::{core::types::Finding, error::SubsnifferError};

#[async_trait]
pub trait Source: Send + Sync {
    fn name(&self) -> &'static str;

    async fn enumerate(
        &self,
        domain: &str,
    ) -> Result<Vec<Finding>, SubsnifferError>;
}
