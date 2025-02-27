// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.
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

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-11, STEPS: `2`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("collectives-polkadot-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --wasm-execution=compiled
// --pallet=pallet_referenda
// --extrinsic=*
// --steps=2
// --repeat=2
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:0 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `159279`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(34_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `317568`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(68_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1165`
		//  Estimated: `159279`
		// Minimum execution time: 97_000_000 picoseconds.
		Weight::from_parts(123_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1173`
		//  Estimated: `159279`
		// Minimum execution time: 104_000_000 picoseconds.
		Weight::from_parts(111_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `702`
		//  Estimated: `317568`
		// Minimum execution time: 140_000_000 picoseconds.
		Weight::from_parts(150_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `317568`
		// Minimum execution time: 81_000_000 picoseconds.
		Weight::from_parts(82_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `317`
		//  Estimated: `4365`
		// Minimum execution time: 38_000_000 picoseconds.
		Weight::from_parts(38_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167`
		//  Estimated: `4365`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(18_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `317568`
		// Minimum execution time: 44_000_000 picoseconds.
		Weight::from_parts(45_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
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
	/// Storage: `AmbassadorReferenda::MetadataOf` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `626`
		//  Estimated: `317568`
		// Minimum execution time: 183_000_000 picoseconds.
		Weight::from_parts(187_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3636`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3636))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1412`
		//  Estimated: `159279`
		// Minimum execution time: 88_000_000 picoseconds.
		Weight::from_parts(97_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1412`
		//  Estimated: `159279`
		// Minimum execution time: 87_000_000 picoseconds.
		Weight::from_parts(92_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `4365`
		// Minimum execution time: 43_000_000 picoseconds.
		Weight::from_parts(46_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `4365`
		// Minimum execution time: 39_000_000 picoseconds.
		Weight::from_parts(43_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `951`
		//  Estimated: `4365`
		// Minimum execution time: 48_000_000 picoseconds.
		Weight::from_parts(50_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `959`
		//  Estimated: `4365`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(48_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263`
		//  Estimated: `159279`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `159279`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(28_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `4365`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `546`
		//  Estimated: `159279`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(46_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `647`
		//  Estimated: `159279`
		// Minimum execution time: 87_000_000 picoseconds.
		Weight::from_parts(93_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `700`
		//  Estimated: `159279`
		// Minimum execution time: 100_000_000 picoseconds.
		Weight::from_parts(120_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `683`
		//  Estimated: `159279`
		// Minimum execution time: 90_000_000 picoseconds.
		Weight::from_parts(100_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `700`
		//  Estimated: `159279`
		// Minimum execution time: 77_000_000 picoseconds.
		Weight::from_parts(82_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704`
		//  Estimated: `159279`
		// Minimum execution time: 68_000_000 picoseconds.
		Weight::from_parts(77_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704`
		//  Estimated: `317568`
		// Minimum execution time: 99_000_000 picoseconds.
		Weight::from_parts(104_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `700`
		//  Estimated: `159279`
		// Minimum execution time: 87_000_000 picoseconds.
		Weight::from_parts(100_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::MetadataOf` (r:0 w:1)
	/// Proof: `AmbassadorReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419`
		//  Estimated: `4365`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(25_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::MetadataOf` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `4365`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
