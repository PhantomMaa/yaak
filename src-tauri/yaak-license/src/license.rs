use crate::error::Result;
use chrono::NaiveDateTime;
use log::warn;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime, WebviewWindow};
use ts_rs::TS;
use yaak_models::query_manager::QueryManagerExt;
use yaak_models::util::UpdateSource;

const KV_NAMESPACE: &str = "license";
const KV_ACTIVATION_ID_KEY: &str = "activation_id";

pub async fn activate_license<R: Runtime>(
    window: &WebviewWindow<R>,
    license_key: &str,
) -> Result<()> {
    // Store a fake activation ID to simulate activation
    let fake_activation_id = format!("local-{}", license_key.to_string().chars().take(8).collect::<String>());
    
    window.app_handle().db().set_key_value_string(
        KV_ACTIVATION_ID_KEY,
        KV_NAMESPACE,
        &fake_activation_id,
        &UpdateSource::from_window(&window),
    );

    if let Err(e) = window.emit("license-activated", true) {
        warn!("Failed to emit check-license event: {}", e);
    }

    Ok(())
}

pub async fn deactivate_license<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    let app_handle = window.app_handle();

    // Simply remove the local activation ID without network request
    app_handle.db().delete_key_value(
        KV_ACTIVATION_ID_KEY,
        KV_NAMESPACE,
        &UpdateSource::from_window(&window),
    )?;

    if let Err(e) = app_handle.emit("license-deactivated", true) {
        warn!("Failed to emit deactivate-license event: {}", e);
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case", tag = "type")]
#[ts(export, export_to = "license.ts")]
pub enum LicenseCheckStatus {
    PersonalUse { trial_ended: NaiveDateTime },
    CommercialUse,
    InvalidLicense,
    Trialing { end: NaiveDateTime },
}

pub async fn check_license<R: Runtime>(_window: &WebviewWindow<R>) -> Result<LicenseCheckStatus> {
    // Always return CommercialUse to bypass all license checks
    Ok(LicenseCheckStatus::CommercialUse)
}

pub async fn get_activation_id<R: Runtime>(app_handle: &AppHandle<R>) -> String {
    app_handle.db().get_key_value_string(KV_ACTIVATION_ID_KEY, KV_NAMESPACE, "")
}
