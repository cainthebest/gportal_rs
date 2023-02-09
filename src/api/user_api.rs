use std::borrow::Borrow;
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct UserApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> UserApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UserApiClient<C> {
        UserApiClient { configuration }
    }
}

pub trait UserApi {
    fn public_api_login(
        &self,
        body: crate::models::UserLoginRequest,
    ) -> Box<dyn Future<Item = crate::models::UserLoginResponse, Error = Error<serde_json::Value>>>;
    fn public_api_register(
        &self,
        body: crate::models::UserRegisterRequest,
    ) -> Box<dyn Future<Item = crate::models::UserRegisterResponse, Error = Error<serde_json::Value>>>;
    fn public_api_request_password_forgotten_token(
        &self,
        body: crate::models::UserRequestPasswordForgottenTokenRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn public_api_reset_user_password(
        &self,
        body: crate::models::UserResetUserPasswordRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_begin_web_authn_registration(
        &self,
        password: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserBeginWebAuthnRegistrationResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_change_user_password(
        &self,
        body: crate::models::UserChangeUserPasswordRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_confirm_e_mail(
        &self,
        body: crate::models::UserConfirmEMailRequest,
    ) -> Box<dyn Future<Item = crate::models::ApiuserUser, Error = Error<serde_json::Value>>>;
    fn user_api_delete_session(
        &self,
        session_id: &str,
        body: crate::models::UserDeleteSessionRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_delete_web_authn_device(
        &self,
        id: &str,
        body: crate::models::UserDeleteWebAuthnDeviceRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_finish_web_authn_registration(
        &self,
        body: crate::models::UserFinishWebAuthnRegistrationRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserFinishWebAuthnRegistrationResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_user(
        &self,
    ) -> Box<dyn Future<Item = crate::models::UserGetUserResponse, Error = Error<serde_json::Value>>>;
    fn user_api_get_user_compute_limit(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserGetUserComputeLimitResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_list_sessions(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserListSessionsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_resend_confirm_e_mail(
        &self,
        body: serde_json::Value,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_update_user(
        &self,
        body: crate::models::UserUpdateUserRequest,
    ) -> Box<
        dyn Future<Item = crate::models::UserUpdateUserResponse, Error = Error<serde_json::Value>>,
    >;
}

impl<C: hyper::client::Connect> UserApi for UserApiClient<C> {
    fn public_api_login(
        &self,
        body: crate::models::UserLoginRequest,
    ) -> Box<dyn Future<Item = crate::models::UserLoginResponse, Error = Error<serde_json::Value>>>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::Post, "/v1/users/login".to_string())
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

    fn public_api_register(
        &self,
        body: crate::models::UserRegisterRequest,
    ) -> Box<dyn Future<Item = crate::models::UserRegisterResponse, Error = Error<serde_json::Value>>>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::Post, "/v1/users/register".to_string())
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

    fn public_api_request_password_forgotten_token(
        &self,
        body: crate::models::UserRequestPasswordForgottenTokenRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/users/recovery/password/token".to_string(),
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

    fn public_api_reset_user_password(
        &self,
        body: crate::models::UserResetUserPasswordRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/users/recovery/password/reset".to_string(),
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

    fn user_api_begin_web_authn_registration(
        &self,
        password: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserBeginWebAuthnRegistrationResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/profile/webauthn/register".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        if let Some(ref s) = password {
            req = req.with_query_param("password".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn user_api_change_user_password(
        &self,
        body: crate::models::UserChangeUserPasswordRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/profile/password".to_string(),
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

    fn user_api_confirm_e_mail(
        &self,
        body: crate::models::UserConfirmEMailRequest,
    ) -> Box<dyn Future<Item = crate::models::ApiuserUser, Error = Error<serde_json::Value>>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::Post, "/v1/user/confirm".to_string())
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

    fn user_api_delete_session(
        &self,
        session_id: &str,
        body: crate::models::UserDeleteSessionRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/profile/sessions/{sessionId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("sessionId".to_string(), session_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_delete_web_authn_device(
        &self,
        id: &str,
        body: crate::models::UserDeleteWebAuthnDeviceRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/profile/webauthn/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_finish_web_authn_registration(
        &self,
        body: crate::models::UserFinishWebAuthnRegistrationRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserFinishWebAuthnRegistrationResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/profile/webauthn/register".to_string(),
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

    fn user_api_get_user(
        &self,
    ) -> Box<dyn Future<Item = crate::models::UserGetUserResponse, Error = Error<serde_json::Value>>>
    {
        let req = __internal_request::Request::new(hyper::Method::Get, "/v1/user".to_string())
            .with_auth(__internal_request::Auth::ApiKey(
                __internal_request::ApiKey {
                    in_header: true,
                    in_query: false,
                    param_name: "Authorization".to_owned(),
                },
            ));

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_user_compute_limit(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserGetUserComputeLimitResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req =
            __internal_request::Request::new(hyper::Method::Get, "/v1/user/limit".to_string())
                .with_auth(__internal_request::Auth::ApiKey(
                    __internal_request::ApiKey {
                        in_header: true,
                        in_query: false,
                        param_name: "Authorization".to_owned(),
                    },
                ));

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_sessions(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserListSessionsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/profile/sessions".to_string(),
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

    fn user_api_resend_confirm_e_mail(
        &self,
        body: serde_json::Value,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/user/confirm/email".to_string(),
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

    fn user_api_update_user(
        &self,
        body: crate::models::UserUpdateUserRequest,
    ) -> Box<
        dyn Future<Item = crate::models::UserUpdateUserResponse, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/v1/user".to_string())
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
