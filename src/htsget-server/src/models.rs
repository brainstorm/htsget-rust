#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponses {
}

impl ErrorResponses {
    pub fn new() -> ErrorResponses {
        ErrorResponses {
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtsgetResponse {
    #[serde(rename = "htsget")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub htsget: Option<models::HtsgetResponseHtsget>,

}

impl HtsgetResponse {
    pub fn new() -> HtsgetResponse {
        HtsgetResponse {
            htsget: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtsgetResponseHtsget {
    #[serde(rename = "format")]
    pub format: String,

    #[serde(rename = "urls")]
    pub urls: Vec<models::UrlsItems>,

    #[serde(rename = "md5")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub md5: Option<String>,

}

impl HtsgetResponseHtsget {
    pub fn new(format: String, urls: Vec<models::UrlsItems>, ) -> HtsgetResponseHtsget {
        HtsgetResponseHtsget {
            format: format,
            urls: urls,
            md5: None,
        }
    }
}

/// The request parameters do not adhere to the specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidInput {
}

impl InvalidInput {
    pub fn new() -> InvalidInput {
        InvalidInput {
        }
    }
}

/// The requested range cannot be satisfied
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidRange {
}

impl InvalidRange {
    pub fn new() -> InvalidRange {
        InvalidRange {
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadIdResponse {
}

impl ReadIdResponse {
    pub fn new() -> ReadIdResponse {
        ReadIdResponse {
        }
    }
}

/// The requested file format is not supported by the server
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsupportedFormat {
}

impl UnsupportedFormat {
    pub fn new() -> UnsupportedFormat {
        UnsupportedFormat {
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlsItems {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "headers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub headers: Option<Object>,

}

impl UrlsItems {
    pub fn new(url: String, ) -> UrlsItems {
        UrlsItems {
            url: url,
            headers: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantIdResponse {
}

impl VariantIdResponse {
    pub fn new() -> VariantIdResponse {
        VariantIdResponse {
        }
    }
}
