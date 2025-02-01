// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::AccountId;
use crate::params::{Deleted, Expand, Metadata, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Account".
///
/// For more details see <https://stripe.com/docs/api/accounts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Account {
    /// Unique identifier for the object.
    pub id: AccountId,

    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Time at which the account was connected.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Whether account details have been submitted.
    ///
    /// Accounts with Stripe Dashboard access, which includes Standard accounts, cannot receive payouts before this is true.
    /// Accounts where this is false should be directed to [an onboarding flow](/connect/onboarding) to finish submitting account details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,

    /// An email address associated with the account.
    ///
    /// It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Whether the funds in this account can be paid out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,

    /// The Stripe account type.
    ///
    /// Can be `standard`, `express`, `custom`, or `none`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}

impl Account {
    /// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
    /// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
    ///
    /// If you’ve already collected information for your connected accounts, you [can prefill that information](https://stripe.com/docs/connect/best-practices#onboarding) when
    /// creating the account.
    ///
    /// Connect Onboarding won’t ask for the prefilled information during account onboarding. You can prefill any information on the account.
    pub fn create(client: &Client, params: CreateAccount<'_>) -> Response<Account> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/accounts", &params)
    }

    /// Retrieves the details of an account.
    pub fn retrieve(client: &Client, id: &AccountId, expand: &[&str]) -> Response<Account> {
        client.get_query(&format!("/accounts/{}", id), Expand { expand })
    }

    /// With <a href="/connect">Connect</a>, you can delete accounts you manage.
    ///
    /// Test-mode accounts can be deleted at any time.
    ///
    /// Live-mode accounts where Stripe is responsible for negative account balances cannot be deleted, which includes Standard accounts.
    ///
    /// Live-mode accounts where your platform is liable for negative account balances, which includes Custom and Express accounts, can be deleted when all <a href="/api/balance/balance_object">balances</a> are zero.  If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/settings/account) instead.
    pub fn delete(client: &Client, id: &AccountId) -> Response<Deleted<AccountId>> {
        client.delete(&format!("/accounts/{}", id))
    }
}

impl Object for Account {
    type Id = AccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "account"
    }
}

/// The parameters for `Account::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateAccount<'a> {
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,

    /// A hash of configuration describing the account controller's attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<CreateAccountController>,

    /// The email address of the account holder.
    ///
    /// This is only to make the account easier to identify to you.
    /// If [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts, Stripe doesn't email the account without your consent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A card or bank account to attach to the account for receiving [payouts](/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    ///
    /// You can provide either a token, like the ones returned by [Stripe.js](/js), or a dictionary, as documented in the `external_account` parameter for [bank account](/api#account_create_bank_account) creation.
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](/api#account_create_bank_account) or [card creation](/api#account_create_card) APIs.
    /// After you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The type of Stripe account to create.
    ///
    /// May be one of `custom`, `express` or `standard`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}

impl<'a> CreateAccount<'a> {
    pub fn new() -> Self {
        CreateAccount {
            account_token: Default::default(),
            controller: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            external_account: Default::default(),
            metadata: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountControllerFees {
    /// A value indicating the responsible payer of Stripe fees on this account.
    ///
    /// Defaults to `account`.
    /// Learn more about [fee behavior on connected accounts](https://docs.stripe.com/connect/direct-charges-fee-payer-behavior).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<CreateAccountControllerFeesPayer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountControllerLosses {
    /// A value indicating who is liable when this account can't pay back negative balances resulting from payments.
    ///
    /// Defaults to `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountControllerLossesPayments>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountControllerStripeDashboard {
    /// Whether this account should have access to the full Stripe Dashboard (`full`), to the Express Dashboard (`express`), or to no Stripe-hosted dashboard (`none`).
    ///
    /// Defaults to `full`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateAccountControllerStripeDashboardType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountController {
    /// A hash of configuration for who pays Stripe fees for product usage on this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<CreateAccountControllerFees>,

    /// A hash of configuration for products that have negative balance liability, and whether Stripe or a Connect application is responsible for them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub losses: Option<CreateAccountControllerLosses>,

    /// A value indicating responsibility for collecting updated information when requirements on the account are due or change.
    ///
    /// Defaults to `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_collection: Option<CreateAccountControllerRequirementCollection>,

    /// A hash of configuration for Stripe-hosted dashboards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_dashboard: Option<CreateAccountControllerStripeDashboard>,
}

/// An enum representing the possible values of an `CreateAccount`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    None,
    Custom,
    Express,
    Standard,
}

impl AccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountType::Custom => "custom",
            AccountType::Express => "express",
            AccountType::Standard => "standard",
            AccountType::None => "none",
        }
    }
}

