//! Module for syncing data to the F3 API

use crate::app_state::backblast_data::BackBlastData;
use crate::shared::common_errors::AppError;
use serde::{Deserialize, Serialize};
use std::env;

/// Request payload for syncing a backblast to F3 API
#[derive(Debug, Serialize)]
pub struct SyncBackblastRequest {
    pub old_id: String,
    pub ao_name: String,
    pub q_names: Vec<String>,
    pub pax_names: Vec<String>,
    pub fng_names: Vec<String>,
    pub date: String,
    pub title: Option<String>,
    pub moleskine: Option<String>,
    pub bb_type: Option<String>,
}

/// Response from F3 API sync endpoints
#[derive(Debug, Deserialize)]
pub struct SyncResponse {
    pub id: Option<i32>,
    pub action: String,
}

impl SyncBackblastRequest {
    pub fn from_data(data: &BackBlastData, id: &str) -> Self {
        let ao_name = data.ao.to_string();
        let q_names: Vec<String> = data.qs.iter().cloned().collect();
        let pax_names: Vec<String> = data.get_pax().into_iter().collect();
        let fng_names: Vec<String> = data.fngs.iter().cloned().collect();
        let date = data.date.format("%Y-%m-%d").to_string();
        let bb_type = Some(data.bb_type.to_string());

        SyncBackblastRequest {
            old_id: id.to_string(),
            ao_name,
            q_names,
            pax_names,
            fng_names,
            date,
            title: data.title.clone(),
            moleskine: data.moleskine.clone(),
            bb_type,
        }
    }
}

/// F3 API sync client
pub struct F3ApiClient {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl F3ApiClient {
    /// Create a new F3 API client from environment variables
    pub fn from_env() -> Option<Self> {
        let base_url = env::var("F3_API_BASE_URL").ok()?;
        let api_key = env::var("F3_SYNC_API_KEY").ok()?;

        Some(Self {
            base_url,
            api_key,
            client: reqwest::Client::new(),
        })
    }

    /// Sync a backblast to the F3 API
    pub async fn sync_backblast(
        &self,
        data: &BackBlastData,
        id: &str,
    ) -> Result<SyncResponse, AppError> {
        let request = SyncBackblastRequest::from_data(data, id);
        let url = format!("{}/shell/sync/backblast", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let sync_response: SyncResponse = response.json().await?;
            tracing::info!(
                "Synced backblast {} to F3 API: {:?}",
                request.old_id,
                sync_response
            );
            Ok(sync_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!(
                "Failed to sync backblast {} to F3 API: {} - {}",
                request.old_id,
                status,
                error_text
            );
            Err(AppError::General(format!(
                "F3 API sync failed: {} - {}",
                status, error_text
            )))
        }
    }
}

/// Helper function to sync backblast (logs errors but doesn't fail the main operation)
pub async fn sync_backblast(data: &BackBlastData, id: &str) {
    tracing::info!("Attempting to sync backblast {} to F3 API", id);
    if let Some(client) = F3ApiClient::from_env() {
        match client.sync_backblast(data, id).await {
            Ok(response) => {
                tracing::info!(
                    "Backblast {} synced successfully: action={:?}, new_id={:?}",
                    id,
                    response.action,
                    response.id
                );
            }
            Err(e) => {
                tracing::warn!("Backblast sync to F3 API failed: {:?}", e);
            }
        }
    } else {
        tracing::info!("F3 API sync not configured (missing F3_API_BASE_URL or F3_SYNC_API_KEY), skipping backblast sync");
    }
}
