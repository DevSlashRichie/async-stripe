// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodConfigurationId, PaymentMethodId, SetupIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{
    Account, ApiErrors, Application, Currency, Customer, SetupAttempt,
};

/// The resource representing a Stripe "SetupIntent".
///
/// For more details see <https://stripe.com/docs/api/setup_intents/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntent {
    /// Unique identifier for the object.
    pub id: SetupIntentId,

    /// ID of the Connect application that created the SetupIntent.
    pub application: Option<Expandable<Application>>,

    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,

    /// Settings for dynamic payment methods compatible with this Setup Intent.
    pub automatic_payment_methods: Option<PaymentFlowsAutomaticPaymentMethodsSetupIntent>,

    /// Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    pub cancellation_reason: Option<SetupIntentCancellationReason>,

    /// The client secret of this SetupIntent.
    ///
    /// Used for client-side retrieval using a publishable key.  The client secret can be used to complete payment setup from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<SetupIntentFlowDirections>>,

    /// The error encountered in the previous SetupIntent confirmation.
    pub last_setup_error: Option<Box<ApiErrors>>,

    /// The most recent SetupAttempt for this SetupIntent.
    pub latest_attempt: Option<Expandable<SetupAttempt>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    pub next_action: Option<SetupIntentNextAction>,

    /// The account (if any) for which the setup is intended.
    pub on_behalf_of: Option<Expandable<Account>>,

    /// Payment method-specific configuration for this SetupIntent.
    pub payment_method_options: Option<SetupIntentPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to set up.
    pub payment_method_types: Vec<String>,

    /// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
    pub status: SetupIntentStatus,

    /// Indicates how the payment method is intended to be used in the future.
    ///
    /// Use `on_session` if you intend to only reuse the payment method when the customer is in your checkout flow.
    ///
    /// Use `off_session` if your customer may or may not be in your checkout flow.
    /// If not provided, this value defaults to `off_session`.
    pub usage: String,
}

impl SetupIntent {
    /// Returns a list of SetupIntents.
    pub fn list(client: &Client, params: &ListSetupIntents<'_>) -> Response<List<SetupIntent>> {
        client.get_query("/setup_intents", params)
    }

    /// Creates a SetupIntent object.
    ///
    /// After you create the SetupIntent, attach a payment method and [confirm](https://stripe.com/docs/api/setup_intents/confirm)
    /// it to collect any required permissions to charge the payment method later.
    pub fn create(client: &Client, params: CreateSetupIntent<'_>) -> Response<SetupIntent> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/setup_intents", &params)
    }

    /// Retrieves the details of a SetupIntent that has previously been created.
    ///
    /// Client-side retrieval using a publishable key is allowed when the `client_secret` is provided in the query string.
    /// When retrieved with a publishable key, only a subset of properties will be returned.
    /// Please refer to the [SetupIntent](https://stripe.com/docs/api#setup_intent_object) object reference for more details.
    pub fn retrieve(client: &Client, id: &SetupIntentId, expand: &[&str]) -> Response<SetupIntent> {
        client.get_query(&format!("/setup_intents/{}", id), Expand { expand })
    }

    /// Updates a SetupIntent object.
    pub fn update(
        client: &Client,
        id: &SetupIntentId,
        params: UpdateSetupIntent<'_>,
    ) -> Response<SetupIntent> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/setup_intents/{}", id), &params)
    }
}

impl Object for SetupIntent {
    type Id = SetupIntentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "setup_intent"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    ///
    /// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects>,

