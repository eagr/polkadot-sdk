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

use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use polkadot_node_core_pvf_common::{
	executor_interface::{prepare, prevalidate},
	prepare::PrepareJobKind,
	pvf::PvfPrepData,
};
use polkadot_primitives::ExecutorParams;
use std::time::Duration;

fn do_prepare_runtime(pvf: PvfPrepData) {
	let maybe_compressed_code = pvf.maybe_compressed_code();
	let raw_validation_code =
		sp_maybe_compressed_blob::decompress(&maybe_compressed_code, usize::MAX).unwrap();

	let blob = match prevalidate(&raw_validation_code) {
		Err(err) => panic!("{:?}", err),
		Ok(b) => b,
	};

	match prepare(blob, &pvf.executor_params()) {
		Ok(_) => (),
		Err(err) => panic!("{:?}", err),
	}
}

fn prepare_rococo_runtime(c: &mut Criterion) {
	let blob = rococo_runtime::WASM_BINARY.unwrap();
	let pvf = match sp_maybe_compressed_blob::decompress(&blob, 64 * 1024 * 1024) {
		Ok(code) => PvfPrepData::from_code(
			code.into_owned(),
			ExecutorParams::default(),
			Duration::from_secs(360),
			PrepareJobKind::Compilation,
			64 * 1024 * 1024,
		),
		Err(e) => {
			panic!("Cannot decompress blob: {:?}", e);
		},
	};

	let mut group = c.benchmark_group("rococo");
	group.sampling_mode(SamplingMode::Flat);
	group.sample_size(20);
	group.measurement_time(Duration::from_secs(240));
	group.bench_function("prepare Rococo runtime", |b| {
		// `PvfPrepData` is designed to be cheap to clone, so cloning shouldn't affect the
		// benchmark accuracy
		b.iter(|| do_prepare_runtime(pvf.clone()))
	});
	group.finish();
}

criterion_group!(preparation, prepare_rococo_runtime);
criterion_main!(preparation);
