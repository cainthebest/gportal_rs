use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct NetworkApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NetworkApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NetworkApiClient<C> {
        NetworkApiClient { configuration }
    }
}

pub trait NetworkApi {
    fn user_api_create_project_network_subnet(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkCreateProjectNetworkSubnetRequest,
    ) -> Box<dyn Future<Item = crate::models::NetworkSubnet, Error = Error<serde_json::Value>>>;
    fn user_api_delete_project_network_subnet(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkDeleteProjectNetworkSubnetRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_get_project_network(
        &self,
        project_id: &str,
        id: &str,
    ) -> Box<dyn Future<Item = crate::models::NetworkNetwork, Error = Error<serde_json::Value>>>;
    fn user_api_list_project_networks(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::NetworkListProjectNetworksResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_update_project_network(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkUpdateProjectNetworkRequest,
    ) -> Box<dyn Future<Item = crate::models::NetworkNetwork, Error = Error<serde_json::Value>>>;
    fn user_api_update_project_network_subnet(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkUpdateProjectNetworkSubnetRequest,
    ) -> Box<dyn Future<Item = crate::models::NetworkSubnet, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> NetworkApi for NetworkApiClient<C> {
    fn user_api_create_project_network_subnet(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkCreateProjectNetworkSubnetRequest,
    ) -> Box<dyn Future<Item = crate::models::NetworkSubnet, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/networks/{id}/subnet".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_delete_project_network_subnet(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkDeleteProjectNetworkSubnetRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/projects/{projectId}/subnet/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_project_network(
        &self,
        project_id: &str,
        id: &str,
    ) -> Box<dyn Future<Item = crate::models::NetworkNetwork, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/networks/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_project_networks(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::NetworkListProjectNetworksResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/networks".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_update_project_network(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkUpdateProjectNetworkRequest,
    ) -> Box<dyn Future<Item = crate::models::NetworkNetwork, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/v1/projects/{projectId}/networks/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_update_project_network_subnet(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::NetworkUpdateProjectNetworkSubnetRequest,
    ) -> Box<dyn Future<Item = crate::models::NetworkSubnet, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/v1/projects/{projectId}/subnet/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }
}
