use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
where
    T: serde::Deserialize<'de>,
{
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

mod request;

mod authentication;
pub use self::authentication::{Authentication, AuthenticationClient};
mod billing_api;
pub use self::billing_api::{BillingApi, BillingApiClient};
mod compute_api;
pub use self::compute_api::{ComputeApi, ComputeApiClient};
mod country_api;
pub use self::country_api::{CountryApi, CountryApiClient};
mod data_center_api;
pub use self::data_center_api::{DataCenterApi, DataCenterApiClient};
mod flavour_api;
pub use self::flavour_api::{FlavourApi, FlavourApiClient};
mod image_api;
pub use self::image_api::{ImageApi, ImageApiClient};
mod network_api;
pub use self::network_api::{NetworkApi, NetworkApiClient};
mod payment_api;
pub use self::payment_api::{PaymentApi, PaymentApiClient};
mod project_api;
pub use self::project_api::{ProjectApi, ProjectApiClient};
mod spla_api;
pub use self::spla_api::{SPLAApi, SPLAApiClient};
mod ssh_key_api;
pub use self::ssh_key_api::{SSHKeyApi, SSHKeyApiClient};
mod support_api;
pub use self::support_api::{SupportApi, SupportApiClient};
mod two_factor_api;
pub use self::two_factor_api::{TwoFactorApi, TwoFactorApiClient};
mod two_factor_recovery_code_api;
pub use self::two_factor_recovery_code_api::{
    TwoFactorRecoveryCodeApi, TwoFactorRecoveryCodeApiClient,
};
mod two_factor_totp_api;
pub use self::two_factor_totp_api::{TwoFactorTOTPApi, TwoFactorTOTPApiClient};
mod user_api;
pub use self::user_api::{UserApi, UserApiClient};
mod user_api_api;
pub use self::user_api_api::{UserAPIApi, UserAPIApiClient};

pub mod client;
pub mod configuration;
