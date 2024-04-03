/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthcheckResponse {
    #[serde(rename = "status")]
    pub status: Status,
}

impl HealthcheckResponse {
    pub fn new(status: Status) -> HealthcheckResponse {
        HealthcheckResponse {
            status,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OK")]
    Ok,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ok
    }
}

