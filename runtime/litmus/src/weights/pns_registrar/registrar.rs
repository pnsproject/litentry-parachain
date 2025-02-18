// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pns_registrar::registrar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-23, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pns_registrar::registrar
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pns_registrar/registrar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pns_registrar::registrar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pns_registrar::registrar::WeightInfo for WeightInfo<T> {
	// Storage: PnsOrigin Origins (r:1 w:0)
	// Storage: PnsRegistrar ReservedList (r:0 w:1)
	fn add_reserved() -> Weight {
		(3_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PnsOrigin Origins (r:1 w:0)
	// Storage: PnsRegistrar ReservedList (r:0 w:1)
	fn remove_reserved() -> Weight {
		(3_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PnsOrigin IsRegistrarOpen (r:1 w:0)
	// Storage: PnsPriceOracle RentPrice (r:1 w:0)
	// Storage: PnsPriceOracle ExchangeRate (r:1 w:0)
	// Storage: PnsRegistry Official (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: PnsRegistrar ReservedList (r:1 w:0)
	// Storage: PnsNft Tokens (r:2 w:2)
	// Storage: PnsPriceOracle BasePrice (r:1 w:0)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:1)
	// Storage: PnsNft Classes (r:1 w:1)
	// Storage: PnsRegistry Origin (r:1 w:1)
	// Storage: PnsNft TokensByOwner (r:0 w:1)
	fn register(_l: u32, ) -> Weight {
		(48_844_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: PnsOrigin IsRegistrarOpen (r:1 w:0)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: PnsPriceOracle RentPrice (r:1 w:0)
	// Storage: PnsPriceOracle ExchangeRate (r:1 w:0)
	// Storage: PnsRegistry Official (r:1 w:0)
	fn renew(l: u32, ) -> Weight {
		(22_948_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((8_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: PnsOrigin IsRegistrarOpen (r:1 w:0)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: PnsNft Tokens (r:1 w:1)
	// Storage: PnsRegistry Origin (r:1 w:0)
	// Storage: PnsNft TokensByOwner (r:0 w:2)
	fn set_owner() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: PnsOrigin IsRegistrarOpen (r:1 w:0)
	// Storage: PnsRegistrar RegistrarInfos (r:1 w:0)
	// Storage: PnsNft Tokens (r:2 w:2)
	// Storage: PnsNft Classes (r:1 w:1)
	// Storage: PnsRegistry Origin (r:1 w:1)
	// Storage: PnsNft TokensByOwner (r:0 w:1)
	fn mint_subname(l: u32, ) -> Weight {
		(35_550_000 as Weight)
			// Standard Error: 0
			.saturating_add((7_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
}
