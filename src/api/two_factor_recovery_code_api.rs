use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct TwoFactorRecoveryCodeApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> TwoFactorRecoveryCodeApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> TwoFactorRecoveryCodeApiClient<C> {
        TwoFactorRecoveryCodeApiClient { configuration }
    }
}

pub trait TwoFactorRecoveryCodeApi {
    fn user_api_regenerate_recovery_codes(
        &self,
        body: crate::models::UserRegenerateRecoveryCodesRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserRegenerateRecoveryCodesResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::connect::Connect> TwoFactorRecoveryCodeApi for TwoFactorRecoveryCodeApiClient<C> {
    fn user_api_regenerate_recovery_codes(
        &self,
        body: crate::models::UserRegenerateRecoveryCodesRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserRegenerateRecoveryCodesResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/profile/recovery_codes".to_string(),
        )
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
