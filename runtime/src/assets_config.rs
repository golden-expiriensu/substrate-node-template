use super::{Runtime, Event, Balance, Balances, AccountId, parameter_types};

pub const FINNEY: Balance = 1_000_000_000_000_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance + bytes as Balance) * FINNEY / 1_000_000
}

// stole from https://github.com/AstarNetwork/Astar/blob/master/runtime/astar/src/lib.rs
parameter_types! {
    pub const AssetDeposit: Balance = 1_000_000;
    pub const ApprovalDeposit: Balance = 1_000_000;
    pub const AssetsStringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = deposit(1, 68);
    pub const MetadataDepositPerByte: Balance = deposit(0, 1);
    pub const AssetAccountDeposit: Balance = deposit(1, 18);
}

impl pallet_assets::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type AssetId = u128;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = AssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}