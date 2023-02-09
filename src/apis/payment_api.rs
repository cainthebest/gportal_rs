use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct PaymentApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PaymentApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PaymentApiClient<C> {
        PaymentApiClient { configuration }
    }
}

pub trait PaymentApi {
    fn user_api_add_credit_card(
        &self,
        body: crate::models::PaymentAddCreditCardRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentAddCreditCardResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_delete_credit_card(
        &self,
        card_id: &str,
        body: crate::models::PaymentDeleteCreditCardRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_list_credit_cards(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentListCreditCardsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> PaymentApi for PaymentApiClient<C> {
    fn user_api_add_credit_card(
        &self,
        body: crate::models::PaymentAddCreditCardRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentAddCreditCardResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/v1/profile/credit_cards".to_string(),
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

    fn user_api_delete_credit_card(
        &self,
        card_id: &str,
        body: crate::models::PaymentDeleteCreditCardRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/v1/profile/credit_cards/{cardId}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("cardId".to_string(), card_id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_list_credit_cards(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentListCreditCardsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(
            hyper::Method::Get,
            "/v1/profile/credit_cards".to_string(),
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
