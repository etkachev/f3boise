//! Module for syncing data to the F3 API

use crate::app_state::backblast_data::BackBlastData;
use crate::app_state::pre_blast_data::PreBlastData;
use crate::db::save_q_line_up::NewQLineUpDbEntry;
use crate::shared::common_errors::AppError;
use crate::users::f3_user::F3User;
use serde::{Deserialize, Serialize};
use std::env;

// ─────────────────────────────────────────────────────────────────────────────
// Request/Response Types
// ─────────────────────────────────────────────────────────────────────────────

/// Request payload for syncing a backblast to F3 API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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

/// Request payload for syncing a preblast to F3 API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncPreblastRequest {
    pub old_id: String,
    pub ao_name: String,
    pub q_names: Vec<String>,
    pub date: String,
    pub start_time: String,
    pub title: Option<String>,
    pub equipment: Vec<String>,
    pub why: Option<String>,
    pub fng_message: Option<String>,
    pub mole_skin: Option<String>,
    pub location_url: Option<String>,
}

/// Request payload for syncing a Q signup to F3 API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncQSignUpRequest {
    pub ao_name: String,
    pub date: String,
    pub q_name: String,
    pub closed: bool,
}

/// Request payload for syncing a user to F3 API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncUserRequest {
    pub slack_id: String,
    pub name: String,
    pub email: String,
    pub parent: Option<String>,
}

/// Response from F3 API sync endpoints
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResponse {
    pub id: Option<i32>,
    pub action: String,
}

/// Response from F3 API user sync endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncUserResponse {
    pub id: String,
    pub action: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Request Builders
// ─────────────────────────────────────────────────────────────────────────────

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

impl SyncPreblastRequest {
    pub fn from_data(data: &PreBlastData, id: &str) -> Self {
        let ao_name = data.ao.to_string();
        let q_names: Vec<String> = data.qs.iter().cloned().collect();
        let date = data.date.format("%Y-%m-%d").to_string();
        let start_time = data.start_time.format("%H:%M").to_string();
        let equipment: Vec<String> = data.equipment.iter().map(|e| e.to_string()).collect();

        SyncPreblastRequest {
            old_id: id.to_string(),
            ao_name,
            q_names,
            date,
            start_time,
            title: Some(data.title.clone()),
            equipment,
            why: Some(data.why.clone()),
            fng_message: data.fng_message.clone(),
            mole_skin: data.mole_skin.clone(),
            location_url: data.location_url.clone(),
        }
    }
}

impl SyncQSignUpRequest {
    pub fn from_data(data: &NewQLineUpDbEntry) -> Self {
        let date = data.date.format("%Y-%m-%d").to_string();
        // Get the first Q name (qs is comma-separated)
        let q_name = data.qs.split(',').next().unwrap_or("").trim().to_string();

        SyncQSignUpRequest {
            ao_name: data.ao.clone(),
            date,
            q_name,
            closed: data.closed,
        }
    }
}

