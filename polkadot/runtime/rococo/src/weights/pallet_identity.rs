// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-02-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `d3a9aad6f7a3`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/production/wbuild/rococo-runtime/rococo_runtime.wasm
// --pallet=pallet_identity
// --header=/__w/polkadot-sdk/polkadot-sdk/polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --no-storage-info
// --no-min-squares
// --no-median-slopes

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 10_004_000 picoseconds.
		Weight::from_parts(10_748_943, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_578
			.saturating_add(Weight::from_parts(115_154, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6977 + r * (5 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 111_710_000 picoseconds.
		Weight::from_parts(113_629_437, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 13_540
			.saturating_add(Weight::from_parts(242_440, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `11003 + s * (2589 ±0)`
		// Minimum execution time: 14_313_000 picoseconds.
		Weight::from_parts(28_127_745, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 6_350
			.saturating_add(Weight::from_parts(3_772_718, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194 + p * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 14_298_000 picoseconds.
		Weight::from_parts(29_044_043, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 4_816
			.saturating_add(Weight::from_parts(1_509_306, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7069 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 58_541_000 picoseconds.
		Weight::from_parts(60_250_804, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 20_548
			.saturating_add(Weight::from_parts(114_534, 0).saturating_mul(r.into()))
			// Standard Error: 4_009
			.saturating_add(Weight::from_parts(1_459_737, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6967 + r * (57 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 81_403_000 picoseconds.
		Weight::from_parts(83_290_223, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 5_622
			.saturating_add(Weight::from_parts(173_704, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6998`
		//  Estimated: `11003`
		// Minimum execution time: 77_512_000 picoseconds.
		Weight::from_parts(79_440_130, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 6_278
			.saturating_add(Weight::from_parts(125_036, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 6_800_000 picoseconds.
		Weight::from_parts(7_391_681, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_377
			.saturating_add(Weight::from_parts(73_824, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 7_034_000 picoseconds.
		Weight::from_parts(7_753_308, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_427
			.saturating_add(Weight::from_parts(72_233, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 6_931_000 picoseconds.
		Weight::from_parts(7_575_417, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_390
			.saturating_add(Weight::from_parts(66_947, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7045 + r * (57 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 99_768_000 picoseconds.
		Weight::from_parts(102_216_354, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 10_095
			.saturating_add(Weight::from_parts(64_928, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7276 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 73_027_000 picoseconds.
		Weight::from_parts(76_370_397, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 20_402
			.saturating_add(Weight::from_parts(114_391, 0).saturating_mul(r.into()))
			// Standard Error: 3_981
			.saturating_add(Weight::from_parts(1_456_205, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `475 + s * (36 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 30_194_000 picoseconds.
		Weight::from_parts(36_404_443, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 2_359
			.saturating_add(Weight::from_parts(109_976, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `591 + s * (3 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 18_677_000 picoseconds.
		Weight::from_parts(21_546_329, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 754
			.saturating_add(Weight::from_parts(54_656, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `638 + s * (35 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 35_542_000 picoseconds.
		Weight::from_parts(39_108_471, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 1_355
			.saturating_add(Weight::from_parts(84_570, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704 + s * (37 ±0)`
		//  Estimated: `6723`
		// Minimum execution time: 25_883_000 picoseconds.
		Weight::from_parts(28_966_665, 0)
			.saturating_add(Weight::from_parts(0, 6723))
			// Standard Error: 1_108
			.saturating_add(Weight::from_parts(90_991, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::AuthorityOf` (r:0 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_360_000 picoseconds.
		Weight::from_parts(7_774_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AuthorityOf` (r:1 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `3517`
		// Minimum execution time: 11_093_000 picoseconds.
		Weight::from_parts(11_618_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AuthorityOf` (r:1 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 1]`.
	fn set_username_for(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3593`
		// Minimum execution time: 68_099_000 picoseconds.
		Weight::from_parts(88_975_391, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameInfoOf` (r:0 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `116`
		//  Estimated: `3567`
		// Minimum execution time: 22_450_000 picoseconds.
		Weight::from_parts(22_997_000, 0)
			.saturating_add(Weight::from_parts(0, 3567))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 1]`.
	fn remove_expired_approval(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `3593`
		// Minimum execution time: 20_551_000 picoseconds.
		Weight::from_parts(51_063_408, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:0)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:0 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3563`
		// Minimum execution time: 14_475_000 picoseconds.
		Weight::from_parts(15_170_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:0)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UnbindingUsernames` (r:1 w:1)
	/// Proof: `Identity::UnbindingUsernames` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn unbind_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `236`
		//  Estimated: `3563`
		// Minimum execution time: 19_177_000 picoseconds.
		Weight::from_parts(20_099_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UnbindingUsernames` (r:1 w:1)
	/// Proof: `Identity::UnbindingUsernames` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297`
		//  Estimated: `3563`
		// Minimum execution time: 24_036_000 picoseconds.
		Weight::from_parts(25_394_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UnbindingUsernames` (r:1 w:1)
	/// Proof: `Identity::UnbindingUsernames` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 1]`.
	fn kill_username(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `3593`
		// Minimum execution time: 21_851_000 picoseconds.
		Weight::from_parts(52_842_483, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:2 w:0)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:2 w:0)
	/// Storage: `Identity::AuthorityOf` (r:0 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn migration_v2_authority_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `6087`
		// Minimum execution time: 9_265_000 picoseconds.
		Weight::from_parts(9_623_000, 0)
			.saturating_add(Weight::from_parts(0, 6087))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:2 w:0)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:2 w:0)
	/// Storage: `Identity::UsernameInfoOf` (r:0 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	fn migration_v2_username_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159`
		//  Estimated: `6099`
		// Minimum execution time: 9_156_000 picoseconds.
		Weight::from_parts(9_512_000, 0)
			.saturating_add(Weight::from_parts(0, 6099))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:2 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:0 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn migration_v2_identity_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7062`
		//  Estimated: `21016`
		// Minimum execution time: 60_988_000 picoseconds.
		Weight::from_parts(61_791_000, 0)
			.saturating_add(Weight::from_parts(0, 21016))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::PendingUsernames` (r:2 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	fn migration_v2_pending_username_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `6144`
		// Minimum execution time: 8_380_000 picoseconds.
		Weight::from_parts(8_826_000, 0)
			.saturating_add(Weight::from_parts(0, 6144))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AuthorityOf` (r:2 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:1 w:1)
	fn migration_v2_cleanup_authority_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `288`
		//  Estimated: `6044`
		// Minimum execution time: 12_286_000 picoseconds.
		Weight::from_parts(12_884_000, 0)
			.saturating_add(Weight::from_parts(0, 6044))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:2 w:0)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:1 w:1)
	fn migration_v2_cleanup_username_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `6136`
		// Minimum execution time: 11_065_000 picoseconds.
		Weight::from_parts(11_615_000, 0)
			.saturating_add(Weight::from_parts(0, 6136))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
