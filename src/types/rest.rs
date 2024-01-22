//! Rest API request related types
//!
//! (as opposed to the streaming event long-polling API parts)

/// a trait for objects describing a REST API endpoint
///
/// this is implemented by types which contain all the necessary information
/// for a request.
pub trait RestApiEndpoint {
    /// the type of the request body
    type RequestBody;

    /// returns the HTTP method to use
    ///
    /// since this is Icinga this is the method passed to X-HTTP-Method-Override
    ///
    /// the actual HTTP method will always be POST if there is a request body
    ///
    /// # Errors
    ///
    /// this should return an error if something went wrong in determining the request method
    fn method(&self) -> Result<reqwest::Method, crate::error::Error>;

    /// returns the URL to use for the request based on the base URL passed in
    /// as a parameter
    ///
    /// # Errors
    ///
    /// this should return an error if something went wrong in determining the URL (e.g. parse error on the fragment joined to the base URL)
    fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error>;

    /// the request body which must be a JSON serializable type
    ///
    /// since it is always JSON we do not need to return a Content-Type
    ///
    /// # Errors
    ///
    /// this should return an error if something went wrong in determining the request body
    fn request_body(
        &self,
    ) -> Result<Option<std::borrow::Cow<Self::RequestBody>>, crate::error::Error>
    where
        Self::RequestBody: Clone + serde::Serialize + std::fmt::Debug;
}

/// this is a marker trait that marks a type as a valid response type for a
/// given RestApiEndpoint
pub trait RestApiResponse<ApiEndpoint> {}
