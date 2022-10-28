use crate::msg::{ExtensionExecuteMsg, ExtensionQueryMsg};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Empty, Uint128};
use cw20::{
    AllAccountsResponse, AllAllowancesResponse, AllowanceResponse, BalanceResponse,
    DownloadLogoResponse, MarketingInfoResponse, MinterResponse, TokenInfoResponse,
};
use cw20::{Cw20Coin, Expiration, Logo};

#[cw_serde]
pub enum Cw4626ExecuteMsg<T = ExtensionExecuteMsg, S = Empty> {
    //--------------------------------------------------------------------------------------------------
    // Standard CW20 ExecuteMsgs
    //--------------------------------------------------------------------------------------------------
    /// Transfer is a base message to move tokens to another account without triggering actions
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    /// Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Transfers amount tokens from owner -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Only with "approval" extension. Sends amount tokens from owner -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),
    //--------------------------------------------------------------------------------------------------
    // CW4626 ExecuteMsgs
    //--------------------------------------------------------------------------------------------------
    Deposit {
        /// The amount of base tokens to deposit
        amount: Uint128,
        /// An optional field containing the recipient of the vault token. If not set, the
        /// caller address will be used instead.
        recipient: Option<String>,
    },

    Redeem {
        /// Amount of vault tokens to redeem
        amount: Uint128,
        /// An optional field containing which address should receive the withdrawn base tokens.
        /// If not set, the caller address will be used instead.
        recipient: Option<String>,
    },

    Callback(S),

    VaultExtension(T),
}

#[cw_serde]
pub enum Cw4626QueryMsg<T = ExtensionQueryMsg> {
    //--------------------------------------------------------------------------------------------------
    // Standard CW20 QueryMsgs
    //--------------------------------------------------------------------------------------------------
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    #[returns(BalanceResponse)]
    Balance { address: String },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    /// Return type: MinterResponse.
    #[returns(MinterResponse)]
    Minter {},
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse.
    #[returns(MarketingInfoResponse)]
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data stored for
    /// this contract.
    /// Return type: DownloadLogoResponse.
    #[returns(DownloadLogoResponse)]
    DownloadLogo {},
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    #[returns(AllAllowancesResponse)]
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.
    #[returns(AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns `VaultStandardInfo` with information on the version of the vault
    /// standard used as well as any enabled extensions.
    #[returns(VaultStandardInfo)]
    VaultStandardInfo {},

    /// Returns `VaultInfo` representing vault requirements, lockup, & vault
    /// token denom.
    #[returns(VaultInfo)]
    Info {},

    /// Returns `Uint128` amount of vault tokens that will be returned for the
    /// passed in assets.
    ///
    /// Allows an on-chain or off-chain user to simulate the effects of their
    /// deposit at the current block, given current on-chain conditions.
    ///
    /// MUST return as close to and no more than the exact amount of Vault
    /// shares that would be minted in a deposit call in the same transaction.
    /// I.e. deposit should return the same or more shares as previewDeposit if
    /// called in the same transaction.
    ///
    /// MUST NOT account for deposit limits like those returned from maxDeposit
    /// and should always act as though the deposit would be accepted,
    /// regardless if the user has enough tokens approved, etc.
    ///
    /// MUST be inclusive of deposit fees. Integrators should be aware of the
    /// existence of deposit fees.
    #[returns(Uint128)]
    PreviewDeposit { amount: Uint128 },

    /// Returns the number of base tokens that would be redeemed in exchange
    /// `amount` for vault tokens. Used by Rover to calculate vault position values.
    #[returns(Uint128)]
    PreviewRedeem { amount: Uint128 },

    /// Returns `Option<Uint128>`, the maximum amount of base tokens that can be
    /// deposited into the Vault for the `recipient`, through a call to Deposit.
    ///
    /// MUST return the maximum amount of base tokens that deposit would
    /// allow to be deposited for `recipient` and not cause a revert, which MUST NOT be higher
    /// than the actual maximum that would be accepted (it should underestimate
    /// if necessary). This assumes that the user has infinite assets, i.e.
    /// MUST NOT rely on the asset balances of `recipient`.
    ///
    /// MUST factor in both global and user-specific limits, like if deposits
    /// are entirely disabled (even temporarily) it MUST return 0.
    #[returns(Option<Uint128>)]
    MaxDeposit { recipient: String },

    /// Returns `Option<Uint128>` maximum amount of Vault shares that can be redeemed
    /// from the owner balance in the Vault, through a call to Withdraw
    ///
    /// TODO: Keep this? Could potentially be combined with MaxWithdraw to return
    /// a MaxWithdrawResponse type that includes both max assets that can be
    /// withdrawn as well as max vault shares that can be withdrawn in exchange
    /// for assets.
    #[returns(Option<Uint128>)]
    MaxRedeem { owner: String },

    /// Returns the amount of assets managed by the vault denominated in base tokens.
    /// Useful for display purposes, and does not have to confer the exact
    /// amount of base tokens.
    #[returns(Uint128)]
    TotalAssets {},

    /// Returns `Uint128` total amount of vault tokens in circulation.
    #[returns(Uint128)]
    TotalVaultTokenSupply {},

    /// The amount of shares that the vault would exchange for the amount of
    /// assets provided, in an ideal scenario where all the conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of shares returned by the vault if the passed in assets were deposited.
    /// This calculation may not reflect the “per-user” price-per-share, and
    /// instead should reflect the “average-user’s” price-per-share, meaning
    /// what the average user should expect to see when exchanging to and from.
    #[returns(Uint128)]
    ConvertToShares { amount: Uint128 },

    /// Returns the amount of base tokens that the Vault would exchange for
    /// the `amount` of shares provided, in an ideal scenario where all the
    /// conditions are met.
    ///
    /// Useful for display purposes and does not have to confer the exact amount
    /// of assets returned by the vault if the passed in shares were withdrawn.
    /// This calculation may not reflect the “per-user” price-per-share, and
    /// instead should reflect the “average-user’s” price-per-share, meaning
    /// what the average user should expect to see when exchanging to and from.
    #[returns(Uint128)]
    ConvertToAssets { amount: Uint128 },

    /// TODO: How to handle return derive? We must supply a type here, but we
    /// don't know it.
    #[returns(Empty)]
    VaultExtension(T),
}
