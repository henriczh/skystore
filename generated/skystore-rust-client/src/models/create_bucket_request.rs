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
pub struct CreateBucketRequest {
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "client_from_region")]
    pub client_from_region: String,
    #[serde(rename = "warmup_regions", skip_serializing_if = "Option::is_none")]
    pub warmup_regions: Option<Vec<String>>,
}

impl CreateBucketRequest {
    pub fn new(bucket: String, client_from_region: String) -> CreateBucketRequest {
        CreateBucketRequest {
            bucket,
            client_from_region,
            warmup_regions: None,
        }
    }
}