    /// Automatically calculates compatible payment methods.
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentNextAction {
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[serde(rename = "type")]
    pub type_: String,

    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentNextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,

    /// The URL you must redirect your customer to in order to authenticate.
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: Timestamp,

    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,

    /// The type of the microdeposit sent to the customer.
    ///
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SetupIntentPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupIntentPaymentMethodOptionsLink>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    /// Currency supported by the bank account.
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsCardMandateOptions>,

    /// Selected network to process this SetupIntent on.
    ///
    /// Depends on the available networks of the card attached to the setup intent.
    /// Can be only set confirm-time.
    pub network: Option<SetupIntentPaymentMethodOptionsCardNetwork>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<SetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    pub amount: i64,

    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,

    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    pub end_date: Option<Timestamp>,

    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: SetupIntentPaymentMethodOptionsCardMandateOptionsInterval,

    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    pub interval_count: Option<u64>,

    /// Unique identifier for the mandate or subscription.
    pub reference: String,

    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: Timestamp,

    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    pub supported_types:
        Option<Vec<SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsLink {
    /// [Deprecated] This is a legacy parameter that no longer has any function.
    pub persistent_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>>,

    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    pub payment_schedule:
        Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,

    /// Transaction type of the mandate.
    pub transaction_type:
        Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsPaypal {
    /// The PayPal Billing Agreement ID (BAID).
    ///
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    pub billing_agreement_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

/// The parameters for `SetupIntent::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSetupIntent<'a> {
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,

    /// When you enable this parameter, this SetupIntent accepts payment methods that you enable in the Dashboard and that are compatible with its other parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<CreateSetupIntentAutomaticPaymentMethods>,

    /// Set to `true` to attempt to confirm this SetupIntent immediately.
    ///
    /// This parameter defaults to `false`.
    /// If a card is the attached payment method, you can provide a `return_url` in case further authentication is necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<CreateSetupIntentFlowDirections>>,

    /// This hash contains details about the mandate to create.
    ///
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<CreateSetupIntentMandateData>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The Stripe account ID created for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// The ID of the payment method configuration to use with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<PaymentMethodConfigurationId>,

    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
    /// value in the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<CreateSetupIntentPaymentMethodData>,

    /// Payment method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSetupIntentPaymentMethodOptions>,

    /// The list of payment method types (for example, card) that this SetupIntent can use.
    ///
    /// If you don't provide this, it defaults to ["card"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,

    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    ///
    /// To redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,

    /// If you populate this hash, this SetupIntent generates a `single_use` mandate after successful completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<CreateSetupIntentSingleUse>,

    /// Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<bool>,
}

impl<'a> CreateSetupIntent<'a> {
    pub fn new() -> Self {
        CreateSetupIntent {
            attach_to_self: Default::default(),
            automatic_payment_methods: Default::default(),
            confirm: Default::default(),
            customer: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            flow_directions: Default::default(),
            mandate_data: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_method: Default::default(),
            payment_method_configuration: Default::default(),
            payment_method_data: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            return_url: Default::default(),
            single_use: Default::default(),
            use_stripe_sdk: Default::default(),
        }
    }
}

/// The parameters for `SetupIntent::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListSetupIntents<'a> {
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return SetupIntents for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SetupIntentId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return SetupIntents that associate with the specified payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SetupIntentId>,
}

impl<'a> ListSetupIntents<'a> {
    pub fn new() -> Self {
        ListSetupIntents {
            attach_to_self: Default::default(),
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_method: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListSetupIntents<'_> {
    type O = SetupIntent;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `SetupIntent::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSetupIntent<'a> {
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,

    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<UpdateSetupIntentFlowDirections>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// The ID of the payment method configuration to use with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<PaymentMethodConfigurationId>,

    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
    /// value in the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<UpdateSetupIntentPaymentMethodData>,

    /// Payment method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateSetupIntentPaymentMethodOptions>,

    /// The list of payment method types (for example, card) that this SetupIntent can set up.
    ///
    /// If you don't provide this array, it defaults to ["card"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
}

impl<'a> UpdateSetupIntent<'a> {
    pub fn new() -> Self {
        UpdateSetupIntent {
            attach_to_self: Default::default(),
            customer: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            flow_directions: Default::default(),
            metadata: Default::default(),
            payment_method: Default::default(),
            payment_method_configuration: Default::default(),
            payment_method_data: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentAutomaticPaymentMethods {
    /// Controls whether this SetupIntent will accept redirect-based payment methods.
    ///
    /// Redirect-based payment methods may require your customer to be redirected to a payment method's app or site for authentication or additional steps.
    ///
    /// To [confirm](https://stripe.com/docs/api/setup_intents/confirm) this SetupIntent, you may be required to provide a `return_url` to redirect customers back to your site after they authenticate or complete the setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<CreateSetupIntentAutomaticPaymentMethodsAllowRedirects>,

    /// Whether this feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateData {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: CreateSetupIntentMandateDataCustomerAcceptance,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodData {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSetupIntentPaymentMethodDataAcssDebit>,

    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateSetupIntentPaymentMethodDataAffirm>,

    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateSetupIntentPaymentMethodDataAfterpayClearpay>,

    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateSetupIntentPaymentMethodDataAlipay>,

    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateSetupIntentPaymentMethodDataAuBecsDebit>,

    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateSetupIntentPaymentMethodDataBacsDebit>,

    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateSetupIntentPaymentMethodDataBancontact>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateSetupIntentPaymentMethodDataBillingDetails>,

    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreateSetupIntentPaymentMethodDataBlik>,

    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateSetupIntentPaymentMethodDataBoleto>,

    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreateSetupIntentPaymentMethodDataCashapp>,

    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateSetupIntentPaymentMethodDataCustomerBalance>,

    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateSetupIntentPaymentMethodDataEps>,

    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateSetupIntentPaymentMethodDataFpx>,

    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreateSetupIntentPaymentMethodDataGiropay>,

    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreateSetupIntentPaymentMethodDataGrabpay>,

    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateSetupIntentPaymentMethodDataIdeal>,

    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<CreateSetupIntentPaymentMethodDataInteracPresent>,

    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateSetupIntentPaymentMethodDataKlarna>,

    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateSetupIntentPaymentMethodDataKonbini>,

    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateSetupIntentPaymentMethodDataLink>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateSetupIntentPaymentMethodDataOxxo>,

    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateSetupIntentPaymentMethodDataP24>,

    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateSetupIntentPaymentMethodDataPaynow>,

    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateSetupIntentPaymentMethodDataPaypal>,

    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateSetupIntentPaymentMethodDataPix>,

    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreateSetupIntentPaymentMethodDataPromptpay>,

    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreateSetupIntentPaymentMethodDataRadarOptions>,

    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreateSetupIntentPaymentMethodDataRevolutPay>,

    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodDataSepaDebit>,

    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateSetupIntentPaymentMethodDataSofort>,

    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreateSetupIntentPaymentMethodDataSwish>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateSetupIntentPaymentMethodDataType,

    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodDataUsBankAccount>,

    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateSetupIntentPaymentMethodDataWechatPay>,

    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<CreateSetupIntentPaymentMethodDataZip>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptions {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSetupIntentPaymentMethodOptionsAcssDebit>,

    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSetupIntentPaymentMethodOptionsCard>,

    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateSetupIntentPaymentMethodOptionsLink>,

    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateSetupIntentPaymentMethodOptionsPaypal>,

    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodOptionsSepaDebit>,

    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentSingleUse {
    /// Amount the customer is granting permission to collect later.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodData {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSetupIntentPaymentMethodDataAcssDebit>,

    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdateSetupIntentPaymentMethodDataAffirm>,

    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<UpdateSetupIntentPaymentMethodDataAfterpayClearpay>,

    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdateSetupIntentPaymentMethodDataAlipay>,

    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdateSetupIntentPaymentMethodDataAuBecsDebit>,

    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdateSetupIntentPaymentMethodDataBacsDebit>,

    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateSetupIntentPaymentMethodDataBancontact>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<UpdateSetupIntentPaymentMethodDataBillingDetails>,

    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdateSetupIntentPaymentMethodDataBlik>,

    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdateSetupIntentPaymentMethodDataBoleto>,

    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<UpdateSetupIntentPaymentMethodDataCashapp>,

    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdateSetupIntentPaymentMethodDataCustomerBalance>,

    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdateSetupIntentPaymentMethodDataEps>,

    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdateSetupIntentPaymentMethodDataFpx>,

    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<UpdateSetupIntentPaymentMethodDataGiropay>,

    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<UpdateSetupIntentPaymentMethodDataGrabpay>,

    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdateSetupIntentPaymentMethodDataIdeal>,

    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<UpdateSetupIntentPaymentMethodDataInteracPresent>,

    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateSetupIntentPaymentMethodDataKlarna>,

    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdateSetupIntentPaymentMethodDataKonbini>,

    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdateSetupIntentPaymentMethodDataLink>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdateSetupIntentPaymentMethodDataOxxo>,

    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdateSetupIntentPaymentMethodDataP24>,

    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<UpdateSetupIntentPaymentMethodDataPaynow>,

    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<UpdateSetupIntentPaymentMethodDataPaypal>,

    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<UpdateSetupIntentPaymentMethodDataPix>,

    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<UpdateSetupIntentPaymentMethodDataPromptpay>,

    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<UpdateSetupIntentPaymentMethodDataRadarOptions>,

    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<UpdateSetupIntentPaymentMethodDataRevolutPay>,

    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodDataSepaDebit>,

    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdateSetupIntentPaymentMethodDataSofort>,

    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<UpdateSetupIntentPaymentMethodDataSwish>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: UpdateSetupIntentPaymentMethodDataType,

    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodDataUsBankAccount>,

    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdateSetupIntentPaymentMethodDataWechatPay>,

    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<UpdateSetupIntentPaymentMethodDataZip>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptions {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebit>,

    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSetupIntentPaymentMethodOptionsCard>,

    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdateSetupIntentPaymentMethodOptionsLink>,

    /// If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<UpdateSetupIntentPaymentMethodOptionsPaypal>,

    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebit>,

    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptance {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<Timestamp>,

    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateSetupIntentMandateDataCustomerAcceptanceOffline>,

    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<CreateSetupIntentMandateDataCustomerAcceptanceOnline>,

    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CreateSetupIntentMandateDataCustomerAcceptanceType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAcssDebit {
    /// Customer's bank account number.
    pub account_number: String,

    /// Institution number of the customer's bank.
    pub institution_number: String,

    /// Transit number of the customer's bank.
    pub transit_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAffirm {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAfterpayClearpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAlipay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,

    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBancontact {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateSetupIntentPaymentMethodDataBillingDetailsAddress>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBlik {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataCashapp {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataCustomerBalance {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataEpsBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateSetupIntentPaymentMethodDataFpxAccountHolderType>,

    /// The customer's bank.
    pub bank: CreateSetupIntentPaymentMethodDataFpxBank,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataGiropay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataGrabpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataIdealBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataInteracPresent {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreateSetupIntentPaymentMethodDataKlarnaDob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataOxxo {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataP24Bank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPaynow {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPaypal {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPix {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPromptpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataRevolutPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreateSetupIntentPaymentMethodDataSofortCountry,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSwish {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,

    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateSetupIntentPaymentMethodDataUsBankAccountAccountType>,

    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataWechatPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataZip {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsCardMandateOptions>,

    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,

    /// Selected network to process this SetupIntent on.
    ///
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateSetupIntentPaymentMethodOptionsCardNetwork>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,

    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsLink {
    /// [Deprecated] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsPaypal {
    /// The PayPal Billing Agreement ID (BAID).
    ///
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_agreement_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections>,

    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions>,

    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAcssDebit {
    /// Customer's bank account number.
    pub account_number: String,

    /// Institution number of the customer's bank.
    pub institution_number: String,

    /// Transit number of the customer's bank.
    pub transit_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAffirm {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAfterpayClearpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAlipay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,

    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBancontact {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateSetupIntentPaymentMethodDataBillingDetailsAddress>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBlik {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataCashapp {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataCustomerBalance {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataEpsBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateSetupIntentPaymentMethodDataFpxAccountHolderType>,

    /// The customer's bank.
    pub bank: UpdateSetupIntentPaymentMethodDataFpxBank,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataGiropay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataGrabpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataIdealBank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataInteracPresent {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<UpdateSetupIntentPaymentMethodDataKlarnaDob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataOxxo {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataP24Bank>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPaynow {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPaypal {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPix {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPromptpay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataRevolutPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: UpdateSetupIntentPaymentMethodDataSofortCountry,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSwish {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,

    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType>,

    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataWechatPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataZip {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsCardMandateOptions>,

    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,

    /// Selected network to process this SetupIntent on.
    ///
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<UpdateSetupIntentPaymentMethodOptionsCardNetwork>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,

    /// If 3D Secure authentication was performed with a third-party provider,
    /// the authentication details to use for this setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsLink {
    /// [Deprecated] This is a legacy parameter that no longer has any function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsPaypal {
    /// The PayPal Billing Agreement ID (BAID).
    ///
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_agreement_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections>,

    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions>,

    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptanceOffline {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptanceOnline {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: String,

    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<Vec<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor>>,

    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    pub amount: i64,

    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,

    /// Currency in which future payments will be charged.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Timestamp>,

    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,

    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,

    /// Unique identifier for the mandate or subscription.
    pub reference: String,

    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: Timestamp,

    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<Vec<CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus>,

    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV).
    ///
    /// This value is 20 bytes, base64-encoded into a 28-character string. (Most 3D Secure providers will return the base64-encoded version, which is what you should specify here.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,

    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,

    /// Network specific 3DS fields.
    ///
    /// Network specific arguments require an explicit card brand choice.
    /// The parameter `payment_method_options.card.network`` must be populated accordingly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions>,

    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS.
    ///
    /// A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<String>,

    /// For 3D Secure 1, the XID.
    ///
    /// For 3D Secure 2, the Directory Server Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        Vec<CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>,
    >,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch:
        Option<Vec<CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    /// The method used to collect offline mandate customer acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Vec<CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<Vec<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor>>,

    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    pub amount: i64,

    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,

    /// Currency in which future payments will be charged.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Timestamp>,

    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,

    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,

    /// Unique identifier for the mandate or subscription.
    pub reference: String,

    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: Timestamp,

    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<Vec<UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure {
    /// The `transStatus` returned from the card Issuer’s ACS in the ARes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ares_trans_status:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus>,

    /// The cryptogram, also known as the "authentication value" (AAV, CAVV or
    /// AEVV).
    ///
    /// This value is 20 bytes, base64-encoded into a 28-character string. (Most 3D Secure providers will return the base64-encoded version, which is what you should specify here.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,

    /// The Electronic Commerce Indicator (ECI) is returned by your 3D Secure
    /// provider and indicates what degree of authentication was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,

    /// Network specific 3DS fields.
    ///
    /// Network specific arguments require an explicit card brand choice.
    /// The parameter `payment_method_options.card.network`` must be populated accordingly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_options:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions>,

    /// The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the
    /// AReq sent to the card Issuer's ACS.
    ///
    /// A string containing 2 digits from 01-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<String>,

    /// For 3D Secure 1, the XID.
    ///
    /// For 3D Secure 2, the Directory Server Transaction ID (dsTransID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,

    /// The version of 3D Secure that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        Vec<UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>,
    >,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch:
        Option<Vec<UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions {
    /// The method used to collect offline mandate customer acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<Vec<UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    /// Cartes Bancaires-specific 3DS fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires:
        Option<CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    /// Cartes Bancaires-specific 3DS fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires:
        Option<UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS
    /// to calculate the Authentication cryptogram.
    ///
    /// Also known as `cavvAlgorithm`. messageExtension: CB-AVALGO.
    pub cb_avalgo:
        CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,

    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// message extension: CB-EXEMPTION; string (4 characters)
    /// This is a 3 byte bitmap (low significant byte first and most significant
    /// bit first) that has been Base64 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<String>,

    /// The risk score returned from Cartes Bancaires in the ARes.
    /// message extension: CB-SCORE; numeric value 0-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires {
    /// The cryptogram calculation algorithm used by the card Issuer's ACS
    /// to calculate the Authentication cryptogram.
    ///
    /// Also known as `cavvAlgorithm`. messageExtension: CB-AVALGO.
    pub cb_avalgo:
        UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,

    /// The exemption indicator returned from Cartes Bancaires in the ARes.
    /// message extension: CB-EXEMPTION; string (4 characters)
    /// This is a 3 byte bitmap (low significant byte first and most significant
    /// bit first) that has been Base64 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_exemption: Option<String>,

    /// The risk score returned from Cartes Bancaires in the ARes.
    /// message extension: CB-SCORE; numeric value 0-99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cb_score: Option<i64>,
}

/// An enum representing the possible values of an `CreateSetupIntentAutomaticPaymentMethods`'s `allow_redirects` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    Always,
    Never,
}

impl CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentAutomaticPaymentMethodsAllowRedirects::Always => "always",
            CreateSetupIntentAutomaticPaymentMethodsAllowRedirects::Never => "never",
        }
    }
}

impl AsRef<str> for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentAutomaticPaymentMethodsAllowRedirects {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `CreateSetupIntent`'s `flow_directions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentFlowDirections {
    Inbound,
    Outbound,
}

impl CreateSetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentFlowDirections::Inbound => "inbound",
            CreateSetupIntentFlowDirections::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for CreateSetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentFlowDirections {
    fn default() -> Self {
        Self::Inbound
    }
}

/// An enum representing the possible values of an `CreateSetupIntentMandateDataCustomerAcceptance`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentMandateDataCustomerAcceptanceType {
    Offline,
    Online,
}

impl CreateSetupIntentMandateDataCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentMandateDataCustomerAcceptanceType::Offline => "offline",
            CreateSetupIntentMandateDataCustomerAcceptanceType::Online => "online",
        }
    }
}

