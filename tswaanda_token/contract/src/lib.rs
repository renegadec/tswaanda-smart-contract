use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

const DATA_IMAGE_SVG_GT_ICON: &str = "https://tswaanda.com/assets/logo-68d8236b.png";

pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub accounts: LookupMap<AccountId, Balance>,

    pub total_supply: Balance,

    pub bytes_for_longest_account_id: StorageUsage,

    pub metadata: LazyOption<FungibleTokenMetadata>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Tswaanda Token".to_string(),
                symbol: "TSWT".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        let mut this = Self {
            total_supply: total_supply.0,
            bytes_for_longest_account_id: 0,
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        this.measure_bytes_for_longest_account_id();

        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        this
    }
}