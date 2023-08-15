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
pub struct HeadObjectResponse {
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "size")]
    pub size: u64,
    #[serde(rename = "etag")]
    pub etag: String,
    #[serde(rename = "last_modified")]
    pub last_modified: String,
}

impl HeadObjectResponse {
    pub fn new(bucket: String, key: String, size: u64, etag: String, last_modified: String) -> HeadObjectResponse {
        HeadObjectResponse {
            bucket,
            key,
            size,
            etag,
            last_modified,
        }
    }
}


