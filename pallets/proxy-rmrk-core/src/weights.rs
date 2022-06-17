// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_proxy_rmrk_core
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-17, STEPS: `50`, REPEAT: 80, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// pallet
// --pallet
// pallet-proxy-rmrk-core
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=80
// --heap-pages=4096
// --output=./pallets/proxy-rmrk-core/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_proxy_rmrk_core.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn destroy_collection() -> Weight;
	fn change_collection_issuer() -> Weight;
	fn lock_collection() -> Weight;
	fn mint_nft(b: u32, ) -> Weight;
	fn burn_nft(b: u32, ) -> Weight;
	fn send() -> Weight;
	fn accept_nft() -> Weight;
	fn reject_nft() -> Weight;
	fn set_property() -> Weight;
	fn set_priority() -> Weight;
	fn add_basic_resource() -> Weight;
	fn add_composable_resource() -> Weight;
	fn add_slot_resource() -> Weight;
	fn remove_resource() -> Weight;
	fn accept_resource() -> Weight;
	fn accept_resource_removal() -> Weight;
}

/// Weights for pallet_proxy_rmrk_core using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Common CreatedCollectionCount (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: RmrkCore CollectionIndex (r:1 w:1)
	// Storage: Common CollectionPropertyPermissions (r:0 w:1)
	// Storage: Common CollectionProperties (r:0 w:1)
	// Storage: Common CollectionById (r:0 w:1)
	// Storage: RmrkCore UniqueCollectionId (r:0 w:1)
	fn create_collection() -> Weight {
		(42_359_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:1)
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:0)
	// Storage: Common DestroyedCollectionCount (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:0 w:1)
	// Storage: Nonfungible TokensBurnt (r:0 w:1)
	// Storage: Common AdminAmount (r:0 w:1)
	fn destroy_collection() -> Weight {
		(45_375_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Common CollectionProperties (r:1 w:0)
	fn change_collection_issuer() -> Weight {
		(22_753_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:1 w:0)
	// Storage: Nonfungible TokensBurnt (r:1 w:0)
	fn lock_collection() -> Weight {
		(24_356_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:2 w:2)
	fn mint_nft(b: u32, ) -> Weight {
		(44_853_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((10_721_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokenChildren (r:1 w:0)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:1)
	// Storage: Nonfungible TokenProperties (r:0 w:1)
	fn burn_nft(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_590_000
			.saturating_add((319_825_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:0)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible TokenChildren (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn send() -> Weight {
		(72_576_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:2 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:2 w:0)
	// Storage: Nonfungible TokenData (r:6 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenChildren (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn accept_nft() -> Weight {
		(79_670_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:5)
	// Storage: Nonfungible TokenProperties (r:1 w:5)
	// Storage: Nonfungible TokenChildren (r:9 w:4)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:5 w:5)
	// Storage: Nonfungible Allowance (r:5 w:0)
	// Storage: Nonfungible Owned (r:0 w:5)
	fn reject_nft() -> Weight {
		(254_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:5 w:0)
	fn set_property() -> Weight {
		(48_651_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:5 w:0)
	fn set_priority() -> Weight {
		(47_579_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn add_basic_resource() -> Weight {
		(55_013_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn add_composable_resource() -> Weight {
		(55_184_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn add_slot_resource() -> Weight {
		(54_792_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:5 w:0)
	fn remove_resource() -> Weight {
		(46_447_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn accept_resource() -> Weight {
		(45_096_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn accept_resource_removal() -> Weight {
		(45_445_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Common CreatedCollectionCount (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: RmrkCore CollectionIndex (r:1 w:1)
	// Storage: Common CollectionPropertyPermissions (r:0 w:1)
	// Storage: Common CollectionProperties (r:0 w:1)
	// Storage: Common CollectionById (r:0 w:1)
	// Storage: RmrkCore UniqueCollectionId (r:0 w:1)
	fn create_collection() -> Weight {
		(42_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:1)
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:0)
	// Storage: Common DestroyedCollectionCount (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:0 w:1)
	// Storage: Nonfungible TokensBurnt (r:0 w:1)
	// Storage: Common AdminAmount (r:0 w:1)
	fn destroy_collection() -> Weight {
		(45_375_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Common CollectionProperties (r:1 w:0)
	fn change_collection_issuer() -> Weight {
		(22_753_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:1 w:0)
	// Storage: Nonfungible TokensBurnt (r:1 w:0)
	fn lock_collection() -> Weight {
		(24_356_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:2 w:2)
	fn mint_nft(b: u32, ) -> Weight {
		(44_853_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((10_721_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokenChildren (r:1 w:0)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:1)
	// Storage: Nonfungible TokenProperties (r:0 w:1)
	fn burn_nft(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_590_000
			.saturating_add((319_825_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((4 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:0)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible TokenChildren (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn send() -> Weight {
		(72_576_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(12 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:2 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:2 w:0)
	// Storage: Nonfungible TokenData (r:6 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenChildren (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn accept_nft() -> Weight {
		(79_670_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(15 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:5)
	// Storage: Nonfungible TokenProperties (r:1 w:5)
	// Storage: Nonfungible TokenChildren (r:9 w:4)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:5 w:5)
	// Storage: Nonfungible Allowance (r:5 w:0)
	// Storage: Nonfungible Owned (r:0 w:5)
	fn reject_nft() -> Weight {
		(254_989_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(29 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:5 w:0)
	fn set_property() -> Weight {
		(48_651_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:5 w:0)
	fn set_priority() -> Weight {
		(47_579_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn add_basic_resource() -> Weight {
		(55_013_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn add_composable_resource() -> Weight {
		(55_184_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn add_slot_resource() -> Weight {
		(54_792_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:5 w:0)
	fn remove_resource() -> Weight {
		(46_447_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn accept_resource() -> Weight {
		(45_096_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: RmrkCore UniqueCollectionId (r:1 w:0)
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: Nonfungible TokenData (r:5 w:0)
	// Storage: Nonfungible TokenAuxProperties (r:1 w:1)
	fn accept_resource_removal() -> Weight {
		(45_445_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
