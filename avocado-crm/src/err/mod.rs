use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};
use std::collections::HashMap;
use tonic::{Code, Status};

pub(crate) struct JsonError(anyhow::Error);

impl IntoResponse for JsonError {
    fn into_response(self) -> Response {
        tracing::error!("App Error: {:?}", self.0);
        if let Some(s) = self.0.downcast_ref::<Status>() {
            tonic_status_to_response(s)
        } else if let Some(j) = self.0.downcast_ref::<JsonRejection>() {
            json_rejection_to_response(j)
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("{}", self.0),
                })),
            )
                .into_response()
        }
    }
}

fn tonic_status_to_response(status: &Status) -> Response {
    let message = Json(json!({"error": status.message()}));
    match status.code() {
        Code::InvalidArgument => {
            if let Ok(error) = serde_json::from_str::<Value>(status.message()) {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::to_value(HashMap::from([("error", error)])).unwrap()),
                )
            } else {
                (StatusCode::BAD_REQUEST, message)
            }
        }
        Code::NotFound => (StatusCode::NOT_FOUND, message),
        Code::PermissionDenied => (StatusCode::UNAUTHORIZED, message),
        Code::Unimplemented => (StatusCode::NOT_IMPLEMENTED, message),
        Code::Unauthenticated => (StatusCode::UNAUTHORIZED, message),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, message),
    }
    .into_response()
}

fn json_rejection_to_response(error: &JsonRejection) -> Response {
    let message = Json(json!({"error": error.to_string()}));
    match error {
        JsonRejection::JsonDataError(_) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        JsonRejection::JsonSyntaxError(_) => (StatusCode::BAD_REQUEST, message),
        JsonRejection::MissingJsonContentType(_) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, message),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, message),
    }
    .into_response()
}

impl<E> From<E> for JsonError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