impl AsRef<str> for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn default() -> Self {
        Self::Offline
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataEps`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl CreateSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataEpsBank::ArzteUndApothekerBank => {
                "arzte_und_apotheker_bank"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::AustrianAnadiBankAg => {
                "austrian_anadi_bank_ag"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::BankAustria => "bank_austria",
            CreateSetupIntentPaymentMethodDataEpsBank::BankhausCarlSpangler => {
                "bankhaus_carl_spangler"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::BawagPskAg => "bawag_psk_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::BksBankAg => "bks_bank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::BrullKallmusBankAg => {
                "brull_kallmus_bank_ag"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            CreateSetupIntentPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg => {
                "capital_bank_grawe_gruppe_ag"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::Dolomitenbank => "dolomitenbank",
            CreateSetupIntentPaymentMethodDataEpsBank::EasybankAg => "easybank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::ErsteBankUndSparkassen => {
                "erste_bank_und_sparkassen"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::HypoVorarlbergBankAg => {
                "hypo_vorarlberg_bank_ag"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::MarchfelderBank => "marchfelder_bank",
            CreateSetupIntentPaymentMethodDataEpsBank::OberbankAg => "oberbank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            CreateSetupIntentPaymentMethodDataEpsBank::SchoellerbankAg => "schoellerbank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::SpardaBankWien => "sparda_bank_wien",
            CreateSetupIntentPaymentMethodDataEpsBank::VolksbankGruppe => "volksbank_gruppe",
            CreateSetupIntentPaymentMethodDataEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            CreateSetupIntentPaymentMethodDataEpsBank::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataFpx`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataFpxAccountHolderType::Company => "company",
            CreateSetupIntentPaymentMethodDataFpxAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataFpx`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl CreateSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataFpxBank::AffinBank => "affin_bank",
            CreateSetupIntentPaymentMethodDataFpxBank::Agrobank => "agrobank",
            CreateSetupIntentPaymentMethodDataFpxBank::AllianceBank => "alliance_bank",
            CreateSetupIntentPaymentMethodDataFpxBank::Ambank => "ambank",
            CreateSetupIntentPaymentMethodDataFpxBank::BankIslam => "bank_islam",
            CreateSetupIntentPaymentMethodDataFpxBank::BankMuamalat => "bank_muamalat",
            CreateSetupIntentPaymentMethodDataFpxBank::BankOfChina => "bank_of_china",
            CreateSetupIntentPaymentMethodDataFpxBank::BankRakyat => "bank_rakyat",
            CreateSetupIntentPaymentMethodDataFpxBank::Bsn => "bsn",
            CreateSetupIntentPaymentMethodDataFpxBank::Cimb => "cimb",
            CreateSetupIntentPaymentMethodDataFpxBank::DeutscheBank => "deutsche_bank",
            CreateSetupIntentPaymentMethodDataFpxBank::HongLeongBank => "hong_leong_bank",
            CreateSetupIntentPaymentMethodDataFpxBank::Hsbc => "hsbc",
            CreateSetupIntentPaymentMethodDataFpxBank::Kfh => "kfh",
            CreateSetupIntentPaymentMethodDataFpxBank::Maybank2e => "maybank2e",
            CreateSetupIntentPaymentMethodDataFpxBank::Maybank2u => "maybank2u",
            CreateSetupIntentPaymentMethodDataFpxBank::Ocbc => "ocbc",
            CreateSetupIntentPaymentMethodDataFpxBank::PbEnterprise => "pb_enterprise",
            CreateSetupIntentPaymentMethodDataFpxBank::PublicBank => "public_bank",
            CreateSetupIntentPaymentMethodDataFpxBank::Rhb => "rhb",
            CreateSetupIntentPaymentMethodDataFpxBank::StandardChartered => "standard_chartered",
            CreateSetupIntentPaymentMethodDataFpxBank::Uob => "uob",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataFpxBank {
    fn default() -> Self {
        Self::AffinBank
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

impl CreateSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataIdealBank::AbnAmro => "abn_amro",
            CreateSetupIntentPaymentMethodDataIdealBank::AsnBank => "asn_bank",
            CreateSetupIntentPaymentMethodDataIdealBank::Bunq => "bunq",
            CreateSetupIntentPaymentMethodDataIdealBank::Handelsbanken => "handelsbanken",
            CreateSetupIntentPaymentMethodDataIdealBank::Ing => "ing",
            CreateSetupIntentPaymentMethodDataIdealBank::Knab => "knab",
            CreateSetupIntentPaymentMethodDataIdealBank::Moneyou => "moneyou",
            CreateSetupIntentPaymentMethodDataIdealBank::N26 => "n26",
            CreateSetupIntentPaymentMethodDataIdealBank::Nn => "nn",
            CreateSetupIntentPaymentMethodDataIdealBank::Rabobank => "rabobank",
            CreateSetupIntentPaymentMethodDataIdealBank::Regiobank => "regiobank",
            CreateSetupIntentPaymentMethodDataIdealBank::Revolut => "revolut",
            CreateSetupIntentPaymentMethodDataIdealBank::SnsBank => "sns_bank",
            CreateSetupIntentPaymentMethodDataIdealBank::TriodosBank => "triodos_bank",
            CreateSetupIntentPaymentMethodDataIdealBank::VanLanschot => "van_lanschot",
            CreateSetupIntentPaymentMethodDataIdealBank::Yoursafe => "yoursafe",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataP24`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
}

