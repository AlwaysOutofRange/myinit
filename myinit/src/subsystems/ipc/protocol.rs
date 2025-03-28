use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    GetStatus,
    StartService { name: String },
    StopService { name: String },
    RestartService { name: String },
    ListServices,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: ResponseStatus,
    pub message: String,
    pub data: Option<String>,
}

impl Response {
    pub fn success(message: &str) -> Self {
        Response {
            status: ResponseStatus::Success,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn success_with_data(message: &str, data: String) -> Self {
        Response {
            status: ResponseStatus::Success,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Response {
            status: ResponseStatus::Error,
            message: message.to_string(),
            data: None,
        }
    }
}

pub fn serialize_command(cmd: &Command) -> Result<String, crate::Error> {
    serde_json::to_string(cmd).map_err(|e| crate::Error {
        message: format!("Failed to serialize command: {}", e),
        fatal: false,
    })
}

pub fn deserialize_command(json: &str) -> Result<Command, crate::Error> {
    serde_json::from_str(json).map_err(|e| crate::Error {
        message: format!("Failed to deserialize command: {}", e),
        fatal: false,
    })
}

pub fn serialize_response(resp: &Response) -> Result<String, crate::Error> {
    serde_json::to_string(resp).map_err(|e| crate::Error {
        message: format!("Failed to serialize response: {}", e),
        fatal: false,
    })
}

pub fn deserialize_response(data: &str) -> Result<Response, crate::Error> {
    serde_json::from_str(data).map_err(|e| crate::Error {
        message: format!("Failed to deserialize response: {}", e),
        fatal: false,
    })
}
