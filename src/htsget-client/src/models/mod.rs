mod error_responses;
pub use self::error_responses::ErrorResponses;
mod htsget_response;
pub use self::htsget_response::HtsgetResponse;
mod htsget_response_htsget;
pub use self::htsget_response_htsget::HtsgetResponseHtsget;
mod invalid_input;
pub use self::invalid_input::InvalidInput;
mod invalid_range;
pub use self::invalid_range::InvalidRange;
mod read_id_response;
pub use self::read_id_response::ReadIdResponse;
mod unsupported_format;
pub use self::unsupported_format::UnsupportedFormat;
mod urls_items;
pub use self::urls_items::UrlsItems;
mod variant_id_response;
pub use self::variant_id_response::VariantIdResponse;

// TODO(farcaller): sort out files
pub struct File;
