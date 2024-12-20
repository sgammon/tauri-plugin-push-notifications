use base64::{engine::general_purpose, Engine as _};
use serde::de::DeserializeOwned;
use std::sync::Mutex;
use tauri::{plugin::PluginApi, AppHandle, Runtime, State};

use crate::models::*;
use crate::PushTokenState;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<PushNotifications<R>> {
    Ok(PushNotifications(app.clone()))
}

/// Access to the push-notifications APIs.
pub struct PushNotifications<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PushNotifications<R> {
    /// Requests permission to access push services.
    pub fn request_push_permission(
        &self,
        _state: State<Mutex<PushTokenState>>,
        _payload: PushPermissionRequest,
    ) -> crate::Result<PushPermissionResponse> {
        // desktop platforms don't use this hook
        Ok(PushPermissionResponse {
            granted: Some(true),
        })
    }

    /// Obtains the most recent push token.
    pub fn get_push_token(
        &self,
        state: State<Mutex<PushTokenState>>,
        _payload: PushTokenRequest,
    ) -> crate::Result<PushTokenResponse> {
        let state = state.lock().unwrap();

        match &state.token {
            Some(token) => {
                let encoded = general_purpose::STANDARD.encode(&token);
                Ok(PushTokenResponse {
                    value: Some(encoded.clone()),
                })
            }
            None => Ok(PushTokenResponse { value: None }),
        }
    }
}
