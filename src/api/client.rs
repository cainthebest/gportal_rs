use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    authentication_api: Box<dyn crate::api::Authentication>,
    billing_api: Box<dyn crate::api::BillingApi>,
    compute_api: Box<dyn crate::api::ComputeApi>,
    country_api: Box<dyn crate::api::CountryApi>,
    data_center_api: Box<dyn crate::api::DataCenterApi>,
    flavour_api: Box<dyn crate::api::FlavourApi>,
    image_api: Box<dyn crate::api::ImageApi>,
    network_api: Box<dyn crate::api::NetworkApi>,
    payment_api: Box<dyn crate::api::PaymentApi>,
    project_api: Box<dyn crate::api::ProjectApi>,
    spla_api: Box<dyn crate::api::SPLAApi>,
    ssh_key_api: Box<dyn crate::api::SSHKeyApi>,
    support_api: Box<dyn crate::api::SupportApi>,
    two_factor_api: Box<dyn crate::api::TwoFactorApi>,
    two_factor_recovery_code_api: Box<dyn crate::api::TwoFactorRecoveryCodeApi>,
    two_factor_totp_api: Box<dyn crate::api::TwoFactorTOTPApi>,
    user_api: Box<dyn crate::api::UserApi>,
    user_api_api: Box<dyn crate::api::UserAPIApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            authentication_api: Box::new(crate::api::AuthenticationClient::new(rc.clone())),
            billing_api: Box::new(crate::api::BillingApiClient::new(rc.clone())),
            compute_api: Box::new(crate::api::ComputeApiClient::new(rc.clone())),
            country_api: Box::new(crate::api::CountryApiClient::new(rc.clone())),
            data_center_api: Box::new(crate::api::DataCenterApiClient::new(rc.clone())),
            flavour_api: Box::new(crate::api::FlavourApiClient::new(rc.clone())),
            image_api: Box::new(crate::api::ImageApiClient::new(rc.clone())),
            network_api: Box::new(crate::api::NetworkApiClient::new(rc.clone())),
            payment_api: Box::new(crate::api::PaymentApiClient::new(rc.clone())),
            project_api: Box::new(crate::api::ProjectApiClient::new(rc.clone())),
            spla_api: Box::new(crate::api::SPLAApiClient::new(rc.clone())),
            ssh_key_api: Box::new(crate::api::SSHKeyApiClient::new(rc.clone())),
            support_api: Box::new(crate::api::SupportApiClient::new(rc.clone())),
            two_factor_api: Box::new(crate::api::TwoFactorApiClient::new(rc.clone())),
            two_factor_recovery_code_api: Box::new(
                crate::api::TwoFactorRecoveryCodeApiClient::new(rc.clone()),
            ),
            two_factor_totp_api: Box::new(crate::api::TwoFactorTOTPApiClient::new(rc.clone())),
            user_api: Box::new(crate::api::UserApiClient::new(rc.clone())),
            user_api_api: Box::new(crate::api::UserAPIApiClient::new(rc.clone())),
        }
    }

    pub fn authentication_api(&self) -> &dyn crate::api::Authentication {
        self.authentication_api.as_ref()
    }

    pub fn billing_api(&self) -> &dyn crate::api::BillingApi {
        self.billing_api.as_ref()
    }

    pub fn compute_api(&self) -> &dyn crate::api::ComputeApi {
        self.compute_api.as_ref()
    }

    pub fn country_api(&self) -> &dyn crate::api::CountryApi {
        self.country_api.as_ref()
    }

    pub fn data_center_api(&self) -> &dyn crate::api::DataCenterApi {
        self.data_center_api.as_ref()
    }

    pub fn flavour_api(&self) -> &dyn crate::api::FlavourApi {
        self.flavour_api.as_ref()
    }

    pub fn image_api(&self) -> &dyn crate::api::ImageApi {
        self.image_api.as_ref()
    }

    pub fn network_api(&self) -> &dyn crate::api::NetworkApi {
        self.network_api.as_ref()
    }

    pub fn payment_api(&self) -> &dyn crate::api::PaymentApi {
        self.payment_api.as_ref()
    }

    pub fn project_api(&self) -> &dyn crate::api::ProjectApi {
        self.project_api.as_ref()
    }

    pub fn spla_api(&self) -> &dyn crate::api::SPLAApi {
        self.spla_api.as_ref()
    }

    pub fn ssh_key_api(&self) -> &dyn crate::api::SSHKeyApi {
        self.ssh_key_api.as_ref()
    }

    pub fn support_api(&self) -> &dyn crate::api::SupportApi {
        self.support_api.as_ref()
    }

    pub fn two_factor_api(&self) -> &dyn crate::api::TwoFactorApi {
        self.two_factor_api.as_ref()
    }

    pub fn two_factor_recovery_code_api(&self) -> &dyn crate::api::TwoFactorRecoveryCodeApi {
        self.two_factor_recovery_code_api.as_ref()
    }

    pub fn two_factor_totp_api(&self) -> &dyn crate::api::TwoFactorTOTPApi {
        self.two_factor_totp_api.as_ref()
    }

    pub fn user_api(&self) -> &dyn crate::api::UserApi {
        self.user_api.as_ref()
    }

    pub fn user_api_api(&self) -> &dyn crate::api::UserAPIApi {
        self.user_api_api.as_ref()
    }
}