impl CreateSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataP24Bank::AliorBank => "alior_bank",
            CreateSetupIntentPaymentMethodDataP24Bank::BankMillennium => "bank_millennium",
            CreateSetupIntentPaymentMethodDataP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            CreateSetupIntentPaymentMethodDataP24Bank::BankPekaoSa => "bank_pekao_sa",
            CreateSetupIntentPaymentMethodDataP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            CreateSetupIntentPaymentMethodDataP24Bank::Blik => "blik",
            CreateSetupIntentPaymentMethodDataP24Bank::BnpParibas => "bnp_paribas",
            CreateSetupIntentPaymentMethodDataP24Bank::Boz => "boz",
            CreateSetupIntentPaymentMethodDataP24Bank::CitiHandlowy => "citi_handlowy",
            CreateSetupIntentPaymentMethodDataP24Bank::CreditAgricole => "credit_agricole",
            CreateSetupIntentPaymentMethodDataP24Bank::Envelobank => "envelobank",
            CreateSetupIntentPaymentMethodDataP24Bank::EtransferPocztowy24 => {
                "etransfer_pocztowy24"
            }
            CreateSetupIntentPaymentMethodDataP24Bank::GetinBank => "getin_bank",
            CreateSetupIntentPaymentMethodDataP24Bank::Ideabank => "ideabank",
            CreateSetupIntentPaymentMethodDataP24Bank::Ing => "ing",
            CreateSetupIntentPaymentMethodDataP24Bank::Inteligo => "inteligo",
            CreateSetupIntentPaymentMethodDataP24Bank::MbankMtransfer => "mbank_mtransfer",
            CreateSetupIntentPaymentMethodDataP24Bank::NestPrzelew => "nest_przelew",
            CreateSetupIntentPaymentMethodDataP24Bank::NoblePay => "noble_pay",
            CreateSetupIntentPaymentMethodDataP24Bank::PbacZIpko => "pbac_z_ipko",
            CreateSetupIntentPaymentMethodDataP24Bank::PlusBank => "plus_bank",
            CreateSetupIntentPaymentMethodDataP24Bank::SantanderPrzelew24 => "santander_przelew24",
            CreateSetupIntentPaymentMethodDataP24Bank::TmobileUsbugiBankowe => {
                "tmobile_usbugi_bankowe"
            }
            CreateSetupIntentPaymentMethodDataP24Bank::ToyotaBank => "toyota_bank",
            CreateSetupIntentPaymentMethodDataP24Bank::Velobank => "velobank",
            CreateSetupIntentPaymentMethodDataP24Bank::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataP24Bank {
    fn default() -> Self {
        Self::AliorBank
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataSofort`'s `country` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl CreateSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataSofortCountry::At => "AT",
            CreateSetupIntentPaymentMethodDataSofortCountry::Be => "BE",
            CreateSetupIntentPaymentMethodDataSofortCountry::De => "DE",
            CreateSetupIntentPaymentMethodDataSofortCountry::Es => "ES",
            CreateSetupIntentPaymentMethodDataSofortCountry::It => "IT",
            CreateSetupIntentPaymentMethodDataSofortCountry::Nl => "NL",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn default() -> Self {
        Self::At
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
}

impl CreateSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataType::AcssDebit => "acss_debit",
            CreateSetupIntentPaymentMethodDataType::Affirm => "affirm",
            CreateSetupIntentPaymentMethodDataType::AfterpayClearpay => "afterpay_clearpay",
            CreateSetupIntentPaymentMethodDataType::Alipay => "alipay",
            CreateSetupIntentPaymentMethodDataType::AuBecsDebit => "au_becs_debit",
            CreateSetupIntentPaymentMethodDataType::BacsDebit => "bacs_debit",
            CreateSetupIntentPaymentMethodDataType::Bancontact => "bancontact",
            CreateSetupIntentPaymentMethodDataType::Blik => "blik",
            CreateSetupIntentPaymentMethodDataType::Boleto => "boleto",
            CreateSetupIntentPaymentMethodDataType::Cashapp => "cashapp",
            CreateSetupIntentPaymentMethodDataType::CustomerBalance => "customer_balance",
            CreateSetupIntentPaymentMethodDataType::Eps => "eps",
            CreateSetupIntentPaymentMethodDataType::Fpx => "fpx",
            CreateSetupIntentPaymentMethodDataType::Giropay => "giropay",
            CreateSetupIntentPaymentMethodDataType::Grabpay => "grabpay",
            CreateSetupIntentPaymentMethodDataType::Ideal => "ideal",
            CreateSetupIntentPaymentMethodDataType::Klarna => "klarna",
            CreateSetupIntentPaymentMethodDataType::Konbini => "konbini",
            CreateSetupIntentPaymentMethodDataType::Link => "link",
            CreateSetupIntentPaymentMethodDataType::Oxxo => "oxxo",
            CreateSetupIntentPaymentMethodDataType::P24 => "p24",
            CreateSetupIntentPaymentMethodDataType::Paynow => "paynow",
            CreateSetupIntentPaymentMethodDataType::Paypal => "paypal",
            CreateSetupIntentPaymentMethodDataType::Pix => "pix",
            CreateSetupIntentPaymentMethodDataType::Promptpay => "promptpay",
            CreateSetupIntentPaymentMethodDataType::RevolutPay => "revolut_pay",
            CreateSetupIntentPaymentMethodDataType::SepaDebit => "sepa_debit",
            CreateSetupIntentPaymentMethodDataType::Sofort => "sofort",
            CreateSetupIntentPaymentMethodDataType::Swish => "swish",
            CreateSetupIntentPaymentMethodDataType::UsBankAccount => "us_bank_account",
            CreateSetupIntentPaymentMethodDataType::WechatPay => "wechat_pay",
            CreateSetupIntentPaymentMethodDataType::Zip => "zip",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataType {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::Company => "company",
            CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::Individual => {
                "individual"
            }
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodDataUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodDataUsBankAccountAccountType::Checking => "checking",
            CreateSetupIntentPaymentMethodDataUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Invoice => "invoice",
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn default() -> Self {
        Self::Invoice
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => {
                "automatic"
            }
            CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn default() -> Self {
        Self::Fixed
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardMandateOptions`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Day => "day",
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Month => "month",
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Sporadic => "sporadic",
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Week => "week",
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardMandateOptions`'s `supported_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::India => "india",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes
{
    fn default() -> Self {
        Self::India
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCard`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl CreateSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Amex => "amex",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Diners => "diners",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Discover => "discover",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Interac => "interac",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Unknown => "unknown",
            CreateSetupIntentPaymentMethodOptionsCardNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn default() -> Self {
        Self::Amex
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardThreeDSecure`'s `ares_trans_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "I")]
    I,
    #[serde(rename = "N")]
    N,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "U")]
    U,
    #[serde(rename = "Y")]
    Y,
}

impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::A => "A",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::C => "C",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::I => "I",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::N => "N",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::R => "R",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::U => "U",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::Y => "Y",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus
{
    fn default() -> Self {
        Self::A
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardThreeDSecure`'s `electronic_commerce_indicator` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    #[serde(rename = "01")]
    V01,
    #[serde(rename = "02")]
    V02,
    #[serde(rename = "05")]
    V05,
    #[serde(rename = "06")]
    V06,
    #[serde(rename = "07")]
    V07,
}

impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V01 => "01",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V02 => "02",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V05 => "05",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V06 => "06",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V07 => "07",
        }
    }
}

impl AsRef<str>
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn default() -> Self {
        Self::V01
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires`'s `cb_avalgo` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    #[serde(rename = "0")]
    V0,
    #[serde(rename = "1")]
    V1,
    #[serde(rename = "2")]
    V2,
    #[serde(rename = "3")]
    V3,
    #[serde(rename = "4")]
    V4,
    #[serde(rename = "A")]
    A,
}

impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V0 => "0",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V1 => "1",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V2 => "2",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V3 => "3",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V4 => "4",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::A => "A",
        }
    }
}

