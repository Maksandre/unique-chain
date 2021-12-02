#![cfg(test)]
#![allow(clippy::from_over_into)]
use crate as pallet_inflation;

use frame_support::{
	traits::{Currency},
	parameter_types,
};
use frame_support::{
	traits::{OnInitialize, Everything},
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, BlockNumberProvider, IdentityLookup},
	testing::Header,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

const YEAR: u64 = 2_629_800;

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = u64;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		Balances: pallet_balances::{Pallet, Call, Storage},
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Inflation: pallet_inflation::{Pallet, Call, Storage},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub TreasuryAccountId: u64 = 1234;
	pub const InflationBlockInterval: u32 = 100; // every time per how many blocks inflation is applied
	pub static MockBlockNumberProvider: u64 = 0;
}

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u64;

	fn current_block_number() -> Self::BlockNumber {
		Self::get()
	}
}

impl pallet_inflation::Config for Test {
	type Currency = Balances;
	type TreasuryAccountId = TreasuryAccountId;
	type InflationBlockInterval = InflationBlockInterval;
	type BlockNumberProvider = MockBlockNumberProvider;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

#[test]
fn inflation_works() {
	new_test_ext().execute_with(|| {
		// Total issuance = 1_000_000_000
		let initial_issuance: u64 = 1_000_000_000;
		let _ = <Balances as Currency<_>>::deposit_creating(&1234, initial_issuance);
		assert_eq!(Balances::free_balance(1234), initial_issuance);

		// BlockInflation should be set after 1st block and
		// first inflation deposit should be equal to BlockInflation
		MockBlockNumberProvider::set(1);
		Inflation::on_initialize(0);

		// Expected 100-block inflation for year 1 is 100 * 100_000_000 / YEAR = 3803
		assert_eq!(Inflation::block_inflation(), 3803);
		assert_eq!(
			Balances::free_balance(1234) - initial_issuance,
			Inflation::block_inflation()
		);
	});
}

#[test]
fn inflation_second_deposit() {
	new_test_ext().execute_with(|| {
		// Total issuance = 1_000_000_000
		let initial_issuance: u64 = 1_000_000_000;
		let _ = <Balances as Currency<_>>::deposit_creating(&1234, initial_issuance);
		assert_eq!(Balances::free_balance(1234), initial_issuance);
		MockBlockNumberProvider::set(1);
		Inflation::on_initialize(0);

		// Next inflation deposit happens when block is multiple of InflationBlockInterval
		let mut block: u32 = 2;
		let balance_before: u64 = Balances::free_balance(1234);
		while block % InflationBlockInterval::get() != 0 {
			MockBlockNumberProvider::set(block as u64);
			Inflation::on_initialize(0);
			block += 1;
		}
		let balance_just_before: u64 = Balances::free_balance(1234);
		assert_eq!(balance_before, balance_just_before);

		// The block with inflation
		MockBlockNumberProvider::set(block as u64);
		Inflation::on_initialize(0);
		let balance_after: u64 = Balances::free_balance(1234);
		assert_eq!(
			balance_after - balance_just_before,
			Inflation::block_inflation()
		);
	});
}

#[test]
fn inflation_in_1_year() {
	new_test_ext().execute_with(|| {
		// Total issuance = 1_000_000_000
		let initial_issuance: u64 = 1_000_000_000;
		let _ = <Balances as Currency<_>>::deposit_creating(&1234, initial_issuance);
		assert_eq!(Balances::free_balance(1234), initial_issuance);
		MockBlockNumberProvider::set(1);
		Inflation::on_initialize(0);

		// Go through all the block inflations for year 1,
		// total issuance will be updated accordingly
		for block in (100..YEAR).step_by(100) {
			MockBlockNumberProvider::set(block);
			Inflation::on_initialize(0);
		}
		assert_eq!(
			initial_issuance + (3803 * (YEAR / 100)),
			<Balances as Currency<_>>::total_issuance()
		);

		MockBlockNumberProvider::set(YEAR);
		Inflation::on_initialize(0);
		let block_inflation_year_1 = Inflation::block_inflation();
		// Expected 100-block inflation for year 2: 100 * 9.33% * initial issuance * 110% / YEAR = 3904
		assert_eq!(block_inflation_year_1, 3904);
	});
}

#[test]
fn inflation_in_1_to_9_years() {
	new_test_ext().execute_with(|| {
		// Total issuance = 1_000_000_000
		let initial_issuance: u64 = 1_000_000_000;

		let _ = <Balances as Currency<_>>::deposit_creating(&1234, initial_issuance);
		assert_eq!(Balances::free_balance(1234), initial_issuance);
		MockBlockNumberProvider::set(1);
		Inflation::on_initialize(0);

		for year in 1..=9 {
			let block_inflation_year_before = Inflation::block_inflation();
			MockBlockNumberProvider::set(YEAR * year);
			Inflation::on_initialize(0);
			let block_inflation_year_after = Inflation::block_inflation();

			// SBP M2 review: this is actually not true (not for the first few years)
			// Assert that next year inflation is less than previous year inflation
			assert!(block_inflation_year_before > block_inflation_year_after);
		}
	});
}

#[test]
fn inflation_after_year_10_is_flat() {
	new_test_ext().execute_with(|| {
		// Total issuance = 1_000_000_000
		let initial_issuance: u64 = 1_000_000_000;
		let _ = <Balances as Currency<_>>::deposit_creating(&1234, initial_issuance);
		assert_eq!(Balances::free_balance(1234), initial_issuance);
		MockBlockNumberProvider::set(YEAR * 9);
		Inflation::on_initialize(0);

		for year in 10..=20 {
			let block_inflation_year_before = Inflation::block_inflation();
			MockBlockNumberProvider::set(YEAR * year);
			Inflation::on_initialize(0);
			let block_inflation_year_after = Inflation::block_inflation();

			// Assert that next year inflation is equal to previous year inflation
			assert_eq!(block_inflation_year_before, block_inflation_year_after);
		}
	});
}

#[test]
fn inflation_rate_by_year() {
	new_test_ext().execute_with(|| {
		let payouts: u64 = YEAR / InflationBlockInterval::get() as u64;

		// Inflation starts at 10% and does down by 2/3% every year until year 9 (included),
		// then it is flat.
		let payout_by_year: [u64; 11] = [1000, 933, 867, 800, 733, 667, 600, 533, 467, 400, 400];

		// For accuracy total issuance = payout0 * payouts * 10;
		let initial_issuance: u64 = payout_by_year[0] * payouts * 10;
		let _ = <Balances as Currency<_>>::deposit_creating(&1234, initial_issuance);
		assert_eq!(Balances::free_balance(1234), initial_issuance);

		for year in 0..=10 {
			// Year first block
			MockBlockNumberProvider::set(YEAR * year);
			Inflation::on_initialize(0);
			let mut actual_payout = Inflation::block_inflation();
			assert_eq!(actual_payout, payout_by_year[year as usize]);

			// Year second block
			MockBlockNumberProvider::set(YEAR * year + 1);
			Inflation::on_initialize(0);
			actual_payout = Inflation::block_inflation();
			assert_eq!(actual_payout, payout_by_year[year as usize]);

			// Year middle block
			MockBlockNumberProvider::set(year * YEAR + YEAR / 2);
			Inflation::on_initialize(0);
			actual_payout = Inflation::block_inflation();
			assert_eq!(actual_payout, payout_by_year[year as usize]);

			// Year last block
			MockBlockNumberProvider::set((year + 1) * YEAR - 1);
			Inflation::on_initialize(0);
			actual_payout = Inflation::block_inflation();
			assert_eq!(actual_payout, payout_by_year[year as usize]);
		}
	});
}
