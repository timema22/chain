//! Mocks for the vesting module.

#![cfg(test)]

use super::*;
use crate::{self as vesting};
use frame_support::{ord_parameter_types, parameter_types};
use frame_system::EnsureSignedBy;
use pallet_balances;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        PalletBalances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        Vesting: vesting::{Pallet, Call, Storage, Event<T>, Config<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

pub type AccountId = u128;
impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = frame_support::traits::Everything;
    type OnSetCode = ();
    type SystemWeightInfo = ();
}

type Balance = u64;

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = MaxLocks;
    type AccountStore = frame_system::Pallet<Test>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

ord_parameter_types! {
    pub const CancelOrigin: AccountId = 42;
    pub const ForceOrigin: AccountId = 43;
}

impl Config for Test {
    type Event = Event;
    type Currency = PalletBalances;
    type CancelOrigin = EnsureSignedBy<CancelOrigin, AccountId>;
    type ForceOrigin = EnsureSignedBy<ForceOrigin, AccountId>;
    type WeightInfo = ();
    type BlockNumberProvider = frame_system::Pallet<Test>;
}

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

pub struct ExtBuilder {
    endowed_accounts: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![],
        }
    }
}

impl ExtBuilder {
    pub fn balances(mut self, endowed_accounts: Vec<(AccountId, Balance)>) -> Self {
        self.endowed_accounts = endowed_accounts;
        self
    }

    pub fn one_hundred_for_alice(self) -> Self {
        self.balances(vec![(ALICE, 100)])
    }

    pub fn build(self) -> sp_io::TestExternalities {
        sp_tracing::try_init_simple();

        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self
                .endowed_accounts
                .into_iter()
                .map(|(account_id, initial_balance)| (account_id, initial_balance))
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}
