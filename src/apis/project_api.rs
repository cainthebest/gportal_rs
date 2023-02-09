use std::borrow::Borrow;
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct ProjectApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ProjectApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ProjectApiClient<C> {
        ProjectApiClient { configuration }
    }
}

pub trait ProjectApi {
    fn user_api_change_default_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectChangeDefaultProjectRequest,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>;
    fn user_api_create_project(
        &self,
        body: crate::models::ProjectCreateProjectRequest,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>;
    fn user_api_delete_project(
        &self,
        project_id: &str,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_get_project(
        &self,
        project_id: &str,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>;
    fn user_api_get_project_logs(
        &self,
        project_id: &str,
        page: Option<i32>,
        search: Option<&str>,
        user_id: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectLogsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_project_traffic(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectTrafficResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_invite_member_to_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectInviteMemberToProjectRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectInviteMemberToProjectResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_join_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectJoinProjectRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_leave_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectLeaveProjectRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_list_project_ssh_keys(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectListProjectSshKeysResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_list_projects(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectListProjectsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_remove_member_from_project(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_update_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectUpdateProjectRequest,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ProjectApi for ProjectApiClient<C> {
    fn user_api_change_default_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectChangeDefaultProjectRequest,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/default".to_string(),
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

    fn user_api_create_project(
        &self,
        body: crate::models::ProjectCreateProjectRequest,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::Post, "/v1/projects".to_string())
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

    fn user_api_delete_project(
        &self,
        project_id: &str,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/projects/{projectId}".to_string(),
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

    fn user_api_get_project(
        &self,
        project_id: &str,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}".to_string(),
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

    fn user_api_get_project_logs(
        &self,
        project_id: &str,
        page: Option<i32>,
        search: Option<&str>,
        user_id: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectLogsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/logs".to_string(),
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
        if let Some(ref s) = user_id {
            req = req.with_query_param("userId".to_string(), s.to_string());
        }
        req = req.with_path_param("projectId".to_string(), project_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_project_traffic(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectTrafficResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/traffic".to_string(),
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

    fn user_api_invite_member_to_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectInviteMemberToProjectRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectInviteMemberToProjectResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/members".to_string(),
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

    fn user_api_join_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectJoinProjectRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/join".to_string(),
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

    fn user_api_leave_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectLeaveProjectRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/projects/{projectId}/leave".to_string(),
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

    fn user_api_list_project_ssh_keys(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectListProjectSshKeysResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/{projectId}/ssh_keys".to_string(),
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

    fn user_api_list_projects(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectListProjectsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(hyper::Method::Get, "/v1/projects".to_string())
            .with_auth(__internal_request::Auth::ApiKey(
                __internal_request::ApiKey {
                    in_header: true,
                    in_query: false,
                    param_name: "Authorization".to_owned(),
                },
            ));

        req.execute(self.configuration.borrow())
    }

    fn user_api_remove_member_from_project(
        &self,
        project_id: &str,
        user_id: &str,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/projects/{projectId}/members/{userId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("userId".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_update_project(
        &self,
        project_id: &str,
        body: crate::models::ProjectUpdateProjectRequest,
    ) -> Box<dyn Future<Item = crate::models::ProjectProject, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/v1/projects/{projectId}".to_string(),
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
}
