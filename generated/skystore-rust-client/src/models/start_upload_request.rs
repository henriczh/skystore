/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StartUploadRequest {
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "client_from_region")]
    pub client_from_region: String,
    #[serde(rename = "is_multipart")]
    pub is_multipart: bool,
    #[serde(rename = "copy_src_bucket", skip_serializing_if = "Option::is_none")]
    pub copy_src_bucket: Option<String>,
    #[serde(rename = "copy_src_key", skip_serializing_if = "Option::is_none")]
    pub copy_src_key: Option<String>,
}

impl StartUploadRequest {
    pub fn new(bucket: String, key: String, client_from_region: String, is_multipart: bool) -> StartUploadRequest {
        StartUploadRequest {
            bucket,
            key,
            client_from_region,
            is_multipart,
            copy_src_bucket: None,
            copy_src_key: None,
        }
    }
}


