/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for SearchReadId
    lazy_static! {
        pub static ref SEARCH_READ_ID_RESULTS_MATCHING_CRITERIA: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SearchReadId
    lazy_static! {
        pub static ref SEARCH_READ_ID_SOMETHING_WENT_WRONG: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SearchVariantId
    lazy_static! {
        pub static ref SEARCH_VARIANT_ID_RESULTS_MATCHING_CRITERIA: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for SearchVariantId
    lazy_static! {
        pub static ref SEARCH_VARIANT_ID_SOMETHING_WENT_WRONG: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
