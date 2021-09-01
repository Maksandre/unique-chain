// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-31, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

// Executed Command:
// target/release/nft
// benchmark
// --pallet
// pallet-nft
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=20
// --output=./pallets/nft/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nft.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn destroy_collection() -> Weight;
	fn add_to_white_list() -> Weight;
	fn remove_from_white_list() -> Weight;
	fn set_public_access_mode() -> Weight;
	fn set_mint_permission() -> Weight;
	fn change_collection_owner() -> Weight;
	fn add_collection_admin() -> Weight;
	fn remove_collection_admin() -> Weight;
	fn set_collection_sponsor() -> Weight;
	fn confirm_sponsorship() -> Weight;
	fn remove_collection_sponsor() -> Weight;
	fn create_item_nft(b: u32, ) -> Weight;
	fn create_multiple_items_nft(b: u32, ) -> Weight;
	fn create_item_fungible() -> Weight;
	fn create_multiple_items_fungible(b: u32, ) -> Weight;
	fn create_item_refungible(b: u32, ) -> Weight;
	fn create_multiple_items_refungible(b: u32, ) -> Weight;
	fn burn_item_nft() -> Weight;
	fn transfer_nft() -> Weight;
	fn transfer_fungible() -> Weight;
	fn transfer_refungible() -> Weight;
	fn set_transfers_enabled_flag() -> Weight;
	fn approve_nft() -> Weight;
	fn transfer_from_nft() -> Weight;
	fn transfer_from_fungible() -> Weight;
	fn transfer_from_refungible() -> Weight;
	fn set_offchain_schema(b: u32, ) -> Weight;
	fn set_const_on_chain_schema(b: u32, ) -> Weight;
	fn set_variable_on_chain_schema(b: u32, ) -> Weight;
	fn set_variable_meta_data_nft(b: u32, ) -> Weight;
	fn set_schema_version() -> Weight;
	fn set_collection_limits() -> Weight;
}

/// Weights for pallet_nft using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_collection() -> Weight {
		(25_851_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn destroy_collection() -> Weight {
		(28_737_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn add_to_white_list() -> Weight {
		(6_237_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_from_white_list() -> Weight {
		(6_252_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_public_access_mode() -> Weight {
		(6_691_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_mint_permission() -> Weight {
		(6_630_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn change_collection_owner() -> Weight {
		(6_521_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn add_collection_admin() -> Weight {
		(8_057_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_collection_admin() -> Weight {
		(8_307_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_collection_sponsor() -> Weight {
		(6_484_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn confirm_sponsorship() -> Weight {
		(6_530_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_collection_sponsor() -> Weight {
		(6_733_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn create_item_nft(_b: u32, ) -> Weight {
		(167_909_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn create_multiple_items_nft(b: u32, ) -> Weight {
		(336_830_000 as Weight)
			// Standard Error: 42_000
			.saturating_add((11_627_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
	}
	fn create_item_fungible() -> Weight {
		(24_123_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn create_multiple_items_fungible(b: u32, ) -> Weight {
		(48_227_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((2_918_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn create_item_refungible(_b: u32, ) -> Weight {
		(26_293_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn create_multiple_items_refungible(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((8_374_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
	}
	fn burn_item_nft() -> Weight {
		(32_237_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn transfer_nft() -> Weight {
		(192_578_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn transfer_fungible() -> Weight {
		(170_749_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn transfer_refungible() -> Weight {
		(35_949_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn set_transfers_enabled_flag() -> Weight {
		(6_376_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn approve_nft() -> Weight {
		(169_825_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn transfer_from_nft() -> Weight {
		(197_912_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn transfer_from_fungible() -> Weight {
		(183_789_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn transfer_from_refungible() -> Weight {
		(37_149_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn set_offchain_schema(_b: u32, ) -> Weight {
		(6_435_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_const_on_chain_schema(_b: u32, ) -> Weight {
		(6_646_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_variable_on_chain_schema(_b: u32, ) -> Weight {
		(6_542_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_variable_meta_data_nft(_b: u32, ) -> Weight {
		(14_697_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_schema_version() -> Weight {
		(6_566_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_collection_limits() -> Weight {
		(6_349_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_collection() -> Weight {
		(25_851_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn destroy_collection() -> Weight {
		(28_737_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn add_to_white_list() -> Weight {
		(6_237_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_from_white_list() -> Weight {
		(6_252_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_public_access_mode() -> Weight {
		(6_691_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_mint_permission() -> Weight {
		(6_630_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn change_collection_owner() -> Weight {
		(6_521_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn add_collection_admin() -> Weight {
		(8_057_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_collection_admin() -> Weight {
		(8_307_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_collection_sponsor() -> Weight {
		(6_484_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn confirm_sponsorship() -> Weight {
		(6_530_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_collection_sponsor() -> Weight {
		(6_733_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn create_item_nft(b: u32, ) -> Weight {
		(167_180_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((10_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn create_multiple_items_nft(b: u32, ) -> Weight {
		(336_830_000 as Weight)
			// Standard Error: 42_000
			.saturating_add((11_627_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
	}
	fn create_item_fungible() -> Weight {
		(24_123_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn create_multiple_items_fungible(b: u32, ) -> Weight {
		(13_217_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((2_971_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn create_item_refungible(_b: u32, ) -> Weight {
		(26_293_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn create_multiple_items_refungible(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((8_374_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
	}
	fn burn_item_nft() -> Weight {
		(32_237_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn transfer_nft() -> Weight {
		(192_578_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(14 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn transfer_fungible() -> Weight {
		(170_749_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn transfer_refungible() -> Weight {
		(35_949_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn set_transfers_enabled_flag() -> Weight {
		(6_376_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn approve_nft() -> Weight {
		(169_825_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn transfer_from_nft() -> Weight {
		(197_912_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(15 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn transfer_from_fungible() -> Weight {
		(183_789_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn transfer_from_refungible() -> Weight {
		(37_149_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn set_offchain_schema(_b: u32, ) -> Weight {
		(6_435_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_const_on_chain_schema(_b: u32, ) -> Weight {
		(6_646_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_variable_on_chain_schema(_b: u32, ) -> Weight {
		(6_542_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_variable_meta_data_nft(_b: u32, ) -> Weight {
		(14_697_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_schema_version() -> Weight {
		(6_566_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_collection_limits() -> Weight {
		(6_349_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}