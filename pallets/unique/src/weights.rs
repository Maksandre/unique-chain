// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_unique
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// --pallet
// pallet-unique
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=20
// --output=./pallets/unique/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_unique.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn destroy_collection() -> Weight;
	fn add_to_allow_list() -> Weight;
	fn remove_from_allow_list() -> Weight;
	fn set_public_access_mode() -> Weight;
	fn set_mint_permission() -> Weight;
	fn change_collection_owner() -> Weight;
	fn add_collection_admin() -> Weight;
	fn remove_collection_admin() -> Weight;
	fn set_collection_sponsor() -> Weight;
	fn confirm_sponsorship() -> Weight;
	fn remove_collection_sponsor() -> Weight;
	fn set_transfers_enabled_flag() -> Weight;
	fn set_offchain_schema(b: u32, ) -> Weight;
	fn set_const_on_chain_schema(b: u32, ) -> Weight;
	fn set_variable_on_chain_schema(b: u32, ) -> Weight;
	fn set_schema_version() -> Weight;
	fn set_collection_limits() -> Weight;
	fn set_meta_update_permission_flag() -> Weight;
}

/// Weights for pallet_unique using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Common CreatedCollectionCount (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Common CollectionById (r:0 w:1)
	fn create_collection() -> Weight {
		(28_864_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:0 w:1)
	// Storage: Nonfungible TokensBurnt (r:0 w:1)
	// Storage: Common AdminAmount (r:0 w:1)
	fn destroy_collection() -> Weight {
		(39_196_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common Allowlist (r:0 w:1)
	fn add_to_allow_list() -> Weight {
		(15_161_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common Allowlist (r:0 w:1)
	fn remove_from_allow_list() -> Weight {
		(17_224_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_public_access_mode() -> Weight {
		(14_557_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_mint_permission() -> Weight {
		(14_835_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn change_collection_owner() -> Weight {
		(15_587_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common IsAdmin (r:1 w:1)
	// Storage: Common AdminAmount (r:1 w:1)
	fn add_collection_admin() -> Weight {
		(24_447_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common IsAdmin (r:1 w:1)
	// Storage: Common AdminAmount (r:1 w:1)
	fn remove_collection_admin() -> Weight {
		(22_141_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_collection_sponsor() -> Weight {
		(15_407_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn confirm_sponsorship() -> Weight {
		(14_939_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn remove_collection_sponsor() -> Weight {
		(14_504_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_transfers_enabled_flag() -> Weight {
		(7_218_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_offchain_schema(_b: u32, ) -> Weight {
		(15_040_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_const_on_chain_schema(_b: u32, ) -> Weight {
		(14_932_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_variable_on_chain_schema(_b: u32, ) -> Weight {
		(15_291_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_schema_version() -> Weight {
		(14_725_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_collection_limits() -> Weight {
		(15_222_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_meta_update_permission_flag() -> Weight {
		(7_270_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Common CreatedCollectionCount (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Common CollectionById (r:0 w:1)
	fn create_collection() -> Weight {
		(28_864_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:0 w:1)
	// Storage: Nonfungible TokensBurnt (r:0 w:1)
	// Storage: Common AdminAmount (r:0 w:1)
	fn destroy_collection() -> Weight {
		(39_196_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common Allowlist (r:0 w:1)
	fn add_to_allow_list() -> Weight {
		(15_161_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common Allowlist (r:0 w:1)
	fn remove_from_allow_list() -> Weight {
		(17_224_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_public_access_mode() -> Weight {
		(14_557_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_mint_permission() -> Weight {
		(14_835_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn change_collection_owner() -> Weight {
		(15_587_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common IsAdmin (r:1 w:1)
	// Storage: Common AdminAmount (r:1 w:1)
	fn add_collection_admin() -> Weight {
		(24_447_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Common IsAdmin (r:1 w:1)
	// Storage: Common AdminAmount (r:1 w:1)
	fn remove_collection_admin() -> Weight {
		(22_141_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_collection_sponsor() -> Weight {
		(15_407_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn confirm_sponsorship() -> Weight {
		(14_939_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn remove_collection_sponsor() -> Weight {
		(14_504_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_transfers_enabled_flag() -> Weight {
		(7_218_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_offchain_schema(_b: u32, ) -> Weight {
		(15_040_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_const_on_chain_schema(_b: u32, ) -> Weight {
		(14_932_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_variable_on_chain_schema(_b: u32, ) -> Weight {
		(15_291_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_schema_version() -> Weight {
		(14_725_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_collection_limits() -> Weight {
		(15_222_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Common CollectionById (r:1 w:1)
	fn set_meta_update_permission_flag() -> Weight {
		(7_270_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
