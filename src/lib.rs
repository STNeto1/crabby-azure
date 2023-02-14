use anyhow::anyhow;
use azure_messaging_servicebus::service_bus::QueueClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub message: String,
}

impl Payload {
    pub fn to_string(&self) -> Result<String, anyhow::Error> {
        let rst = serde_json::to_string(&self).map_err(|e| anyhow!("{}", e))?;

        return Ok(rst);
    }

    pub fn from_string(raw_data: &String) -> Result<Self, anyhow::Error> {
        let payload: Payload =
            serde_json::from_str(&raw_data.to_string()).map_err(|e| anyhow!("{}", e))?;

        return Ok(payload);
    }
}

pub fn create_azure_client() -> QueueClient {
    let service_bus_namespace =
        std::env::var("AZURE_SERVICE_BUS_NAMESPACE").expect("missing AZURE_SERVICE_BUS_NAMESPACE");
    let queue_name = std::env::var("AZURE_QUEUE_NAME").expect("missing AZURE_QUEUE_NAME");
    let policy_name = std::env::var("AZURE_POLICY_NAME").expect("missing AZURE_POLICY_NAME");
    let policy_key = std::env::var("AZURE_POLICY_KEY").expect("missing AZURE_POLICY_KEY");

    let http_client = azure_core::new_http_client();
    let client = QueueClient::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )
    .unwrap();

    return client;
}
