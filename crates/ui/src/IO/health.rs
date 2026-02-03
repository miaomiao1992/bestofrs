use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct HealthCheckDto {
    pub ok: bool,
}

#[get("/api/health")]
pub async fn health_check() -> ServerFnResult<HealthCheckDto> {
    Ok(HealthCheckDto { ok: true })
}