impl AsRef<str>
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn default() -> Self {
        Self::V0
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsCardThreeDSecure`'s `version` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
    V2_2_0,
}

impl CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::V1_0_2 => "1.0.2",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::V2_1_0 => "2.1.0",
            CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::V2_2_0 => "2.2.0",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn default() -> Self {
        Self::V1_0_2
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Balances => "balances",
            CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Ownership => "ownership",
            CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::PaymentMethod => "payment_method",
            CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Transactions,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Balances => "balances",
            CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper => "paper",
        }
    }
}

impl AsRef<str>
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn default() -> Self {
        Self::Paper
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks`'s `requested` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::Ach => "ach",
            CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::UsDomesticWire => {
                "us_domestic_wire"
            }
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `CreateSetupIntentPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => {
                "automatic"
            }
            CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => {
                "instant"
            }
            CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `PaymentFlowsAutomaticPaymentMethodsSetupIntent`'s `allow_redirects` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    Always,
    Never,
}

impl PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::Always => "always",
            PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects::Never => "never",
        }
    }
}

impl AsRef<str> for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `SetupIntent`'s `cancellation_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}

impl SetupIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentCancellationReason::Abandoned => "abandoned",
            SetupIntentCancellationReason::Duplicate => "duplicate",
            SetupIntentCancellationReason::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for SetupIntentCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentCancellationReason {
    fn default() -> Self {
        Self::Abandoned
    }
}

