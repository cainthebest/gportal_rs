use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct SSHKeyApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SSHKeyApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SSHKeyApiClient<C> {
        SSHKeyApiClient { configuration }
    }
}

pub trait SSHKeyApi {
    fn user_api_create_user_ssh_key(
        &self,
        body: crate::models::UserCreateUserSshKeyRequest,
    ) -> Box<dyn Future<Item = crate::models::SecuritySshKey, Error = Error<serde_json::Value>>>;
    fn user_api_delete_user_ssh_key(
        &self,
        ssh_key_id: &str,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_list_user_ssh_keys(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserListUserSshKeysResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> SSHKeyApi for SSHKeyApiClient<C> {
    fn user_api_create_user_ssh_key(
        &self,
        body: crate::models::UserCreateUserSshKeyRequest,
    ) -> Box<dyn Future<Item = crate::models::SecuritySshKey, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/profile/ssh_keys".to_string(),
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

    fn user_api_delete_user_ssh_key(
        &self,
        ssh_key_id: &str,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/profile/ssh_keys/{sshKeyId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("sshKeyId".to_string(), ssh_key_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_user_ssh_keys(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::UserListUserSshKeysResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/profile/ssh_keys".to_string(),
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
}
