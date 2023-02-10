use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct TwoFactorTOTPApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> TwoFactorTOTPApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TwoFactorTOTPApiClient<C> {
        TwoFactorTOTPApiClient { configuration }
    }
}

pub trait TwoFactorTOTPApi {
    fn user_api_add_totp(
        &self,
        body: crate::models::UserAddTotpRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_create_totp(
        &self,
    ) -> Box<
        dyn Future<Item = crate::models::UserCreateTotpResponse, Error = Error<serde_json::Value>>,
    >;
    fn user_api_remove_totp(
        &self,
        body: crate::models::UserRemoveTotpRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::connect::Connect> TwoFactorTOTPApi for TwoFactorTOTPApiClient<C> {
    fn user_api_add_totp(
        &self,
        body: crate::models::UserAddTotpRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/v1/profile/totp".to_string())
                .with_auth(__internal_request::Auth::ApiKey(
                    __internal_request::ApiKey {
                        in_header: true,
                        in_query: false,
                        param_name: "Authorization".to_owned(),
                    },
                ));
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_create_totp(
        &self,
    ) -> Box<
        dyn Future<Item = crate::models::UserCreateTotpResponse, Error = Error<serde_json::Value>>,
    > {
        let req =
            __internal_request::Request::new(hyper::Method::GET, "/v1/profile/totp".to_string())
                .with_auth(__internal_request::Auth::ApiKey(
                    __internal_request::ApiKey {
                        in_header: true,
                        in_query: false,
                        param_name: "Authorization".to_owned(),
                    },
                ));

        req.execute(self.configuration.borrow())
    }

    fn user_api_remove_totp(
        &self,
        body: crate::models::UserRemoveTotpRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::DELETE, "/v1/profile/totp".to_string())
                .with_auth(__internal_request::Auth::ApiKey(
                    __internal_request::ApiKey {
                        in_header: true,
                        in_query: false,
                        param_name: "Authorization".to_owned(),
                    },
                ));
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }
}
