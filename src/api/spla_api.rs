use std::borrow::Borrow;
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct SPLAApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> SPLAApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SPLAApiClient<C> {
        SPLAApiClient { configuration }
    }
}

pub trait SPLAApi {
    fn user_api_get_spla_price(
        &self,
        project_id: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::SplaGetSplaPriceResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::connect::Connect> SPLAApi for SPLAApiClient<C> {
    fn user_api_get_spla_price(
        &self,
        project_id: Option<&str>,
    ) -> Box<
        dyn Future<
            Item = crate::models::SplaGetSplaPriceResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/v1/spla/price".to_string())
                .with_auth(__internal_request::Auth::ApiKey(
                    __internal_request::ApiKey {
                        in_header: true,
                        in_query: false,
                        param_name: "Authorization".to_owned(),
                    },
                ));
        if let Some(ref s) = project_id {
            req = req.with_query_param("projectId".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }
}
