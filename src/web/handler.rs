//! Module holding internal handlers for health checks etc.

use crate::{error::Error::*, WebResult, DB};
use log::error;
use warp::{reject, Reply};

const HEALTH: &str = "OK";

pub async fn health_handler(db: DB) -> WebResult<impl Reply> {
    // TODO: better health check for mongo
    let _ = db
        .list_collection_names(None)
        .await
        .map_err(|e| reject::custom(MongoError(e)))?;
    Ok(HEALTH)
}

pub async fn metrics_handler() -> WebResult<impl Reply> {
    use prometheus::Encoder;
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        error!("could not encode prometheus metrics: {}", e);
    };
    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            error!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();
    Ok(res)
}
