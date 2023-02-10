use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct ImageApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> ImageApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ImageApiClient<C> {
        ImageApiClient { configuration }
    }
}

pub trait ImageApi {
    fn user_api_create_project_image(
        &self,
        project_id: &str,
        body: crate::models::ImageCreateProjectImageRequest,
    ) -> Box<dyn Future<Item = crate::models::ApiimageImage, Error = Error<serde_json::Value>>>;
    fn user_api_delete_project_image(
        &self,
        project_id: &str,
        image_id: &str,
        body: crate::models::ImageDeleteProjectImageRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_delete_project_image_version(
        &self,
        project_id: &str,
        image_version_id: &str,
        body: crate::models::ImageDeleteProjectImageVersionRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_get_project_image(
        &self,
        project_id: &str,
        image_id: &str,
    ) -> Box<dyn Future<Item = crate::models::ApiimageImage, Error = Error<serde_json::Value>>>;
    fn user_api_list_project_images(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ImageListProjectImagesResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_list_public_images(
        &self,
        flavour_id: &str,
    ) -> Box<
        dyn Future<Item = crate::models::ImageListImagesResponse, Error = Error<serde_json::Value>>,
    >;
}

impl<C: hyper::client::connect::Connect> ImageApi for ImageApiClient<C> {
    fn user_api_create_project_image(
        &self,
        project_id: &str,
        body: crate::models::ImageCreateProjectImageRequest,
    ) -> Box<dyn Future<Item = crate::models::ApiimageImage, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/images".to_string(),
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

    fn user_api_delete_project_image(
        &self,
        project_id: &str,
        image_id: &str,
        body: crate::models::ImageDeleteProjectImageRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/images/{imageId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("imageId".to_string(), image_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_delete_project_image_version(
        &self,
        project_id: &str,
        image_version_id: &str,
        body: crate::models::ImageDeleteProjectImageVersionRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/images/version/{imageVersionId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("imageVersionId".to_string(), image_version_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_project_image(
        &self,
        project_id: &str,
        image_id: &str,
    ) -> Box<dyn Future<Item = crate::models::ApiimageImage, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/images/{imageId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("imageId".to_string(), image_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_project_images(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ImageListProjectImagesResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/images".to_string(),
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

    fn user_api_list_public_images(
        &self,
        flavour_id: &str,
    ) -> Box<
        dyn Future<Item = crate::models::ImageListImagesResponse, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/images/{flavourId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("flavourId".to_string(), flavour_id.to_string());

        req.execute(self.configuration.borrow())
    }
}
