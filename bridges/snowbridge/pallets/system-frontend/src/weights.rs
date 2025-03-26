
//! Autogenerated weights for `snowbridge_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-09, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `crake.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain
// bridge-hub-rococo-dev
// --pallet=snowbridge_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template
// ../parachain/templates/module-weight-template.hbs
// --output
// ../parachain/pallets/control/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `snowbridge_system`.
pub trait WeightInfo {
	fn register_token() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn register_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `6044`
		// Minimum execution time: 45_000_000 picoseconds.
		Weight::from_parts(45_000_000, 6044)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
