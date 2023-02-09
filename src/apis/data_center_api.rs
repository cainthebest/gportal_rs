use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct DataCenterApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DataCenterApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DataCenterApiClient<C> {
        DataCenterApiClient { configuration }
    }
}

pub trait DataCenterApi {
    fn user_api_list_data_centers(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::RegionListDataCenterResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> DataCenterApi for DataCenterApiClient<C> {
    fn user_api_list_data_centers(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::RegionListDataCenterResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req =
            __internal_request::Request::new(hyper::Method::Get, "/v1/datacenters".to_string())
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
