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

//! Helper methods for npos-elections.

use crate::{
	Assignment, Error, ExtendedBalance, IdentifierT, PerThing128, StakedAssignment, Supports,
	VoteWeight,
};
use alloc::{collections::BTreeMap, vec::Vec};
use sp_arithmetic::PerThing;

/// Converts a vector of ratio assignments into ones with absolute budget value.
///
/// Note that this will NOT attempt at normalizing the result.
pub fn assignment_ratio_to_staked<A: IdentifierT, P: PerThing128, FS>(
	ratios: Vec<Assignment<A, P>>,
	stake_of: FS,
) -> Vec<StakedAssignment<A>>
where
	for<'r> FS: Fn(&'r A) -> VoteWeight,
{
	ratios
		.into_iter()
		.map(|a| {
			let stake = stake_of(&a.who);
			a.into_staked(stake.into())
		})
		.collect()
}

/// Same as [`assignment_ratio_to_staked`] and try and do normalization.
pub fn assignment_ratio_to_staked_normalized<A: IdentifierT, P: PerThing128, FS>(
	ratio: Vec<Assignment<A, P>>,
	stake_of: FS,
) -> Result<Vec<StakedAssignment<A>>, Error>
where
	for<'r> FS: Fn(&'r A) -> VoteWeight,
{
	let mut staked = assignment_ratio_to_staked(ratio, &stake_of);
	staked.iter_mut().try_for_each(|a| {
		a.try_normalize(stake_of(&a.who).into()).map_err(Error::ArithmeticError)
	})?;
	Ok(staked)
}

/// Converts a vector of staked assignments into ones with ratio values.
///
/// Note that this will NOT attempt at normalizing the result.
pub fn assignment_staked_to_ratio<A: IdentifierT, P: PerThing>(
	staked: Vec<StakedAssignment<A>>,
) -> Vec<Assignment<A, P>> {
	staked.into_iter().map(|a| a.into_assignment()).collect()
}

/// Same as [`assignment_staked_to_ratio`] and try and do normalization.
pub fn assignment_staked_to_ratio_normalized<A: IdentifierT, P: PerThing128>(
	staked: Vec<StakedAssignment<A>>,
) -> Result<Vec<Assignment<A, P>>, Error> {
	let mut ratio = staked.into_iter().map(|a| a.into_assignment()).collect::<Vec<_>>();
	for assignment in ratio.iter_mut() {
		assignment.try_normalize().map_err(Error::ArithmeticError)?;
	}
	Ok(ratio)
}

/// Convert some [`Supports`]s into vector of [`StakedAssignment`]
pub fn supports_to_staked_assignment<A: IdentifierT>(
	supports: Supports<A>,
) -> Vec<StakedAssignment<A>> {
	let mut staked: BTreeMap<A, Vec<(A, ExtendedBalance)>> = BTreeMap::new();
	for (target, support) in supports {
		for (voter, amount) in support.voters {
			staked.entry(voter).or_default().push((target.clone(), amount))
		}
	}

	staked
		.into_iter()
		.map(|(who, distribution)| StakedAssignment { who, distribution })
		.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_arithmetic::Perbill;

	#[test]
	fn into_staked_works() {
		let assignments = vec![
			Assignment {
				who: 1u32,
				distribution: vec![
					(10u32, Perbill::from_float(0.5)),
					(20, Perbill::from_float(0.5)),
				],
			},
			Assignment {
				who: 2u32,
				distribution: vec![
					(10, Perbill::from_float(0.33)),
					(20, Perbill::from_float(0.67)),
				],
			},
		];

		let stake_of = |_: &u32| -> VoteWeight { 100 };
		let staked = assignment_ratio_to_staked(assignments, stake_of);

		assert_eq!(
			staked,
			vec![
				StakedAssignment { who: 1u32, distribution: vec![(10u32, 50), (20, 50),] },
				StakedAssignment { who: 2u32, distribution: vec![(10u32, 33), (20, 67),] }
			]
		);
	}
}
