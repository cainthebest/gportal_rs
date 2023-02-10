use std::borrow::Borrow;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct BillingClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> BillingClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BillingClient<C> {
        BillingClient { configuration }
    }
}

pub trait Billing {
    fn user_api_create_billing_address(
        &self,
        body: crate::models::PaymentCreateBillingAddressRequest,
    ) -> Box<
        dyn Future<Item = crate::models::PaymentBillingAddress, Error = Error<serde_json::Value>>,
    >;
    fn user_api_delete_billing_address(
        &self,
        id: &str,
        body: crate::models::PaymentDeleteBillingAddressRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_get_project_bill_pdf(
        &self,
        project_id: &str,
        bill_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectBillPdfResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_project_bills(
        &self,
        project_id: &str,
        year: i32,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectBillsResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_project_current_billing_preview(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectCurrentBillingPreviewResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_project_current_billing_preview_pdf(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectCurrentBillingPreviewPdfResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_get_projects_outstanding_balance(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectsOutstandingBalanceResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_list_billing_addresses(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentListBillingAddressesResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_pay_project_now(
        &self,
        project_id: &str,
        body: crate::models::ProjectPayProjectNowRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>>;
    fn user_api_redeem_voucher(
        &self,
        project_id: &str,
        body: crate::models::VoucherRedeemVoucherRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::VoucherRedeemVoucherResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn user_api_update_billing_address(
        &self,
        id: &str,
        body: crate::models::PaymentUpdateBillingAddressRequest,
    ) -> Box<
        dyn Future<Item = crate::models::PaymentBillingAddress, Error = Error<serde_json::Value>>,
    >;
}

impl<C: hyper::client::connect::Connect> Billing for BillingClient<C> {
    fn user_api_create_billing_address(
        &self,
        body: crate::models::PaymentCreateBillingAddressRequest,
    ) -> Box<
        dyn Future<Item = crate::models::PaymentBillingAddress, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/profile/billing/addresses".to_string(),
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

    fn user_api_delete_billing_address(
        &self,
        id: &str,
        body: crate::models::PaymentDeleteBillingAddressRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/v1/profile/billing/addresses/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_project_bill_pdf(
        &self,
        project_id: &str,
        bill_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectBillPdfResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/billing/bill/{billId}/pdf".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("billId".to_string(), bill_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_project_bills(
        &self,
        project_id: &str,
        year: i32,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectBillsResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/billing/bills/{year}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("year".to_string(), year.to_string());

        req.execute(self.configuration.borrow())
    }

    fn user_api_get_project_current_billing_preview(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectCurrentBillingPreviewResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/billing/preview".to_string(),
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

    fn user_api_get_project_current_billing_preview_pdf(
        &self,
        project_id: &str,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectCurrentBillingPreviewPdfResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/{projectId}/billing/preview/pdf".to_string(),
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

    fn user_api_get_projects_outstanding_balance(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::ProjectGetProjectsOutstandingBalanceResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/projects/balance".to_string(),
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

    fn user_api_list_billing_addresses(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::PaymentListBillingAddressesResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let req = __internal_request::Request::new(
            hyper::Method::GET,
            "/v1/profile/billing/addresses".to_string(),
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

    fn user_api_pay_project_now(
        &self,
        project_id: &str,
        body: crate::models::ProjectPayProjectNowRequest,
    ) -> Box<dyn Future<Item = serde_json::Value, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/billing/now".to_string(),
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

    fn user_api_redeem_voucher(
        &self,
        project_id: &str,
        body: crate::models::VoucherRedeemVoucherRequest,
    ) -> Box<
        dyn Future<
            Item = crate::models::VoucherRedeemVoucherResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/v1/projects/{projectId}/billing/voucher".to_string(),
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

    fn user_api_update_billing_address(
        &self,
        id: &str,
        body: crate::models::PaymentUpdateBillingAddressRequest,
    ) -> Box<
        dyn Future<Item = crate::models::PaymentBillingAddress, Error = Error<serde_json::Value>>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/v1/profile/billing/addresses/{id}".to_string(),
        )
        .with_auth(__internal_request::Auth::ApiKey(
            __internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            },
        ));
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }
}
