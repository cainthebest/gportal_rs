use std::borrow::Borrow;
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct ComputeApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ComputeApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ComputeApiClient<C> {
        ComputeApiClient { configuration }
    }
}

pub trait ComputeApi {
    fn user_api_change_compute_resource_billing_period(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeChangeComputeResourceBillingPeriodRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    >;
    fn user_api_change_compute_resource_renew_state(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeChangeComputeResourceRenewStateRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    >;
    fn user_api_change_compute_resource_rescue_mode(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeComputeResourceRescueModeRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_create_compute_resource(
        &self,
        project_id: &str,
        body: crate::models::ComputeCreateComputeResourceRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeCreateComputeResourceResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_destroy_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeDestroyComputeResourceRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_get_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    >;
    fn user_api_get_compute_resource_console(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeGetComputeResourceConsoleResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_compute_resource_pricing(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeGetComputeResourcePricingResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_compute_resource_traffic(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeGetComputeResourceTrafficResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_list_compute_resources(
        &self,
        project_id: &str,
        page: Option<i32>,
        search: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeListComputeResourcesResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_power_action_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputePowerActionComputeResourceRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_reinstall_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeReinstallComputeResourceRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    >;
    fn user_api_update_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeUpdateComputeResourceRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    >;
}

impl<C: hyper::client::connect::Connect> ComputeApi for ComputeApiClient<C> {
    fn user_api_change_compute_resource_billing_period(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeChangeComputeResourceBillingPeriodRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/compute/{resourceId}/billing_period".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_change_compute_resource_renew_state(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeChangeComputeResourceRenewStateRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/compute/{resourceId}/renew".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_change_compute_resource_rescue_mode(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeComputeResourceRescueModeRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/compute/{resourceId}/rescue".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_create_compute_resource(
        &self,
        project_id: &str,
        body: crate::models::ComputeCreateComputeResourceRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeCreateComputeResourceResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/compute".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_destroy_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeDestroyComputeResourceRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/v1/projects/{projectId}/compute/{resourceId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/compute/{resourceId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_compute_resource_console(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeGetComputeResourceConsoleResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/compute/{resourceId}/console".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_compute_resource_pricing(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeGetComputeResourcePricingResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/compute/{resourceId}/pricing".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_compute_resource_traffic(
        &self,
        project_id: &str,
        resource_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeGetComputeResourceTrafficResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/compute/{resourceId}/traffic".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_compute_resources(
        &self,
        project_id: &str,
        page: Option<i32>,
        search: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::ComputeListComputeResourcesResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/compute".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = search {
            req = req.with_query_param("search".to_string(), s.to_string());
        }
        req = req.with_path_param("projectId".to_string(), project_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_power_action_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputePowerActionComputeResourceRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/compute/{resourceId}/power".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_reinstall_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeReinstallComputeResourceRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/compute/{resourceId}/reinstall".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_update_compute_resource(
        &self,
        project_id: &str,
        resource_id: &str,
        body: crate::models::ComputeUpdateComputeResourceRequest,
    ) -> Box<
        dyn Future<Item = crate::models::ComputeComputeResource, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/v1/projects/{projectId}/compute/{resourceId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("resourceId".to_string(), resource_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }
}
