use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct CountryApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> CountryApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> CountryApiClient<C> {
        CountryApiClient { configuration }
    }
}

pub trait CountryApi {
    fn user_api_list_countries(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentListCountriesResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> CountryApi for CountryApiClient<C> {
    fn user_api_list_countries(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentListCountriesResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(hyper::Method::Get, "/v1/countries".to_string())
            .with_auth(__internal_request::Auth::ApiKey(
                __internal_request::ApiKey {
                    in_header: true,
                    in_query: false,
                    param_name: "Authorization".to_owned(),
                },
            ));

        req.execute(self.configuration.borrow())
    }
}
