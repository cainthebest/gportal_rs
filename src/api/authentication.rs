use std::{borrow::Borrow, rc::Rc};

use futures::Future;
use hyper::client::connect::Connect;
use serde_json::Value as JsonValue;

use super::{configuration::Configuration, request as __internal_request, Error};
use crate::models::{
    JwtListJwtPublicKeysResponse, UserListLongLivedTokensResponse, UserLongLivedToken,
    UserRefreshTokensRequest, UserRefreshTokensResponse, UserRevokeLongLivedTokenRequest,
};

pub struct AuthenticationClient<C: Connect> {
    configuration: Rc<Configuration<C>>,
}

impl<C: Connect> AuthenticationClient<C> {
    pub fn new(configuration: Rc<Configuration<C>>) -> AuthenticationClient<C> {
        AuthenticationClient { configuration }
    }
}

pub trait Authentication {
    fn list_jwt_public_keys(
        &self,
    ) -> Box<dyn Future<Item = JwtListJwtPublicKeysResponse, Error = Error<JsonValue>>>;

    fn refresh_tokens(
        &self,
        body: UserRefreshTokensRequest,
    ) -> Box<dyn Future<Item = UserRefreshTokensResponse, Error = Error<JsonValue>>>;

    fn create_long_lived_token(
        &self,
    ) -> Box<dyn Future<Item = UserLongLivedToken, Error = Error<JsonValue>>>;

    fn list_long_lived_tokens(
        &self,
    ) -> Box<dyn Future<Item = UserListLongLivedTokensResponse, Error = Error<JsonValue>>>;

    fn revoke_long_lived_token(
        &self,
        token_id: &str,
        body: UserRevokeLongLivedTokenRequest,
    ) -> Box<dyn Future<Item = JsonValue, Error = Error<JsonValue>>>;
}

impl<C: Connect> Authentication for AuthenticationClient<C> {
    fn list_jwt_public_keys(
        &self,
    ) -> Box<dyn Future<Item = JwtListJwtPublicKeysResponse, Error = Error<JsonValue>>> {
        let req = __internal_request::Request::new(hyper::Method::GET, "/v1/jwt_keys".to_string())
            .with_auth(__internal_request::Auth::ApiKey(
                __internal_request::ApiKey {
                    in_header: true,
                    in_query: false,
                    param_name: "Authorization".to_owned(),
                },
            ));

        req.execute(self.configuration.borrow())
    }

    fn refresh_tokens(
        &self,
        body: UserRefreshTokensRequest,
    ) -> Box<dyn Future<Item = UserRefreshTokensResponse, Error = Error<JsonValue>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/v1/token/refresh".to_string())
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

    fn create_long_lived_token(
        &self,
    ) -> Box<dyn Future<Item = UserLongLivedToken, Error = Error<JsonValue>>> {
        let req = __internal_request::Request::new(
            hyper::Method::POST,
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

    fn list_long_lived_tokens(
        &self,
    ) -> Box<dyn Future<Item = UserListLongLivedTokensResponse, Error = Error<JsonValue>>> {
        let req = __internal_request::Request::new(
            hyper::Method::GET,
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

    fn revoke_long_lived_token(
        &self,
        token_id: &str,
        body: UserRevokeLongLivedTokenRequest,
    ) -> Box<dyn Future<Item = JsonValue, Error = Error<JsonValue>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
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
