// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_proxy_rmrk_equip
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-01, STEPS: `50`, REPEAT: 80, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/unique-collator
// benchmark
// pallet
// --pallet
// pallet-proxy-rmrk-equip
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=80
// --heap-pages=4096
// --output=./pallets/proxy-rmrk-equip/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_proxy_rmrk_equip.
pub trait WeightInfo {
	fn create_base(b: u32, ) -> Weight;
	fn theme_add(b: u32, ) -> Weight;
	fn equippable() -> Weight;
}

/// Weights for pallet_proxy_rmrk_equip using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Common CreatedCollectionCount (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Common CollectionPropertyPermissions (r:0 w:1)
	// Storage: Common CollectionProperties (r:0 w:1)
	// Storage: Common CollectionById (r:0 w:1)
	// Storage: RmrkEquip InernalPartId (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn create_base(b: u32, ) -> Weight {
		(57_871_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((19_870_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: RmrkEquip BaseHasDefaultTheme (r:1 w:0)
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn theme_add(b: u32, ) -> Weight {
		(46_121_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((2_988_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: RmrkEquip InernalPartId (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	fn equippable() -> Weight {
		(32_032_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Common CreatedCollectionCount (r:1 w:1)
	// Storage: Common DestroyedCollectionCount (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Common CollectionPropertyPermissions (r:0 w:1)
	// Storage: Common CollectionProperties (r:0 w:1)
	// Storage: Common CollectionById (r:0 w:1)
	// Storage: RmrkEquip InernalPartId (r:1 w:1)
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn create_base(b: u32, ) -> Weight {
		(57_871_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((19_870_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes((4 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: RmrkEquip BaseHasDefaultTheme (r:1 w:0)
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn theme_add(b: u32, ) -> Weight {
		(46_121_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((2_988_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Common CollectionProperties (r:1 w:0)
	// Storage: Common CollectionById (r:1 w:0)
	// Storage: RmrkEquip InernalPartId (r:1 w:0)
	// Storage: Nonfungible TokenProperties (r:1 w:1)
	fn equippable() -> Weight {
		(32_032_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
