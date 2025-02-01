//! generated module
//!
//! Contains the generated impls we use. This code
//! is automatically generated from the openapi spec
//! and should not be changed manually. To update the
//! spec, use cargo make.
//!
//! It is possible more files are generated than are
//! listed as modules here. These are modules that
//! have not yet been exposed by the client.

#[path = "generated"]
pub mod core {
    pub mod address;
    pub mod api_errors;
    pub mod billing_details;
    pub mod charge;
    pub mod connect_account_reference;
    pub mod custom_unit_amount;
    pub mod customer;
    pub mod ephemeral_key;
    pub mod payment_intent;
    pub mod payment_method_details_card_wallet_apple_pay;
    pub mod payment_method_details_card_wallet_google_pay;
    pub mod refund;
    pub mod setup_attempt;
    pub mod setup_intent;
    pub mod version;
}

#[path = "generated"]
pub mod payment {
    pub mod bank_account;
    pub mod card;
    pub mod payment_method;
    pub mod payment_method_card_present_networks;
    pub mod payment_method_domain;
}

#[path = "generated"]
#[cfg(feature = "connect")]
pub mod connect {
    pub mod account;
    pub mod account_link;
    pub mod account_session;
    pub mod application;
    pub mod application_fee;
    pub mod connect_collection_transfer;
    pub mod fee_refund;
    pub mod login_link;
    // pub mod transfer;
    // pub mod transfer_reversal;
}

#[cfg(feature = "events")]
pub mod event;

#[path = "generated"]
#[cfg(feature = "webhook-endpoints")]
pub mod webhook_endpoints {
    pub mod webhook_endpoint;
}

#[cfg(not(feature = "full"))]
pub mod placeholders;
