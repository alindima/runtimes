// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Autogenerated weights for `pallet_broker`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./coretime-kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./coretime-kusama-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_broker
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./coretime-kusama-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_broker`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_broker::WeightInfo for WeightInfo<T> {
	fn swap_leases() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `1886`
		// Minimum execution time: 6_597_000 picoseconds.
		Weight::from_parts(6_969_000, 0)
			.saturating_add(Weight::from_parts(0, 1886))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	
	/// Storage: `Broker::Configuration` (r:0 w:1)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	fn configure() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_019_000 picoseconds.
		Weight::from_parts(2_113_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10888`
		//  Estimated: `13506`
		// Minimum execution time: 21_802_000 picoseconds.
		Weight::from_parts(22_323_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12090`
		//  Estimated: `13506`
		// Minimum execution time: 20_534_000 picoseconds.
		Weight::from_parts(21_308_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466`
		//  Estimated: `1951`
		// Minimum execution time: 10_629_000 picoseconds.
		Weight::from_parts(10_941_000, 0)
			.saturating_add(Weight::from_parts(0, 1951))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:0 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:60)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn start_sales(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12567`
		//  Estimated: `14052`
		// Minimum execution time: 112_537_000 picoseconds.
		Weight::from_parts(119_546_511, 0)
			.saturating_add(Weight::from_parts(0, 14052))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(66))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:0 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn purchase() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3593`
		// Minimum execution time: 32_291_000 picoseconds.
		Weight::from_parts(33_428_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:2)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `434`
		//  Estimated: `4698`
		// Minimum execution time: 69_914_000 picoseconds.
		Weight::from_parts(78_519_000, 0)
			.saturating_add(Weight::from_parts(0, 4698))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3550`
		// Minimum execution time: 12_634_000 picoseconds.
		Weight::from_parts(13_156_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn partition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3550`
		// Minimum execution time: 13_869_000 picoseconds.
		Weight::from_parts(14_691_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Regions` (r:1 w:3)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn interlace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3550`
		// Minimum execution time: 15_415_000 picoseconds.
		Weight::from_parts(16_270_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn assign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `936`
		//  Estimated: `4681`
		// Minimum execution time: 24_709_000 picoseconds.
		Weight::from_parts(25_781_000, 0)
			.saturating_add(Weight::from_parts(0, 4681))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:2 w:2)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:0 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1002`
		//  Estimated: `5996`
		// Minimum execution time: 29_694_000 picoseconds.
		Weight::from_parts(31_352_000, 0)
			.saturating_add(Weight::from_parts(0, 5996))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:3 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[1, 3]`.
	fn claim_revenue(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `652`
		//  Estimated: `6196 + m * (2520 ±0)`
		// Minimum execution time: 55_764_000 picoseconds.
		Weight::from_parts(56_067_599, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			// Standard Error: 45_065
			.saturating_add(Weight::from_parts(1_490_168, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(m.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3593`
		// Minimum execution time: 38_451_000 picoseconds.
		Weight::from_parts(39_235_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn drop_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465`
		//  Estimated: `3550`
		// Minimum execution time: 55_440_000 picoseconds.
		Weight::from_parts(65_949_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn drop_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `3533`
		// Minimum execution time: 120_176_000 picoseconds.
		Weight::from_parts(135_905_000, 0)
			.saturating_add(Weight::from_parts(0, 3533))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn drop_history() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `857`
		//  Estimated: `3593`
		// Minimum execution time: 92_128_000 picoseconds.
		Weight::from_parts(106_439_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:1)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	fn drop_renewal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `957`
		//  Estimated: `4698`
		// Minimum execution time: 46_730_000 picoseconds.
		Weight::from_parts(66_288_000, 0)
			.saturating_add(Weight::from_parts(0, 4698))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 1000]`.
	fn request_core_count(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 18_819_000 picoseconds.
		Weight::from_parts(19_830_463, 0)
			.saturating_add(Weight::from_parts(0, 3539))
			// Standard Error: 44
			.saturating_add(Weight::from_parts(8, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::CoreCountInbox` (r:1 w:1)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn process_core_count(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `266`
		//  Estimated: `1487`
		// Minimum execution time: 5_566_000 picoseconds.
		Weight::from_parts(6_078_648, 0)
			.saturating_add(Weight::from_parts(0, 1487))
			// Standard Error: 16
			.saturating_add(Weight::from_parts(47, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn process_revenue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447`
		//  Estimated: `6196`
		// Minimum execution time: 37_255_000 picoseconds.
		Weight::from_parts(37_998_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:60)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn rotate_sale(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12514`
		//  Estimated: `13506`
		// Minimum execution time: 91_560_000 picoseconds.
		Weight::from_parts(98_810_427, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(65))
	}
	/// Storage: `Broker::InstaPoolIo` (r:1 w:0)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3493`
		// Minimum execution time: 5_927_000 picoseconds.
		Weight::from_parts(6_236_000, 0)
			.saturating_add(Weight::from_parts(0, 3493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workload` (r:1 w:1)
	/// Proof: `Broker::Workload` (`max_values`: None, `max_size`: Some(1212), added: 3687, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn process_core_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1321`
		//  Estimated: `4786`
		// Minimum execution time: 30_690_000 picoseconds.
		Weight::from_parts(31_495_000, 0)
			.saturating_add(Weight::from_parts(0, 4786))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	fn request_revenue_info_at() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 180_000 picoseconds.
		Weight::from_parts(205_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Broker::CoreCountInbox` (r:0 w:1)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	fn notify_core_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_694_000 picoseconds.
		Weight::from_parts(1_891_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::CoreCountInbox` (r:1 w:0)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn do_tick_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398`
		//  Estimated: `3863`
		// Minimum execution time: 12_033_000 picoseconds.
		Weight::from_parts(12_490_000, 0)
			.saturating_add(Weight::from_parts(0, 3863))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