/// An enum representing the possible values of an `SetupIntent`'s `flow_directions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentFlowDirections {
    Inbound,
    Outbound,
}

impl SetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentFlowDirections::Inbound => "inbound",
            SetupIntentFlowDirections::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for SetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentFlowDirections {
    fn default() -> Self {
        Self::Inbound
    }
}

/// An enum representing the possible values of an `SetupIntentNextActionVerifyWithMicrodeposits`'s `microdeposit_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}

impl SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::Amounts => "amounts",
            SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::DescriptorCode => {
                "descriptor_code"
            }
        }
    }
}

impl AsRef<str> for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn default() -> Self {
        Self::Amounts
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn default() -> Self {
        Self::Fixed
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsCardMandateOptions`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Day => "day",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Month => "month",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Sporadic => "sporadic",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Week => "week",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Year => "year",
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsCardMandateOptions`'s `supported_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::India => "india",
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn default() -> Self {
        Self::India
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsCard`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl SetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsCardNetwork::Amex => "amex",
            SetupIntentPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            SetupIntentPaymentMethodOptionsCardNetwork::Diners => "diners",
            SetupIntentPaymentMethodOptionsCardNetwork::Discover => "discover",
            SetupIntentPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            SetupIntentPaymentMethodOptionsCardNetwork::Interac => "interac",
            SetupIntentPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            SetupIntentPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            SetupIntentPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            SetupIntentPaymentMethodOptionsCardNetwork::Unknown => "unknown",
            SetupIntentPaymentMethodOptionsCardNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsCardNetwork {
    fn default() -> Self {
        Self::Amex
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    Invoice,
    Subscription,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::Invoice => "invoice",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::Subscription => {
                "subscription"
            }
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn default() -> Self {
        Self::Invoice
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Combined => {
                "combined"
            }
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Interval => {
                "interval"
            }
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Sporadic => {
                "sporadic"
            }
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Business => {
                "business"
            }
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Personal => {
                "personal"
            }
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `SetupIntentPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => {
                "automatic"
            }
            SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `SetupIntent`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

impl SetupIntentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupIntentStatus::Canceled => "canceled",
            SetupIntentStatus::Processing => "processing",
            SetupIntentStatus::RequiresAction => "requires_action",
            SetupIntentStatus::RequiresConfirmation => "requires_confirmation",
            SetupIntentStatus::RequiresPaymentMethod => "requires_payment_method",
            SetupIntentStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for SetupIntentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupIntentStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `UpdateSetupIntent`'s `flow_directions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentFlowDirections {
    Inbound,
    Outbound,
}

impl UpdateSetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentFlowDirections::Inbound => "inbound",
            UpdateSetupIntentFlowDirections::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentFlowDirections {
    fn default() -> Self {
        Self::Inbound
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataEps`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl UpdateSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataEpsBank::ArzteUndApothekerBank => {
                "arzte_und_apotheker_bank"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::AustrianAnadiBankAg => {
                "austrian_anadi_bank_ag"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::BankAustria => "bank_austria",
            UpdateSetupIntentPaymentMethodDataEpsBank::BankhausCarlSpangler => {
                "bankhaus_carl_spangler"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::BawagPskAg => "bawag_psk_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::BksBankAg => "bks_bank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::BrullKallmusBankAg => {
                "brull_kallmus_bank_ag"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            UpdateSetupIntentPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg => {
                "capital_bank_grawe_gruppe_ag"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::Dolomitenbank => "dolomitenbank",
            UpdateSetupIntentPaymentMethodDataEpsBank::EasybankAg => "easybank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::ErsteBankUndSparkassen => {
                "erste_bank_und_sparkassen"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::HypoVorarlbergBankAg => {
                "hypo_vorarlberg_bank_ag"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::MarchfelderBank => "marchfelder_bank",
            UpdateSetupIntentPaymentMethodDataEpsBank::OberbankAg => "oberbank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            UpdateSetupIntentPaymentMethodDataEpsBank::SchoellerbankAg => "schoellerbank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::SpardaBankWien => "sparda_bank_wien",
            UpdateSetupIntentPaymentMethodDataEpsBank::VolksbankGruppe => "volksbank_gruppe",
            UpdateSetupIntentPaymentMethodDataEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            UpdateSetupIntentPaymentMethodDataEpsBank::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataFpx`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataFpxAccountHolderType::Company => "company",
            UpdateSetupIntentPaymentMethodDataFpxAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataFpx`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl UpdateSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataFpxBank::AffinBank => "affin_bank",
            UpdateSetupIntentPaymentMethodDataFpxBank::Agrobank => "agrobank",
            UpdateSetupIntentPaymentMethodDataFpxBank::AllianceBank => "alliance_bank",
            UpdateSetupIntentPaymentMethodDataFpxBank::Ambank => "ambank",
            UpdateSetupIntentPaymentMethodDataFpxBank::BankIslam => "bank_islam",
            UpdateSetupIntentPaymentMethodDataFpxBank::BankMuamalat => "bank_muamalat",
            UpdateSetupIntentPaymentMethodDataFpxBank::BankOfChina => "bank_of_china",
            UpdateSetupIntentPaymentMethodDataFpxBank::BankRakyat => "bank_rakyat",
            UpdateSetupIntentPaymentMethodDataFpxBank::Bsn => "bsn",
            UpdateSetupIntentPaymentMethodDataFpxBank::Cimb => "cimb",
            UpdateSetupIntentPaymentMethodDataFpxBank::DeutscheBank => "deutsche_bank",
            UpdateSetupIntentPaymentMethodDataFpxBank::HongLeongBank => "hong_leong_bank",
            UpdateSetupIntentPaymentMethodDataFpxBank::Hsbc => "hsbc",
            UpdateSetupIntentPaymentMethodDataFpxBank::Kfh => "kfh",
            UpdateSetupIntentPaymentMethodDataFpxBank::Maybank2e => "maybank2e",
            UpdateSetupIntentPaymentMethodDataFpxBank::Maybank2u => "maybank2u",
            UpdateSetupIntentPaymentMethodDataFpxBank::Ocbc => "ocbc",
            UpdateSetupIntentPaymentMethodDataFpxBank::PbEnterprise => "pb_enterprise",
            UpdateSetupIntentPaymentMethodDataFpxBank::PublicBank => "public_bank",
            UpdateSetupIntentPaymentMethodDataFpxBank::Rhb => "rhb",
            UpdateSetupIntentPaymentMethodDataFpxBank::StandardChartered => "standard_chartered",
            UpdateSetupIntentPaymentMethodDataFpxBank::Uob => "uob",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn default() -> Self {
        Self::AffinBank
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

impl UpdateSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataIdealBank::AbnAmro => "abn_amro",
            UpdateSetupIntentPaymentMethodDataIdealBank::AsnBank => "asn_bank",
            UpdateSetupIntentPaymentMethodDataIdealBank::Bunq => "bunq",
            UpdateSetupIntentPaymentMethodDataIdealBank::Handelsbanken => "handelsbanken",
            UpdateSetupIntentPaymentMethodDataIdealBank::Ing => "ing",
            UpdateSetupIntentPaymentMethodDataIdealBank::Knab => "knab",
            UpdateSetupIntentPaymentMethodDataIdealBank::Moneyou => "moneyou",
            UpdateSetupIntentPaymentMethodDataIdealBank::N26 => "n26",
            UpdateSetupIntentPaymentMethodDataIdealBank::Nn => "nn",
            UpdateSetupIntentPaymentMethodDataIdealBank::Rabobank => "rabobank",
            UpdateSetupIntentPaymentMethodDataIdealBank::Regiobank => "regiobank",
            UpdateSetupIntentPaymentMethodDataIdealBank::Revolut => "revolut",
            UpdateSetupIntentPaymentMethodDataIdealBank::SnsBank => "sns_bank",
            UpdateSetupIntentPaymentMethodDataIdealBank::TriodosBank => "triodos_bank",
            UpdateSetupIntentPaymentMethodDataIdealBank::VanLanschot => "van_lanschot",
            UpdateSetupIntentPaymentMethodDataIdealBank::Yoursafe => "yoursafe",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataP24`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
}

impl UpdateSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataP24Bank::AliorBank => "alior_bank",
            UpdateSetupIntentPaymentMethodDataP24Bank::BankMillennium => "bank_millennium",
            UpdateSetupIntentPaymentMethodDataP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            UpdateSetupIntentPaymentMethodDataP24Bank::BankPekaoSa => "bank_pekao_sa",
            UpdateSetupIntentPaymentMethodDataP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            UpdateSetupIntentPaymentMethodDataP24Bank::Blik => "blik",
            UpdateSetupIntentPaymentMethodDataP24Bank::BnpParibas => "bnp_paribas",
            UpdateSetupIntentPaymentMethodDataP24Bank::Boz => "boz",
            UpdateSetupIntentPaymentMethodDataP24Bank::CitiHandlowy => "citi_handlowy",
            UpdateSetupIntentPaymentMethodDataP24Bank::CreditAgricole => "credit_agricole",
            UpdateSetupIntentPaymentMethodDataP24Bank::Envelobank => "envelobank",
            UpdateSetupIntentPaymentMethodDataP24Bank::EtransferPocztowy24 => {
                "etransfer_pocztowy24"
            }
            UpdateSetupIntentPaymentMethodDataP24Bank::GetinBank => "getin_bank",
            UpdateSetupIntentPaymentMethodDataP24Bank::Ideabank => "ideabank",
            UpdateSetupIntentPaymentMethodDataP24Bank::Ing => "ing",
            UpdateSetupIntentPaymentMethodDataP24Bank::Inteligo => "inteligo",
            UpdateSetupIntentPaymentMethodDataP24Bank::MbankMtransfer => "mbank_mtransfer",
            UpdateSetupIntentPaymentMethodDataP24Bank::NestPrzelew => "nest_przelew",
            UpdateSetupIntentPaymentMethodDataP24Bank::NoblePay => "noble_pay",
            UpdateSetupIntentPaymentMethodDataP24Bank::PbacZIpko => "pbac_z_ipko",
            UpdateSetupIntentPaymentMethodDataP24Bank::PlusBank => "plus_bank",
            UpdateSetupIntentPaymentMethodDataP24Bank::SantanderPrzelew24 => "santander_przelew24",
            UpdateSetupIntentPaymentMethodDataP24Bank::TmobileUsbugiBankowe => {
                "tmobile_usbugi_bankowe"
            }
            UpdateSetupIntentPaymentMethodDataP24Bank::ToyotaBank => "toyota_bank",
            UpdateSetupIntentPaymentMethodDataP24Bank::Velobank => "velobank",
            UpdateSetupIntentPaymentMethodDataP24Bank::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn default() -> Self {
        Self::AliorBank
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataSofort`'s `country` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl UpdateSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataSofortCountry::At => "AT",
            UpdateSetupIntentPaymentMethodDataSofortCountry::Be => "BE",
            UpdateSetupIntentPaymentMethodDataSofortCountry::De => "DE",
            UpdateSetupIntentPaymentMethodDataSofortCountry::Es => "ES",
            UpdateSetupIntentPaymentMethodDataSofortCountry::It => "IT",
            UpdateSetupIntentPaymentMethodDataSofortCountry::Nl => "NL",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn default() -> Self {
        Self::At
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
}

impl UpdateSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataType::AcssDebit => "acss_debit",
            UpdateSetupIntentPaymentMethodDataType::Affirm => "affirm",
            UpdateSetupIntentPaymentMethodDataType::AfterpayClearpay => "afterpay_clearpay",
            UpdateSetupIntentPaymentMethodDataType::Alipay => "alipay",
            UpdateSetupIntentPaymentMethodDataType::AuBecsDebit => "au_becs_debit",
            UpdateSetupIntentPaymentMethodDataType::BacsDebit => "bacs_debit",
            UpdateSetupIntentPaymentMethodDataType::Bancontact => "bancontact",
            UpdateSetupIntentPaymentMethodDataType::Blik => "blik",
            UpdateSetupIntentPaymentMethodDataType::Boleto => "boleto",
            UpdateSetupIntentPaymentMethodDataType::Cashapp => "cashapp",
            UpdateSetupIntentPaymentMethodDataType::CustomerBalance => "customer_balance",
            UpdateSetupIntentPaymentMethodDataType::Eps => "eps",
            UpdateSetupIntentPaymentMethodDataType::Fpx => "fpx",
            UpdateSetupIntentPaymentMethodDataType::Giropay => "giropay",
            UpdateSetupIntentPaymentMethodDataType::Grabpay => "grabpay",
            UpdateSetupIntentPaymentMethodDataType::Ideal => "ideal",
            UpdateSetupIntentPaymentMethodDataType::Klarna => "klarna",
            UpdateSetupIntentPaymentMethodDataType::Konbini => "konbini",
            UpdateSetupIntentPaymentMethodDataType::Link => "link",
            UpdateSetupIntentPaymentMethodDataType::Oxxo => "oxxo",
            UpdateSetupIntentPaymentMethodDataType::P24 => "p24",
            UpdateSetupIntentPaymentMethodDataType::Paynow => "paynow",
            UpdateSetupIntentPaymentMethodDataType::Paypal => "paypal",
            UpdateSetupIntentPaymentMethodDataType::Pix => "pix",
            UpdateSetupIntentPaymentMethodDataType::Promptpay => "promptpay",
            UpdateSetupIntentPaymentMethodDataType::RevolutPay => "revolut_pay",
            UpdateSetupIntentPaymentMethodDataType::SepaDebit => "sepa_debit",
            UpdateSetupIntentPaymentMethodDataType::Sofort => "sofort",
            UpdateSetupIntentPaymentMethodDataType::Swish => "swish",
            UpdateSetupIntentPaymentMethodDataType::UsBankAccount => "us_bank_account",
            UpdateSetupIntentPaymentMethodDataType::WechatPay => "wechat_pay",
            UpdateSetupIntentPaymentMethodDataType::Zip => "zip",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataType {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::Company => "company",
            UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType::Individual => {
                "individual"
            }
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodDataUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType::Checking => "checking",
            UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Invoice => "invoice",
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn default() -> Self {
        Self::Invoice
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => {
                "automatic"
            }
            UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn default() -> Self {
        Self::Fixed
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardMandateOptions`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Day => "day",
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Month => "month",
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Sporadic => "sporadic",
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Week => "week",
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardMandateOptions`'s `supported_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes::India => "india",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes
{
    fn default() -> Self {
        Self::India
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCard`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Amex => "amex",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Diners => "diners",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Discover => "discover",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Interac => "interac",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Unknown => "unknown",
            UpdateSetupIntentPaymentMethodOptionsCardNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn default() -> Self {
        Self::Amex
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure`'s `ares_trans_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "I")]
    I,
    #[serde(rename = "N")]
    N,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "U")]
    U,
    #[serde(rename = "Y")]
    Y,
}

impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::A => "A",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::C => "C",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::I => "I",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::N => "N",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::R => "R",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::U => "U",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus::Y => "Y",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureAresTransStatus
{
    fn default() -> Self {
        Self::A
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure`'s `electronic_commerce_indicator` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    #[serde(rename = "01")]
    V01,
    #[serde(rename = "02")]
    V02,
    #[serde(rename = "05")]
    V05,
    #[serde(rename = "06")]
    V06,
    #[serde(rename = "07")]
    V07,
}

impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V01 => "01",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V02 => "02",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V05 => "05",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V06 => "06",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::V07 => "07",
        }
    }
}

impl AsRef<str>
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn default() -> Self {
        Self::V01
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires`'s `cb_avalgo` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    #[serde(rename = "0")]
    V0,
    #[serde(rename = "1")]
    V1,
    #[serde(rename = "2")]
    V2,
    #[serde(rename = "3")]
    V3,
    #[serde(rename = "4")]
    V4,
    #[serde(rename = "A")]
    A,
}

impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V0 => "0",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V1 => "1",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V2 => "2",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V3 => "3",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::V4 => "4",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::A => "A",
        }
    }
}

impl AsRef<str>
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn default() -> Self {
        Self::V0
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsCardThreeDSecure`'s `version` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
    V2_2_0,
}

impl UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::V1_0_2 => "1.0.2",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::V2_1_0 => "2.1.0",
            UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion::V2_2_0 => "2.2.0",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsCardThreeDSecureVersion {
    fn default() -> Self {
        Self::V1_0_2
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Balances => "balances",
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Ownership => "ownership",
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::PaymentMethod => "payment_method",
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Transactions,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Balances => "balances",
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptions`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper => "paper",
        }
    }
}

impl AsRef<str>
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn default() -> Self {
        Self::Paper
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks`'s `requested` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::Ach => "ach",
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested::UsDomesticWire => {
                "us_domestic_wire"
            }
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `UpdateSetupIntentPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => {
                "automatic"
            }
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => {
                "instant"
            }
            UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}
