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

//! The crate's mock.

use crate as pallet_asset_rate;
/*
use frame_support::derive_impl;
use sp_runtime::BuildStorage;
*/
use frame::{runtime::prelude::{derive_impl, construct_runtime}, testing_prelude::BuildStorage, deps::frame_system, testing_prelude::TestExternalities};

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		AssetRate: pallet_asset_rate,
		Balances: pallet_balances,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u64>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
}

impl pallet_asset_rate::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type CreateOrigin = frame_system::EnsureRoot<u64>;
	type RemoveOrigin = frame_system::EnsureRoot<u64>;
	type UpdateOrigin = frame_system::EnsureRoot<u64>;
	type Currency = Balances;
	type AssetKind = u32;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
