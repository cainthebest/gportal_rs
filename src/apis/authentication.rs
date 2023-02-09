use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct AuthenticationClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthenticationClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthenticationClient<C> {
        AuthenticationClient { configuration }
    }
}

pub trait Authentication {
    fn public_api_list_jwt_public_keys(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::JwtListJwtPublicKeysResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn public_api_refresh_tokens(
        &self,
        body: crate::models::UserRefreshTokensRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserRefreshTokensResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_create_long_lived_token(
        &self,
    ) -> Box<dyn Future<Item = crate::models::UserLongLivedToken, Error = Error<serde_json::Value>>>;
    fn user_api_list_long_lived_tokens(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserListLongLivedTokensResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_revoke_long_lived_token(
        &self,
        token_id: &str,
        body: crate::models::UserRevokeLongLivedTokenRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> Authentication for AuthenticationClient<C> {
    fn public_api_list_jwt_public_keys(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::JwtListJwtPublicKeysResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(hyper::Method::Get, "/v1/jwt_keys".to_string())
            .with_auth(__internal_request::Auth::ApiKey(
                __internal_request::ApiKey {
                    in_header: true,
                    in_query: false,
                    param_name: "Authorization".to_owned(),
                },
            ));

        req.execute(self.configuration.borrow())
    }

    fn public_api_refresh_tokens(
        &self,
        body: crate::models::UserRefreshTokensRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserRefreshTokensResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req =
            __internal_request::Request::new(hyper::Method::Post, "/v1/token/refresh".to_string())
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

    fn user_api_create_long_lived_token(
        &self,
    ) -> Box<dyn Future<Item = crate::models::UserLongLivedToken, Error = Error<serde_json::Value>>>
    {
        let req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/profile/long_life_tokens".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_long_lived_tokens(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserListLongLivedTokensResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/profile/long_life_tokens".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));

        req.execute(self.configuration.borrow())
    }

    fn user_api_revoke_long_lived_token(
        &self,
        token_id: &str,
        body: crate::models::UserRevokeLongLivedTokenRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/profile/long_life_tokens/{tokenId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("tokenId".to_string(), token_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }
}
