// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus. If not, see <https://www.gnu.org/licenses/>.

//! The AuRa consensus algorithm for parachains.
//!
//! This extends the Substrate provided AuRa consensus implementation to make it compatible for
//! parachains. The main entry points for of this consensus algorithm are [`AuraConsensus::build`]
//! and [`fn@import_queue`].
//!
//! For more information about AuRa, the Substrate crate should be checked.

use codec::{Codec, Encode};
use cumulus_client_consensus_common::{
	ParachainBlockImportMarker, ParachainCandidate, ParachainConsensus,
};
use cumulus_primitives_core::{relay_chain::Hash as PHash, PersistedValidationData};

use cumulus_primitives_core::relay_chain::HeadData;
use futures::lock::Mutex;
use polkadot_primitives::{BlockNumber as RBlockNumber, Hash as RHash};
use sc_client_api::{backend::AuxStore, BlockOf};
use sc_consensus::BlockImport;
use sc_consensus_slots::{BackoffAuthoringBlocksStrategy, SimpleSlotWorker, SlotInfo};
use sc_telemetry::TelemetryHandle;
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_blockchain::HeaderBackend;
use sp_consensus::{EnableProofRecording, Environment, ProofRecording, Proposer, SyncOracle};
use sp_consensus_aura::{AuraApi, SlotDuration};
use sp_core::crypto::Pair;
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::KeystorePtr;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member, NumberFor};
use std::{
	convert::TryFrom,
	fs,
	fs::File,
	marker::PhantomData,
	path::PathBuf,
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc,
	},
};

mod import_queue;

pub use import_queue::{build_verifier, import_queue, BuildVerifierParams, ImportQueueParams};
use polkadot_node_primitives::PoV;
pub use sc_consensus_aura::{
	slot_duration, standalone::slot_duration_at, AuraVerifier, BuildAuraWorkerParams,
	SlotProportion,
};
pub use sc_consensus_slots::InherentDataProviderExt;

pub mod collator;
pub mod collators;
pub mod equivocation_import_queue;

const LOG_TARGET: &str = "aura::cumulus";

/// The implementation of the AURA consensus for parachains.
pub struct AuraConsensus<B, CIDP, W> {
	create_inherent_data_providers: Arc<CIDP>,
	aura_worker: Arc<Mutex<W>>,
	slot_duration: SlotDuration,
	last_slot_processed: Arc<AtomicU64>,
	_phantom: PhantomData<B>,
}

impl<B, CIDP, W> Clone for AuraConsensus<B, CIDP, W> {
	fn clone(&self) -> Self {
		Self {
			create_inherent_data_providers: self.create_inherent_data_providers.clone(),
			aura_worker: self.aura_worker.clone(),
			slot_duration: self.slot_duration,
			last_slot_processed: self.last_slot_processed.clone(),
			_phantom: PhantomData,
		}
	}
}

/// Parameters of [`AuraConsensus::build`].
#[deprecated = "Use the `aura::collators::basic` collator instead"]
pub struct BuildAuraConsensusParams<PF, BI, CIDP, Client, BS, SO> {
	pub proposer_factory: PF,
	pub create_inherent_data_providers: CIDP,
	pub block_import: BI,
	pub para_client: Arc<Client>,
	pub backoff_authoring_blocks: Option<BS>,
	pub sync_oracle: SO,
	pub keystore: KeystorePtr,
	pub force_authoring: bool,
	pub slot_duration: SlotDuration,
	pub telemetry: Option<TelemetryHandle>,
	pub block_proposal_slot_portion: SlotProportion,
	pub max_block_proposal_slot_portion: Option<SlotProportion>,
}

impl<B, CIDP> AuraConsensus<B, CIDP, ()>
where
	B: BlockT,
	CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + 'static,
	CIDP::InherentDataProviders: InherentDataProviderExt,
{
	/// Create a new boxed instance of AURA consensus.
	#[allow(deprecated)]
	#[deprecated = "Use the `aura::collators::basic` collator instead"]
	pub fn build<P, Client, BI, SO, PF, BS, Error>(
		BuildAuraConsensusParams {
			proposer_factory,
			create_inherent_data_providers,
			block_import,
			para_client,
			backoff_authoring_blocks,
			sync_oracle,
			keystore,
			force_authoring,
			slot_duration,
			telemetry,
			block_proposal_slot_portion,
			max_block_proposal_slot_portion,
		}: BuildAuraConsensusParams<PF, BI, CIDP, Client, BS, SO>,
	) -> Box<dyn ParachainConsensus<B>>
	where
		Client:
			ProvideRuntimeApi<B> + BlockOf + AuxStore + HeaderBackend<B> + Send + Sync + 'static,
		Client::Api: AuraApi<B, P::Public>,
		BI: BlockImport<B> + ParachainBlockImportMarker + Send + Sync + 'static,
		SO: SyncOracle + Send + Sync + Clone + 'static,
		BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + Sync + 'static,
		PF: Environment<B, Error = Error> + Send + Sync + 'static,
		PF::Proposer: Proposer<
			B,
			Error = Error,
			ProofRecording = EnableProofRecording,
			Proof = <EnableProofRecording as ProofRecording>::Proof,
		>,
		Error: std::error::Error + Send + From<sp_consensus::Error> + 'static,
		P: Pair + 'static,
		P::Public: AppPublic + Member + Codec,
		P::Signature: TryFrom<Vec<u8>> + Member + Codec,
	{
		let worker = sc_consensus_aura::build_aura_worker::<P, _, _, _, _, _, _, _, _>(
			BuildAuraWorkerParams {
				client: para_client,
				block_import,
				justification_sync_link: (),
				proposer_factory,
				sync_oracle,
				force_authoring,
				backoff_authoring_blocks,
				keystore,
				telemetry,
				block_proposal_slot_portion,
				max_block_proposal_slot_portion,
				compatibility_mode: sc_consensus_aura::CompatibilityMode::None,
			},
		);

		Box::new(AuraConsensus {
			create_inherent_data_providers: Arc::new(create_inherent_data_providers),
			aura_worker: Arc::new(Mutex::new(worker)),
			last_slot_processed: Default::default(),
			slot_duration,
			_phantom: PhantomData,
		})
	}
}

