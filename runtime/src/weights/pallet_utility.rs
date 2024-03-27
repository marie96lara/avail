// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-12-189`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/avail-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3997`
		// Minimum execution time: 10_104_000 picoseconds.
		Weight::from_parts(34_332_442, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			// Standard Error: 3_490
			.saturating_add(Weight::from_parts(7_581_574, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3997`
		// Minimum execution time: 14_746_000 picoseconds.
		Weight::from_parts(15_180_000, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3997`
		// Minimum execution time: 10_137_000 picoseconds.
		Weight::from_parts(14_172_962, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			// Standard Error: 4_583
			.saturating_add(Weight::from_parts(8_087_537, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_637_000 picoseconds.
		Weight::from_parts(14_038_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3997`
		// Minimum execution time: 10_394_000 picoseconds.
		Weight::from_parts(28_615_023, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			// Standard Error: 4_041
			.saturating_add(Weight::from_parts(7_638_485, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
