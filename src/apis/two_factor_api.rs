use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct TwoFactorApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TwoFactorApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TwoFactorApiClient<C> {
        TwoFactorApiClient { configuration }
    }
}

pub trait TwoFactorApi {
    fn user_api_get_two_factor_methods(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserGetTwoFactorMethodsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> TwoFactorApi for TwoFactorApiClient<C> {
    fn user_api_get_two_factor_methods(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserGetTwoFactorMethodsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req =
            __internal_request::Request::new(hyper::Method::Get, "/v1/profile/2fa".to_string())
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