impl<B, CIDP, W> AuraConsensus<B, CIDP, W>
where
	B: BlockT,
	CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + 'static,
	CIDP::InherentDataProviders: InherentDataProviderExt,
{
	/// Create the inherent data.
	///
	/// Returns the created inherent data and the inherent data providers used.
	async fn inherent_data(
		&self,
		parent: B::Hash,
		validation_data: &PersistedValidationData,
		relay_parent: PHash,
	) -> Option<CIDP::InherentDataProviders> {
		self.create_inherent_data_providers
			.create_inherent_data_providers(parent, (relay_parent, validation_data.clone()))
			.await
			.map_err(|e| {
				tracing::error!(
					target: LOG_TARGET,
					error = ?e,
					"Failed to create inherent data providers.",
				)
			})
			.ok()
	}
}

#[async_trait::async_trait]
impl<B, CIDP, W> ParachainConsensus<B> for AuraConsensus<B, CIDP, W>
where
	B: BlockT,
	CIDP: CreateInherentDataProviders<B, (PHash, PersistedValidationData)> + Send + Sync + 'static,
	CIDP::InherentDataProviders: InherentDataProviderExt + Send,
	W: SimpleSlotWorker<B> + Send + Sync,
	W::Proposer: Proposer<B, Proof = <EnableProofRecording as ProofRecording>::Proof>,
{
	async fn produce_candidate(
		&mut self,
		parent: &B::Header,
		relay_parent: PHash,
		validation_data: &PersistedValidationData,
	) -> Option<ParachainCandidate<B>> {
		let inherent_data_providers =
			self.inherent_data(parent.hash(), validation_data, relay_parent).await?;

		let info = SlotInfo::new(
			inherent_data_providers.slot(),
			Box::new(inherent_data_providers),
			self.slot_duration.as_duration(),
			parent.clone(),
			// Set the block limit to 50% of the maximum PoV size.
			//
			// TODO: If we got benchmarking that includes the proof size,
			// we should be able to use the maximum pov size.
			Some((validation_data.max_pov_size / 2) as usize),
		);

		// With async backing this function will be called every relay chain block.
		//
		// Most parachains currently run with 12 seconds slots and thus, they would try to produce
		// multiple blocks per slot which very likely would fail on chain. Thus, we have this "hack"
		// to only produce on block per slot.
		//
		// With https://github.com/paritytech/polkadot-sdk/issues/3168 this implementation will be
		// obsolete and also the underlying issue will be fixed.
		if self.last_slot_processed.fetch_max(*info.slot, Ordering::Relaxed) >= *info.slot {
			return None
		}

		let res = self.aura_worker.lock().await.on_slot(info).await?;

		Some(ParachainCandidate { block: res.block, proof: res.storage_proof })
	}
}

/// Export the given `pov` to the file system at `path`.
///
/// The file will be named `block_hash_block_number.pov`.
///
/// The `parent_header`, `relay_parent_storage_root` and `relay_parent_number` will also be
/// stored in the file alongside the `pov`. This enables stateless validation of the `pov`.
pub(crate) fn export_pov_to_path<Block: BlockT>(
	path: PathBuf,
	pov: PoV,
	block_hash: Block::Hash,
	block_number: NumberFor<Block>,
	parent_header: Block::Header,
	relay_parent_storage_root: RHash,
	relay_parent_number: RBlockNumber,
	max_pov_size: u32,
) {
	if let Err(error) = fs::create_dir_all(&path) {
		tracing::error!(target: LOG_TARGET, %error, path = %path.display(), "Failed to create PoV export directory");
		return
	}

	let mut file = match File::create(path.join(format!("{block_hash:?}_{block_number}.pov"))) {
		Ok(f) => f,
		Err(error) => {
			tracing::error!(target: LOG_TARGET, %error, "Failed to export PoV.");
			return
		},
	};

	pov.encode_to(&mut file);
	PersistedValidationData {
		parent_head: HeadData(parent_header.encode()),
		relay_parent_number,
		relay_parent_storage_root,
		max_pov_size,
	}
	.encode_to(&mut file);
}