impl SyncUserRequest {
    pub fn from_data(user: &F3User) -> Self {
        use crate::db::pax_parent_tree::F3Parent;

        let parent = user.parent.as_ref().and_then(|p| match p {
            F3Parent::Pax(pax) => Some(pax.name.clone()),
            _ => None,
        });

        SyncUserRequest {
            slack_id: user.id.clone().unwrap_or_default(),
            name: user.name.clone(),
            email: user.email.clone(),
            parent,
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
        self.post_sync(&url, &request, "backblast", &request.old_id)
            .await
    }

    /// Sync a preblast to the F3 API
    pub async fn sync_preblast(
        &self,
        data: &PreBlastData,
        id: &str,
    ) -> Result<SyncResponse, AppError> {
        let request = SyncPreblastRequest::from_data(data, id);
        let url = format!("{}/shell/sync/preblast", self.base_url);
        self.post_sync(&url, &request, "preblast", &request.old_id)
            .await
    }

    /// Sync a Q signup to the F3 API
    pub async fn sync_q_signup(&self, data: &NewQLineUpDbEntry) -> Result<SyncResponse, AppError> {
        let request = SyncQSignUpRequest::from_data(data);
        let url = format!("{}/shell/sync/q_signup", self.base_url);
        let id = format!("{}:{}", request.ao_name, request.date);
        self.post_sync(&url, &request, "q_signup", &id).await
    }

    /// Sync a user to the F3 API
    pub async fn sync_user(&self, user: &F3User) -> Result<SyncUserResponse, AppError> {
        let request = SyncUserRequest::from_data(user);
        let url = format!("{}/shell/sync/user", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let sync_response: SyncUserResponse = response.json().await?;
            println!(
                "Synced user {} to F3 API: action={}",
                request.name, sync_response.action
            );
            Ok(sync_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            println!(
                "Failed to sync user {} to F3 API: {} - {}",
                request.name, status, error_text
            );
            Err(AppError::General(format!(
                "F3 API sync failed: {} - {}",
                status, error_text
            )))
        }
    }

    /// Generic POST for sync endpoints
    async fn post_sync<T: Serialize>(
        &self,
        url: &str,
        request: &T,
        entity_type: &str,
        id: &str,
    ) -> Result<SyncResponse, AppError> {
        let response = self
            .client
            .post(url)
            .header("X-API-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await?;

        if response.status().is_success() {
            let sync_response: SyncResponse = response.json().await?;
            println!(
                "Synced {} {} to F3 API: action={}, new_id={:?}",
                entity_type, id, sync_response.action, sync_response.id
            );
            Ok(sync_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            println!(
                "Failed to sync {} {} to F3 API: {} - {}",
                entity_type, id, status, error_text
            );
            Err(AppError::General(format!(
                "F3 API sync failed: {} - {}",
                status, error_text
            )))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper Functions (safe wrappers that don't fail main operations)
// ─────────────────────────────────────────────────────────────────────────────

/// Helper function to sync backblast (logs errors but doesn't fail the main operation)
pub async fn sync_backblast(data: &BackBlastData, id: &str) {
    println!("Attempting to sync backblast {} to F3 API", id);
    if let Some(client) = F3ApiClient::from_env() {
        if let Err(e) = client.sync_backblast(data, id).await {
            println!("Backblast sync to F3 API failed: {:?}", e);
        }
    } else {
        println!("F3 API sync not configured, skipping backblast sync");
    }
}

/// Helper function to sync preblast (logs errors but doesn't fail the main operation)
pub async fn sync_preblast(data: &PreBlastData, id: &str) {
    println!("Attempting to sync preblast {} to F3 API", id);
    if let Some(client) = F3ApiClient::from_env() {
        if let Err(e) = client.sync_preblast(data, id).await {
            println!("Preblast sync to F3 API failed: {:?}", e);
        }
    } else {
        println!("F3 API sync not configured, skipping preblast sync");
    }
}

/// Helper function to sync Q signup (logs errors but doesn't fail the main operation)
pub async fn sync_q_signup(data: &NewQLineUpDbEntry) {
    println!(
        "Attempting to sync Q signup {}:{} to F3 API",
        data.ao, data.date
    );
    if let Some(client) = F3ApiClient::from_env() {
        if let Err(e) = client.sync_q_signup(data).await {
            println!("Q signup sync to F3 API failed: {:?}", e);
        }
    } else {
        println!("F3 API sync not configured, skipping Q signup sync");
    }
}

/// Helper function to sync user (logs errors but doesn't fail the main operation)
pub async fn sync_user(user: &F3User) {
    println!("Attempting to sync user {} to F3 API", user.name);
    if let Some(client) = F3ApiClient::from_env() {
        if let Err(e) = client.sync_user(user).await {
            println!("User sync to F3 API failed: {:?}", e);
        }
    } else {
        println!("F3 API sync not configured, skipping user sync");
    }
}
