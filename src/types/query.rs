//! Types related to query API calls

use serde::{Deserialize, Serialize};

use super::{enums::object_type::IcingaObjectType, metadata::IcingaMetadata};

/// wrapper for Json results
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsWrapper<T> {
    /// the internal field in the Icinga2 object containing all an array of the actual results
    pub results: Vec<T>,
}

/// the result of an icinga query to a type with joins
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResultObjectWithJoins<Obj, ObjJoins> {
    /// dependency attributes
    pub attrs: Obj,
    /// joins
    pub joins: ObjJoins,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be the one matching Obj
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// the result of an icinga query to a type without joins
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResultObject<Obj> {
    /// dependency attributes
    pub attrs: Obj,
    /// metadata, only contains data if the parameter meta contains one or more values
    pub meta: IcingaMetadata,
    /// object name
    pub name: String,
    /// type of icinga object, should always be the one matching Obj
    #[serde(rename = "type")]
    pub object_type: IcingaObjectType,
}

/// implement a query REST API Endpoint for the given Icinga type with join support
macro_rules! query_with_joins {
    ($name:ident, $builder_name:ident, $object_category:path, $path_component:path, $return_type:ty, $join_types:ty, $join_return_type:ty, $object_type:expr, $url_fragment:expr) => {
        use std::collections::BTreeMap;

#[rustfmt::skip]
        use crate::types::{
            enums::object_type::IcingaObjectType,
            filter::IcingaFilter,
            join_types::{
                add_joins_to_url,
                $path_component::{$join_return_type, $join_types},
                IcingaJoins,
            },
            metadata::{add_meta_to_url, IcingaMetadataType},
            query::{QueryResultObject, QueryResultObjectWithJoins, ResultsWrapper},
            rest::{RestApiEndpoint, RestApiResponse},
            $object_category::{
                $path_component::{
                    $return_type,
                }
            }
        };

        /// query for Icinga objects of this type
        #[derive(Debug, Clone, derive_builder::Builder)]
        #[builder(
            build_fn(error = "crate::error::Error", validate = "Self::validate"),
            derive(Debug)
        )]
        pub struct $name<'a> {
            /// the joins (related objects) to return along with each result
            #[builder(default, setter(strip_option, into))]
            joins: Option<IcingaJoins<'a, $join_types>>,
            /// the metadata to return along with each result
            #[builder(default, setter(strip_option, into))]
            meta: Option<Vec<IcingaMetadataType>>,
            /// filter the results
            #[builder(default, setter(strip_option, into))]
            filter: Option<IcingaFilter>,
        }

        impl<'a> $name<'a> {
            /// create a new builder for this endpoint
            ///
            /// this is usually the first step to calling this REST API endpoint
            #[must_use]
            pub fn builder() -> $builder_name<'a> {
                $builder_name::default()
            }
        }

        impl<'a> $builder_name<'a> {
            /// makes sure the filter object type is the correct one for the type of return values this endpoint returns
            ///
            /// # Errors
            ///
            /// this returns an error if the filter field object type does not match the return type of the API call
            pub fn validate(&self) -> Result<(), crate::error::Error> {
                let expected = $object_type;
                if let Some(Some(filter)) = &self.filter {
                    if filter.object_type != expected {
                        Err(crate::error::Error::FilterObjectTypeMismatch(
                            vec![expected],
                            filter.object_type.to_owned(),
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
        }

        impl<'a> RestApiEndpoint for $name<'a> {
            type RequestBody = IcingaFilter;

            fn method(&self) -> Result<http::Method, crate::error::Error> {
                Ok(http::Method::GET)
            }

            fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
                let mut url = base_url
                    .join($url_fragment)
                    .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
                if let Some(joins) = &self.joins {
                    add_joins_to_url(&mut url, &joins)?;
                }
                if let Some(meta) = &self.meta {
                    add_meta_to_url(&mut url, &meta)?;
                }
                Ok(url)
            }

            fn request_body(
                &self,
            ) -> Result<Option<std::borrow::Cow<Self::RequestBody>>, crate::error::Error>
            where
                Self::RequestBody: Clone + serde::Serialize + std::fmt::Debug,
            {
                Ok(self.filter.as_ref().map(|f| std::borrow::Cow::Borrowed(f)))
            }
        }

        impl<'a> RestApiResponse<$name<'a>> for ResultsWrapper<QueryResultObject<$return_type>> {}

        impl<'a> RestApiResponse<$name<'a>>
            for ResultsWrapper<QueryResultObject<BTreeMap<String, serde_json::Value>>>
        {
        }

        impl<'a> RestApiResponse<$name<'a>>
            for ResultsWrapper<QueryResultObjectWithJoins<$return_type, $join_return_type>>
        {
        }

        impl<'a> RestApiResponse<$name<'a>>
            for ResultsWrapper<
                QueryResultObjectWithJoins<BTreeMap<String, serde_json::Value>, $join_return_type>,
            >
        {
        }

        impl<'a> RestApiResponse<$name<'a>>
            for ResultsWrapper<
                QueryResultObjectWithJoins<$return_type, BTreeMap<String, serde_json::Value>>,
            >
        {
        }

        impl<'a> RestApiResponse<$name<'a>>
            for ResultsWrapper<
                QueryResultObjectWithJoins<
                    BTreeMap<String, serde_json::Value>,
                    BTreeMap<String, serde_json::Value>,
                >,
            >
        {
        }
    };
}
pub(crate) use query_with_joins;

/// implement a query REST API Endpoint for the given Icinga type without join support
macro_rules! query {
    ($name:ident, $builder_name:ident, $object_category:path, $path_component:path, $return_type:ty, $object_type:expr, $url_fragment:expr) => {
        use std::collections::BTreeMap;

#[rustfmt::skip]
        use crate::types::{
            enums::object_type::IcingaObjectType,
            filter::IcingaFilter,
            metadata::{add_meta_to_url, IcingaMetadataType},
            query::{QueryResultObject, ResultsWrapper},
            rest::{RestApiEndpoint, RestApiResponse},
            $object_category::{
                $path_component::{
                    $return_type,
                },
            },
        };

        /// query for Icinga objects of this type
        #[derive(Debug, Clone, derive_builder::Builder)]
        #[builder(
            build_fn(error = "crate::error::Error", validate = "Self::validate"),
            derive(Debug)
        )]
        pub struct $name {
            /// the metadata to return along with each result
            #[builder(default, setter(strip_option, into))]
            meta: Option<Vec<IcingaMetadataType>>,
            /// filter the results
            #[builder(default, setter(strip_option, into))]
            filter: Option<IcingaFilter>,
        }

        impl $name {
            /// create a new builder for this endpoint
            ///
            /// this is usually the first step to calling this REST API endpoint
            #[must_use]
            pub fn builder() -> $builder_name {
                $builder_name::default()
            }
        }

        impl $builder_name {
            /// makes sure the filter object type is the correct one for the type of return values this endpoint returns
            ///
            /// # Errors
            ///
            /// this returns an error if the filter field object type does not match the return type of the API call
            pub fn validate(&self) -> Result<(), crate::error::Error> {
                let expected = $object_type;
                if let Some(Some(filter)) = &self.filter {
                    if filter.object_type != expected {
                        Err(crate::error::Error::FilterObjectTypeMismatch(
                            vec![expected],
                            filter.object_type.to_owned(),
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
        }

        impl RestApiEndpoint for $name {
            type RequestBody = IcingaFilter;

            fn method(&self) -> Result<http::Method, crate::error::Error> {
                Ok(http::Method::GET)
            }

            fn url(&self, base_url: &url::Url) -> Result<url::Url, crate::error::Error> {
                let mut url = base_url
                    .join($url_fragment)
                    .map_err(crate::error::Error::CouldNotParseUrlFragment)?;
                if let Some(meta) = &self.meta {
                    add_meta_to_url(&mut url, &meta)?;
                }
                Ok(url)
            }

            fn request_body(
                &self,
            ) -> Result<Option<std::borrow::Cow<Self::RequestBody>>, crate::error::Error>
            where
                Self::RequestBody: Clone + serde::Serialize + std::fmt::Debug,
            {
                Ok(self.filter.as_ref().map(|f| std::borrow::Cow::Borrowed(f)))
            }
        }

        impl RestApiResponse<$name> for ResultsWrapper<QueryResultObject<$return_type>> {}

        impl RestApiResponse<$name>
            for ResultsWrapper<QueryResultObject<BTreeMap<String, serde_json::Value>>>
        {
        }
    };
}
pub(crate) use query;
