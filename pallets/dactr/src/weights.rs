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

//! Autogenerated weights for `da_control`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-12-189`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/avail-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=da_control
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/da_control_weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `da_control`.
pub trait WeightInfo {
	fn create_application_key() -> Weight;
	fn submit_block_length_proposal() -> Weight;
	fn submit_data(i: u32, ) -> Weight;
	fn set_application_key() -> Weight;
	fn data_root(i: u32, ) -> Weight;
	fn data_root_batch(i: u32, ) -> Weight;
}

/// Weights for `da_control` using the Avail node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `DataAvailability::AppKeys` (r:1 w:1)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DataAvailability::NextAppId` (r:1 w:1)
	/// Proof: `DataAvailability::NextAppId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn create_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `3583`
		// Minimum execution time: 24_049_000 picoseconds.
		Weight::from_parts(24_820_000, 3583)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::DynamicBlockLength` (r:1 w:1)
	/// Proof: `System::DynamicBlockLength` (`max_values`: Some(1), `max_size`: Some(24), added: 519, mode: `MaxEncodedLen`)
	fn submit_block_length_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1509`
		// Minimum execution time: 16_491_000 picoseconds.
		Weight::from_parts(17_044_000, 1509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_800_000 picoseconds.
		Weight::from_parts(4_371_059, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_692, 0).saturating_mul(i.into()))
	}
	/// Storage: `DataAvailability::AppKeys` (r:2 w:2)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	fn set_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377`
		//  Estimated: `6176`
		// Minimum execution time: 32_681_000 picoseconds.
		Weight::from_parts(33_451_000, 6176)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_859_000 picoseconds.
		Weight::from_parts(2_928_000, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(5_198, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_200_000 picoseconds.
		Weight::from_parts(8_193_988, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(5_269, 0).saturating_mul(i.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `DataAvailability::AppKeys` (r:1 w:1)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DataAvailability::NextAppId` (r:1 w:1)
	/// Proof: `DataAvailability::NextAppId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn create_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `3583`
		// Minimum execution time: 24_049_000 picoseconds.
		Weight::from_parts(24_820_000, 3583)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `System::DynamicBlockLength` (r:1 w:1)
	/// Proof: `System::DynamicBlockLength` (`max_values`: Some(1), `max_size`: Some(24), added: 519, mode: `MaxEncodedLen`)
	fn submit_block_length_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1509`
		// Minimum execution time: 16_491_000 picoseconds.
		Weight::from_parts(17_044_000, 1509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_800_000 picoseconds.
		Weight::from_parts(4_371_059, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_692, 0).saturating_mul(i.into()))
	}
	/// Storage: `DataAvailability::AppKeys` (r:2 w:2)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	fn set_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377`
		//  Estimated: `6176`
		// Minimum execution time: 32_681_000 picoseconds.
		Weight::from_parts(33_451_000, 6176)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_859_000 picoseconds.
		Weight::from_parts(2_928_000, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(5_198, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_200_000 picoseconds.
		Weight::from_parts(8_193_988, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(5_269, 0).saturating_mul(i.into()))
	}
}