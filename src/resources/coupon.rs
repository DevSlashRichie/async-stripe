// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::CouponId;
use crate::params::{Deleted, Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::Currency;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Coupon".
///
/// For more details see [https://stripe.com/docs/api/coupons/object](https://stripe.com/docs/api/coupons/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coupon {
    /// Unique identifier for the object.
    pub id: CouponId,

    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// One of `forever`, `once`, and `repeating`.
    ///
    /// Describes how long a customer who applies this coupon will get the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<CouponDuration>,

    /// If `duration` is `repeating`, the number of months the coupon applies.
    ///
    /// Null if coupon `duration` is `forever` or `once`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Name of the coupon displayed to customers on for instance invoices or receipts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon.
    ///
    /// For example, a coupon with percent_off of 50 will make a %s100 invoice %s50 instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,

    /// Date after which the coupon can no longer be redeemed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<Timestamp>,

    /// Number of times this coupon has been applied to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times_redeemed: Option<i64>,

    /// Taking account of the above properties, whether this coupon can still be applied to a customer.
    #[serde(default)]
    pub valid: bool,
}

impl Coupon {
    /// Returns a list of your coupons.
    pub fn list(client: &Client, params: CouponListParams<'_>) -> Response<List<Coupon>> {
        client.get_query("/coupons", &params)
    }

    /// Retrieves the coupon with the given ID.
    pub fn retrieve(client: &Client, id: &CouponId, expand: &[&str]) -> Response<Coupon> {
        client.get_query(&format!("/coupons/{}", id), &Expand { expand })
    }

    /// Retrieves the coupon with the given ID.
    pub fn delete(client: &Client, id: &CouponId) -> Response<Deleted<CouponId>> {
        client.delete(&format!("/coupons/{}", id))
    }
}

impl Object for Coupon {
    type Id = CouponId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "coupon"
    }
}

/// The parameters for `Coupon::list`.
#[derive(Clone, Debug, Serialize)]
pub struct CouponListParams<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a CouponId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a CouponId>,
}

impl<'a> CouponListParams<'a> {
    pub fn new() -> Self {
        CouponListParams {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `Coupon`'s `duration` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
}
