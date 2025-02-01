// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::params::{Deleted, Expand, Metadata, Object, Timestamp};
// use crate::resources::{
//     Address, CashBalance, Currency, Discount, InvoiceSettingRenderingOptions, PaymentMethod,
//     PaymentSource, PaymentSourceParams, Shipping, Subscription, TaxId, TestHelpersTestClock,
// };

/// The resource representing a Stripe "Customer".
///
/// For more details see <https://stripe.com/docs/api/customers/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer {
    /// Unique identifier for the object.
    pub id: CustomerId,

    /// The current balance, if any, that's stored on the customer.
    ///
    /// If negative, the customer has credit to apply to their next invoice.
    /// If positive, the customer has an amount owed that's added to their next invoice.
    /// The balance only considers amounts that Stripe hasn't successfully applied to any invoice.
    /// It doesn't reflect unpaid invoices.
    /// This balance is only taken into account after invoices finalize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,
    /// ID of the default payment source for the customer.

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Tracks the most recent state change on any invoice belonging to the customer.
    ///
    /// Paying an invoice or marking it uncollectible via the API will set this field to false.
    /// An automatic payment failure or passing the `invoice.due_date` will set this field to `true`.  If an invoice becomes uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't reset to `false`.  If you care whether the customer has paid their most recent subscription invoice, use `subscription.status` instead.
    /// Paying or marking uncollectible any customer invoice regardless of whether it is the latest invoice for a subscription will always set this field to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The customer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The current multi-currency balances, if any, that's stored on the customer.
    ///
    /// If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency.
    /// If negative, the customer has an amount owed that's added to their next invoice denominated in that currency.
    /// These balances don't apply to unpaid invoices.
    /// They solely track amounts that Stripe hasn't successfully applied to any invoice.
    /// Stripe only applies a balance in a specific currency to an invoice after that invoice (which is in the same currency) finalizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<i64>,

    /// The prefix for the customer used to generate unique invoice numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The suffix of the customer's next invoice number (for example, 0001).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The customer's preferred locales (languages), ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
}

impl Customer {
    /// Returns a list of your customers.
    ///
    /// The customers are returned sorted by creation date, with the most recent customers appearing first.
    /// Creates a new customer object.
    pub fn create(client: &Client, params: CreateCustomer<'_>) -> Response<Customer> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/customers", &params)
    }

    /// Retrieves a Customer object.
    pub fn retrieve(client: &Client, id: &CustomerId, expand: &[&str]) -> Response<Customer> {
        client.get_query(&format!("/customers/{}", id), Expand { expand })
    }

    /// Permanently deletes a customer.
    ///
    /// It cannot be undone.
    /// Also immediately cancels any active subscriptions on the customer.
    pub fn delete(client: &Client, id: &CustomerId) -> Response<Deleted<CustomerId>> {
        client.delete(&format!("/customers/{}", id))
    }
}

impl Object for Customer {
    type Id = CustomerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "customer"
    }
}

/// The parameters for `Customer::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateCustomer<'a> {
    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// The sequence to be used on the customer's next invoice.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,

    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

impl<'a> CreateCustomer<'a> {
    pub fn new() -> Self {
        CreateCustomer {
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            invoice_prefix: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            next_invoice_sequence: Default::default(),
            payment_method: Default::default(),
            phone: Default::default(),
            preferred_locales: Default::default(),
            validate: Default::default(),
        }
    }
}
