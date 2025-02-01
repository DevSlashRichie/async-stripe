// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{ChargeId, CustomerId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{
    Account, Address, Application, ApplicationFee, BillingDetails, ChargeSourceParams, Currency,
    Customer, PaymentIntent, PaymentMethod, PaymentMethodDetailsCardInstallmentsPlan,
    PaymentMethodDetailsCardWalletApplePay, PaymentMethodDetailsCardWalletGooglePay, PaymentSource,
    Refund,
};

/// The resource representing a Stripe "Charge".
///
/// For more details see <https://stripe.com/docs/api/charges/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Charge {
    /// Unique identifier for the object.
    pub id: ChargeId,

    /// Amount intended to be collected by this payment.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,

    /// Amount in cents (or local equivalent) captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,

    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that created the charge.
    pub application: Option<Expandable<Application>>,

    /// The application fee (if any) for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee: Option<Expandable<ApplicationFee>>,

    /// The amount of the application fee (if any) requested for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee_amount: Option<i64>,

    /// Authorization code on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,

    pub billing_details: BillingDetails,

    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    ///
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    pub calculated_statement_descriptor: Option<String>,

    /// If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// ID of the customer this charge is for if one exists.
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Whether the charge has been disputed.
    pub disputed: bool,

    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/error-codes) for a list of codes).
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for charge failure if available.
    pub failure_message: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub on_behalf_of: Option<Expandable<Account>>,

    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,

    /// ID of the PaymentIntent associated with this charge, if one exists.
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// ID of the payment method used in this charge.
    pub payment_method: Option<String>,

    /// This is the email address that the receipt for this charge was sent to.
    pub receipt_email: Option<String>,

    /// This is the transaction number that appears on email receipts sent for this charge.
    ///
    /// This attribute will be `null` until a receipt has been sent.
    pub receipt_number: Option<String>,

    /// This is the URL to view the receipt for this charge.
    ///
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: Option<String>,

    /// Whether the charge has been fully refunded.
    ///
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the charge.
    pub refunds: Option<List<Refund>>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,

    /// Details about the payment method at the time of the transaction.
    pub payment_method_details: Option<PaymentMethodDetails>,
}

impl Charge {
    /// Retrieves the details of a charge that has previously been created.
    ///
    /// Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information.
    /// The same information is returned when creating or refunding the charge.
    pub fn retrieve(client: &Client, id: &ChargeId, expand: &[&str]) -> Response<Charge> {
        client.get_query(&format!("/charges/{}", id), Expand { expand })
    }
}

impl Object for Charge {
    type Id = ChargeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "charge"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodDetailsCard>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCard {
    /// When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<Timestamp>,
}