impl AsRef<str> for AccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountType {
    fn default() -> Self {
        Self::Custom
    }
}

/// An enum representing the possible values of an `AccountUnificationAccountControllerFees`'s `payer` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerFeesPayer {
    Account,
    Application,
    ApplicationCustom,
    ApplicationExpress,
}

impl AccountUnificationAccountControllerFeesPayer {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountUnificationAccountControllerFeesPayer::Account => "account",
            AccountUnificationAccountControllerFeesPayer::Application => "application",
            AccountUnificationAccountControllerFeesPayer::ApplicationCustom => "application_custom",
            AccountUnificationAccountControllerFeesPayer::ApplicationExpress => {
                "application_express"
            }
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerFeesPayer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountUnificationAccountControllerFeesPayer {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `AccountUnificationAccountControllerLosses`'s `payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerLossesPayments {
    Application,
    Stripe,
}

impl AccountUnificationAccountControllerLossesPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountUnificationAccountControllerLossesPayments::Application => "application",
            AccountUnificationAccountControllerLossesPayments::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerLossesPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountUnificationAccountControllerLossesPayments {
    fn default() -> Self {
        Self::Application
    }
}

/// An enum representing the possible values of an `AccountUnificationAccountController`'s `requirement_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerRequirementCollection {
    Application,
    Stripe,
}

impl AccountUnificationAccountControllerRequirementCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountUnificationAccountControllerRequirementCollection::Application => "application",
            AccountUnificationAccountControllerRequirementCollection::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerRequirementCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountUnificationAccountControllerRequirementCollection {
    fn default() -> Self {
        Self::Application
    }
}

/// An enum representing the possible values of an `AccountUnificationAccountControllerStripeDashboard`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerStripeDashboardType {
    Express,
    Full,
    None,
}

impl AccountUnificationAccountControllerStripeDashboardType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountUnificationAccountControllerStripeDashboardType::Express => "express",
            AccountUnificationAccountControllerStripeDashboardType::Full => "full",
            AccountUnificationAccountControllerStripeDashboardType::None => "none",
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerStripeDashboardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountUnificationAccountControllerStripeDashboardType {
    fn default() -> Self {
        Self::Express
    }
}

/// An enum representing the possible values of an `AccountUnificationAccountController`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}

impl AccountUnificationAccountControllerType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountUnificationAccountControllerType::Account => "account",
            AccountUnificationAccountControllerType::Application => "application",
        }
    }
}

impl AsRef<str> for AccountUnificationAccountControllerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AccountUnificationAccountControllerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateAccountControllerFees`'s `payer` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountControllerFeesPayer {
    Account,
    Application,
}

impl CreateAccountControllerFeesPayer {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateAccountControllerFeesPayer::Account => "account",
            CreateAccountControllerFeesPayer::Application => "application",
        }
    }
}

impl AsRef<str> for CreateAccountControllerFeesPayer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateAccountControllerFeesPayer {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateAccountControllerLosses`'s `payments` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountControllerLossesPayments {
    Application,
    Stripe,
}

impl CreateAccountControllerLossesPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateAccountControllerLossesPayments::Application => "application",
            CreateAccountControllerLossesPayments::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for CreateAccountControllerLossesPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateAccountControllerLossesPayments {
    fn default() -> Self {
        Self::Application
    }
}

/// An enum representing the possible values of an `CreateAccountController`'s `requirement_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountControllerRequirementCollection {
    Application,
    Stripe,
}

impl CreateAccountControllerRequirementCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateAccountControllerRequirementCollection::Application => "application",
            CreateAccountControllerRequirementCollection::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for CreateAccountControllerRequirementCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateAccountControllerRequirementCollection {
    fn default() -> Self {
        Self::Application
    }
}

/// An enum representing the possible values of an `CreateAccountControllerStripeDashboard`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountControllerStripeDashboardType {
    Express,
    Full,
    None,
}

impl CreateAccountControllerStripeDashboardType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateAccountControllerStripeDashboardType::Express => "express",
            CreateAccountControllerStripeDashboardType::Full => "full",
            CreateAccountControllerStripeDashboardType::None => "none",
        }
    }
}

impl AsRef<str> for CreateAccountControllerStripeDashboardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
