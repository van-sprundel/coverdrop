use async_trait::async_trait;
use chrono::Duration;
use common::{
    api::api_client::ApiClient, protocol::constants::MINUTE_IN_SECONDS, task::Task, time,
};
use journalist_vault::JournalistVault;

use crate::app_state::PublicInfo;

pub struct RefreshPublicInfo {
    api_client: ApiClient,
    vault: JournalistVault,
    public_info: PublicInfo,
}

impl RefreshPublicInfo {
    pub fn new(api_client: &ApiClient, vault: &JournalistVault, public_info: &PublicInfo) -> Self {
        Self {
            api_client: api_client.clone(),
            vault: vault.clone(),
            public_info: public_info.clone(),
        }
    }
}

#[async_trait]
impl Task for RefreshPublicInfo {
    fn name(&self) -> &'static str {
        "refresh_public_info"
    }

    async fn run(&self) -> anyhow::Result<()> {
        let anchor_org_pks = self.vault.org_pks(time::now()).await?;

        tracing::debug!("Getting public info from API {:?}", self.api_client);

        let public_info = self
            .api_client
            .get_public_keys()
            .await?
            .into_trusted(&anchor_org_pks, time::now());

        tracing::debug!("Setting public info");

        self.public_info.set(public_info).await;

        tracing::debug!("Public info should be set");

        Ok(())
    }

    fn interval(&self) -> Duration {
        Duration::seconds(MINUTE_IN_SECONDS)
    }
}
