#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate uuid;
extern crate chrono;
extern crate percent_encoding;
extern crate url;


use std::sync::Arc;
use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, XSpanId, XSpanIdString, Has, RequestParser};
use swagger::auth::Scopes;

use {Api,
     SearchReadIdResponse,
     SearchVariantIdResponse
     };
#[allow(unused_imports)]
use models;

pub mod context;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&[
            r"^/brainkod/htsget/1.1.1/reads/(?P<id>[^/?#]*)$",
            r"^/brainkod/htsget/1.1.1/variants/(?P<id>[^/?#]*)$"
        ]).unwrap();
    }
    pub static ID_READS_ID: usize = 0;
    lazy_static! {
        pub static ref REGEX_READS_ID: regex::Regex = regex::Regex::new(r"^/brainkod/htsget/1.1.1/reads/(?P<id>[^/?#]*)$").unwrap();
    }
    pub static ID_VARIANTS_ID: usize = 1;
    lazy_static! {
        pub static ref REGEX_VARIANTS_ID: regex::Regex = regex::Regex::new(r"^/brainkod/htsget/1.1.1/variants/(?P<id>[^/?#]*)$").unwrap();
    }
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        // This match statement is duplicated below in `parse_operation_id()`.
        // Please update both places if changing how this code is autogenerated.
        match &method {

            // SearchReadId - GET /reads/{id}
            &hyper::Method::Get if path.matched(paths::ID_READS_ID) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "read:genomic_reads".to_string(), // read access to genomic reads
                            "read:genomic_variants".to_string(), // read access to genomic variants
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_READS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE READS_ID in set but failed match against \"{}\"", path, paths::REGEX_READS_ID.as_str())
                    );

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<String>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter id: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"]))))
                };



                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_format = query_params.iter().filter(|e| e.0 == "format").map(|e| e.1.to_owned())

                    .nth(0);

                let param_format = param_format.and_then(|param_format| param_format.parse::<>().ok());
                let param_reference_name = query_params.iter().filter(|e| e.0 == "referenceName").map(|e| e.1.to_owned())

                    .nth(0);

                let param_reference_name = param_reference_name.and_then(|param_reference_name| param_reference_name.parse::<>().ok());
                let param_start = query_params.iter().filter(|e| e.0 == "start").map(|e| e.1.to_owned())

                    .nth(0);

                let param_start = param_start.and_then(|param_start| param_start.parse::<>().ok());
                let param_end = query_params.iter().filter(|e| e.0 == "end").map(|e| e.1.to_owned())

                    .nth(0);

                let param_end = param_end.and_then(|param_end| param_end.parse::<>().ok());
                let param_fields = query_params.iter().filter(|e| e.0 == "fields").map(|e| e.1.to_owned())

                    .nth(0);

                let param_fields = param_fields.and_then(|param_fields| param_fields.parse::<>().ok());
                let param_tags = query_params.iter().filter(|e| e.0 == "tags").map(|e| e.1.to_owned())

                    .nth(0);

                let param_tags = param_tags.and_then(|param_tags| param_tags.parse::<>().ok());
                let param_notags = query_params.iter().filter(|e| e.0 == "notags").map(|e| e.1.to_owned())

                    .nth(0);

                let param_notags = param_notags.and_then(|param_notags| param_notags.parse::<>().ok());



                Box::new({
                        {{

                                Box::new(api_impl.search_read_id(param_id, param_format, param_reference_name, param_start, param_end, param_fields, param_tags, param_notags, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SearchReadIdResponse::ResultsMatchingCriteria

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SEARCH_READ_ID_RESULTS_MATCHING_CRITERIA.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SearchReadIdResponse::SomethingWentWrong

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SEARCH_READ_ID_SOMETHING_WENT_WRONG.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SearchReadIdResponse::UnexpectedError


                                                => {
                                                    response.set_status(StatusCode::try_from(500).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            // SearchVariantId - GET /variants/{id}
            &hyper::Method::Get if path.matched(paths::ID_VARIANTS_ID) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "read:genomic_reads".to_string(), // read access to genomic reads
                            "read:genomic_variants".to_string(), // read access to genomic variants
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_VARIANTS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE VARIANTS_ID in set but failed match against \"{}\"", path, paths::REGEX_VARIANTS_ID.as_str())
                    );

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<String>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter id: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"]))))
                };



                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_format = query_params.iter().filter(|e| e.0 == "format").map(|e| e.1.to_owned())

                    .nth(0);

                let param_format = param_format.and_then(|param_format| param_format.parse::<>().ok());
                let param_reference_name = query_params.iter().filter(|e| e.0 == "referenceName").map(|e| e.1.to_owned())

                    .nth(0);

                let param_reference_name = param_reference_name.and_then(|param_reference_name| param_reference_name.parse::<>().ok());
                let param_start = query_params.iter().filter(|e| e.0 == "start").map(|e| e.1.to_owned())

                    .nth(0);

                let param_start = param_start.and_then(|param_start| param_start.parse::<>().ok());
                let param_end = query_params.iter().filter(|e| e.0 == "end").map(|e| e.1.to_owned())

                    .nth(0);

                let param_end = param_end.and_then(|param_end| param_end.parse::<>().ok());
                let param_fields = query_params.iter().filter(|e| e.0 == "fields").map(|e| e.1.to_owned())

                    .nth(0);

                let param_fields = param_fields.and_then(|param_fields| param_fields.parse::<>().ok());
                let param_tags = query_params.iter().filter(|e| e.0 == "tags").map(|e| e.1.to_owned())

                    .nth(0);

                let param_tags = param_tags.and_then(|param_tags| param_tags.parse::<>().ok());
                let param_notags = query_params.iter().filter(|e| e.0 == "notags").map(|e| e.1.to_owned())

                    .nth(0);

                let param_notags = param_notags.and_then(|param_notags| param_notags.parse::<>().ok());



                Box::new({
                        {{

                                Box::new(api_impl.search_variant_id(param_id, param_format, param_reference_name, param_start, param_end, param_fields, param_tags, param_notags, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SearchVariantIdResponse::ResultsMatchingCriteria

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SEARCH_VARIANT_ID_RESULTS_MATCHING_CRITERIA.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SearchVariantIdResponse::SomethingWentWrong

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::SEARCH_VARIANT_ID_SOMETHING_WENT_WRONG.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                SearchVariantIdResponse::UnexpectedError


                                                => {
                                                    response.set_status(StatusCode::try_from(500).unwrap());

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

impl<T, C> Clone for Service<T, C>
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl RequestParser for ApiRequestParser {
    fn parse_operation_id(request: &Request) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {

            // SearchReadId - GET /reads/{id}
            &hyper::Method::Get if path.matched(paths::ID_READS_ID) => Ok("SearchReadId"),

            // SearchVariantId - GET /variants/{id}
            &hyper::Method::Get if path.matched(paths::ID_VARIANTS_ID) => Ok("SearchVariantId"),
            _ => Err(()),
        }
    }
}
