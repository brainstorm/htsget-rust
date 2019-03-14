//! Server implementation of openapi_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use std::collections::HashMap;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use openapi_client::{Api, ApiError,
                      SearchReadIdResponse,
                      SearchVariantIdResponse
};
use openapi_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{

    /// Gets the reads from a pre-indexed id
    fn search_read_id(&self, id: String, format: Option<String>, reference_name: Option<String>, start: Option<i64>, end: Option<i64>, fields: Option<String>, tags: Option<String>, notags: Option<String>, context: &C) -> Box<Future<Item=SearchReadIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("search_read_id(\"{}\", {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", id, format, reference_name, start, end, fields, tags, notags, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Gets the variants from a pre-indexed id
    fn search_variant_id(&self, id: String, format: Option<String>, reference_name: Option<String>, start: Option<i64>, end: Option<i64>, fields: Option<String>, tags: Option<String>, notags: Option<String>, context: &C) -> Box<Future<Item=SearchVariantIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("search_variant_id(\"{}\", {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", id, format, reference_name, start, end, fields, tags, notags, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
