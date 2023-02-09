use std::borrow::Borrow;
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct UserAPIApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> UserAPIApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UserAPIApiClient<C> {
        UserAPIApiClient { configuration }
    }
}

pub trait UserAPIApi {
    fn user_api_subscribe_project_notifications(
        &self,
        project_ids: Option<Vec<String>>,
    ) -> Box<
        dyn Future<
            Item = crate::models::StreamResultOfNotificationProjectNotification,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> UserAPIApi for UserAPIApiClient<C> {
    fn user_api_subscribe_project_notifications(
        &self,
        project_ids: Option<Vec<String>>,
    ) -> Box<
        dyn Future<
            Item = crate::models::StreamResultOfNotificationProjectNotification,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/projects/notifications".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        if let Some(ref s) = project_ids {
            req = req.with_query_param("projectIds".to_string(), s.join(",").to_string());
        }

        req.execute(self.configuration.borrow())
    }
}
