use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient {
    authentication_api: Box<dyn crate::apis::AuthenticationApi>,
    billing_api: Box<dyn crate::apis::BillingApi>,
    compute_api: Box<dyn crate::apis::ComputeApi>,
    country_api: Box<dyn crate::apis::CountryApi>,
    data_center_api: Box<dyn crate::apis::DataCenterApi>,
    flavour_api: Box<dyn crate::apis::FlavourApi>,
    image_api: Box<dyn crate::apis::ImageApi>,
    network_api: Box<dyn crate::apis::NetworkApi>,
    payment_api: Box<dyn crate::apis::PaymentApi>,
    project_api: Box<dyn crate::apis::ProjectApi>,
    spla_api: Box<dyn crate::apis::SPLAApi>,
    ssh_key_api: Box<dyn crate::apis::SSHKeyApi>,
    support_api: Box<dyn crate::apis::SupportApi>,
    two_factor_api: Box<dyn crate::apis::TwoFactorApi>,
    two_factor_recovery_code_api: Box<dyn crate::apis::TwoFactorRecoveryCodeApi>,
    two_factor_totp_api: Box<dyn crate::apis::TwoFactorTOTPApi>,
    user_api: Box<dyn crate::apis::UserApi>,
    user_api_api: Box<dyn crate::apis::UserAPIApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(rc.clone())),
            billing_api: Box::new(crate::apis::BillingApiClient::new(rc.clone())),
            compute_api: Box::new(crate::apis::ComputeApiClient::new(rc.clone())),
            country_api: Box::new(crate::apis::CountryApiClient::new(rc.clone())),
            data_center_api: Box::new(crate::apis::DataCenterApiClient::new(rc.clone())),
            flavour_api: Box::new(crate::apis::FlavourApiClient::new(rc.clone())),
            image_api: Box::new(crate::apis::ImageApiClient::new(rc.clone())),
            network_api: Box::new(crate::apis::NetworkApiClient::new(rc.clone())),
            payment_api: Box::new(crate::apis::PaymentApiClient::new(rc.clone())),
            project_api: Box::new(crate::apis::ProjectApiClient::new(rc.clone())),
            spla_api: Box::new(crate::apis::SPLAApiClient::new(rc.clone())),
            ssh_key_api: Box::new(crate::apis::SSHKeyApiClient::new(rc.clone())),
            support_api: Box::new(crate::apis::SupportApiClient::new(rc.clone())),
            two_factor_api: Box::new(crate::apis::TwoFactorApiClient::new(rc.clone())),
            two_factor_recovery_code_api: Box::new(
                crate::apis::TwoFactorRecoveryCodeApiClient::new(rc.clone()),
            ),
            two_factor_totp_api: Box::new(crate::apis::TwoFactorTOTPApiClient::new(rc.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(rc.clone())),
            user_api_api: Box::new(crate::apis::UserAPIApiClient::new(rc.clone())),
        }
    }

    pub fn authentication_api(&self) -> &dyn crate::apis::AuthenticationApi {
        self.authentication_api.as_ref()
    }

    pub fn billing_api(&self) -> &dyn crate::apis::BillingApi {
        self.billing_api.as_ref()
    }

    pub fn compute_api(&self) -> &dyn crate::apis::ComputeApi {
        self.compute_api.as_ref()
    }

    pub fn country_api(&self) -> &dyn crate::apis::CountryApi {
        self.country_api.as_ref()
    }

    pub fn data_center_api(&self) -> &dyn crate::apis::DataCenterApi {
        self.data_center_api.as_ref()
    }

    pub fn flavour_api(&self) -> &dyn crate::apis::FlavourApi {
        self.flavour_api.as_ref()
    }

    pub fn image_api(&self) -> &dyn crate::apis::ImageApi {
        self.image_api.as_ref()
    }

    pub fn network_api(&self) -> &dyn crate::apis::NetworkApi {
        self.network_api.as_ref()
    }

    pub fn payment_api(&self) -> &dyn crate::apis::PaymentApi {
        self.payment_api.as_ref()
    }

    pub fn project_api(&self) -> &dyn crate::apis::ProjectApi {
        self.project_api.as_ref()
    }

    pub fn spla_api(&self) -> &dyn crate::apis::SPLAApi {
        self.spla_api.as_ref()
    }

    pub fn ssh_key_api(&self) -> &dyn crate::apis::SSHKeyApi {
        self.ssh_key_api.as_ref()
    }

    pub fn support_api(&self) -> &dyn crate::apis::SupportApi {
        self.support_api.as_ref()
    }

    pub fn two_factor_api(&self) -> &dyn crate::apis::TwoFactorApi {
        self.two_factor_api.as_ref()
    }

    pub fn two_factor_recovery_code_api(&self) -> &dyn crate::apis::TwoFactorRecoveryCodeApi {
        self.two_factor_recovery_code_api.as_ref()
    }

    pub fn two_factor_totp_api(&self) -> &dyn crate::apis::TwoFactorTOTPApi {
        self.two_factor_totp_api.as_ref()
    }

    pub fn user_api(&self) -> &dyn crate::apis::UserApi {
        self.user_api.as_ref()
    }

    pub fn user_api_api(&self) -> &dyn crate::apis::UserAPIApi {
        self.user_api_api.as_ref()
    }
}
