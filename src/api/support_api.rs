use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct SupportApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SupportApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SupportApiClient<C> {
        SupportApiClient { configuration }
    }
}

pub trait SupportApi {
    fn user_api_add_project_support_ticket_comment(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::SupportAddProjectSupportTicketCommentRequest,
    ) -> Box<dyn Future<Item = crate::models::SupportSupportTicket, Error = Error<serde_json::Value>>>;
    fn user_api_change_project_support_package(
        &self,
        project_id: &str,
        plan: &str,
        body: crate::models::SupportChangeProjectSupportPackageRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_close_project_support_ticket(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::SupportCloseProjectSupportTicketRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_create_project_support_ticket(
        &self,
        project_id: &str,
        body: crate::models::SupportCreateProjectSupportTicketRequest,
    ) -> Box<dyn Future<Item = crate::models::SupportSupportTicket, Error = Error<serde_json::Value>>>;
    fn user_api_get_project_support_ticket(
        &self,
        project_id: &str,
        id: &str,
    ) -> Box<dyn Future<Item = crate::models::SupportSupportTicket, Error = Error<serde_json::Value>>>;
    fn user_api_list_project_support_packages(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::SupportListProjectSupportPackagesResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_list_project_support_tickets(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::SupportListProjectSupportTicketsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> SupportApi for SupportApiClient<C> {
    fn user_api_add_project_support_ticket_comment(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::SupportAddProjectSupportTicketCommentRequest,
    ) -> Box<dyn Future<Item = crate::models::SupportSupportTicket, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/v1/projects/{projectId}/support/tickets/{id}".to_string(),
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

    fn user_api_change_project_support_package(
        &self,
        project_id: &str,
        plan: &str,
        body: crate::models::SupportChangeProjectSupportPackageRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/support/plans/{plan}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("plan".to_string(), plan.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_close_project_support_ticket(
        &self,
        project_id: &str,
        id: &str,
        body: crate::models::SupportCloseProjectSupportTicketRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/v1/projects/{projectId}/support/tickets/{id}/close".to_string(),
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

    fn user_api_create_project_support_ticket(
        &self,
        project_id: &str,
        body: crate::models::SupportCreateProjectSupportTicketRequest,
    ) -> Box<dyn Future<Item = crate::models::SupportSupportTicket, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/support/tickets".to_string(),
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

    fn user_api_get_project_support_ticket(
        &self,
        project_id: &str,
        id: &str,
    ) -> Box<dyn Future<Item = crate::models::SupportSupportTicket, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/support/tickets/{id}".to_string(),
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

    fn user_api_list_project_support_packages(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::SupportListProjectSupportPackagesResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/support/plans".to_string(),
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

    fn user_api_list_project_support_tickets(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::SupportListProjectSupportTicketsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/support/tickets".to_string(),
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
}
