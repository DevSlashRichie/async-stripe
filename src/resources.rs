//! resources module
//!
//! This module exposes various elements of the
//! stripe api depending on the features exposed.
//!
//! Some of these modules are hand-written, and
//! some are generated.

mod currency;
#[allow(clippy::module_inception)]
#[allow(clippy::new_without_default)]
pub mod generated;
mod types;

#[path = "resources"]
mod core {
    pub mod account_ext;
    pub mod charge_ext;
    pub mod customer_ext;
    pub mod payment_intent_ext;
    pub mod payment_source;
    pub mod placeholders;
    pub mod setup_intent_ext;
}

#[path = "resources"]
mod payment {
    pub mod bank_account_ext;
    pub mod card;
    pub mod payment_method_ext;
    pub mod source_ext;
}

#[cfg(feature = "events")]
mod webhook_events;

#[path = "resources"]
#[cfg(feature = "connect")]
mod connect {
    pub mod login_links_ext;
}

#[path = "resources"]
#[cfg(feature = "webhook-endpoints")]
mod webhook_endpoints {
    pub mod webhook_endpoint_ext;
}

#[rustfmt::skip]
pub use {
    currency::*,
    types::*,

    self::core::{
        account_ext::*,
        charge_ext::*,
        customer_ext::*,
        payment_intent_ext::*,
        payment_source::*,
        placeholders::*,
        setup_intent_ext::*,
    },
    generated::core::{
        address::*,
        billing_details::*,
        charge::*,
        connect_account_reference::*,
        customer::*,
        custom_unit_amount::*,
        ephemeral_key::*,
        payment_intent::*,
        payment_method_details_card_wallet_apple_pay::*,
        payment_method_details_card_wallet_google_pay::*,
        refund::*,
        setup_attempt::*,
        setup_intent::*,
        api_errors::*,
    },

    payment::{
        bank_account_ext::*,
        card::*,
        payment_method_ext::*,
        source_ext::*
    },
    generated::payment::{
        card::*,
        bank_account::*,
        payment_method::*,
        payment_method_card_present_networks::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "events")]
pub use {
    webhook_events::*,
    webhook_events::NotificationEventData,
    generated::event::*,
};

#[rustfmt::skip]
#[cfg(feature = "connect")]
pub use {
    connect::{
        login_links_ext::*,
    },
    generated::connect::{
        account_link::*,
        account::*,
        application::*,
        application_fee::*,
        connect_collection_transfer::*,
        fee_refund::*,
        login_link::*,
    }
};

#[rustfmt::skip]
#[cfg(feature = "webhook-endpoints")]
pub use {
    webhook_endpoints::webhook_endpoint_ext::*,
    generated::webhook_endpoints::webhook_endpoint::*,
};

#[cfg(not(feature = "full"))]
pub use generated::placeholders::*;

/// this struct is just a stub for code not using the "connect" feature
/// see https://github.com/arlyon/async-stripe/issues/49 for more context
/// if there are more features that requires a fully fledged CompanyParams
/// we probably need to update the code generation and move to a shared place
#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompanyParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}

/// this struct is just a stub for code not using the "connect" feature
/// see https://github.com/arlyon/async-stripe/issues/49 for more context
/// if there are more features that requires a fully fledged PersonParams
/// we probably need to update the code generation and move to a shared place
#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PersonParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}
