use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct FlavourApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> FlavourApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FlavourApiClient<C> {
        FlavourApiClient { configuration }
    }
}

pub trait FlavourApi {
    fn user_api_get_project_flavours(
        &self,
        project_id: &str,
        datacenter_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::FlavourGetProjectFlavoursResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::connect::Connect> FlavourApi for FlavourApiClient<C> {
    fn user_api_get_project_flavours(
        &self,
        project_id: &str,
        datacenter_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::FlavourGetProjectFlavoursResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/flavours/{datacenterId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("datacenterId".to_string(), datacenter_id.to_string());

        req.execute(self.configuration.borrow())
    }
}
