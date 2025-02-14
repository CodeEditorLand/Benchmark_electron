// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

fn root_dir() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.to_path_buf()
}

fn write_json(filename:&PathBuf, value:&Value) {
	let f = File::create(filename).expect("Unable to create file");

	serde_json::to_writer(f, value).expect("Unable to write json");
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BenchResult {
	created_at:String,
	sha1:String,

	exec_time:HashMap<String, HashMap<String, f64>>,
	binary_size:HashMap<String, u64>,
	max_memory:HashMap<String, u64>,
	thread_count:HashMap<String, u64>,
	syscall_count:HashMap<String, u64>,
}

fn main() {
	let electron_data = root_dir().join("gh-pages").join("electron-data.json");

	let electron_recent = root_dir().join("gh-pages").join("electron-recent.json");

	// current data
	let current_data_buffer = BufReader::new(
		File::open(root_dir().join("bench.json")).expect("Unable to read current data file"),
	);

	let current_data:BenchResult =
		serde_json::from_reader(current_data_buffer).expect("Unable to read current data buffer");

	// all data's
	let all_data_buffer =
		BufReader::new(File::open(&electron_data).expect("Unable to read all data file"));

	let mut all_data:Vec<BenchResult> =
		serde_json::from_reader(all_data_buffer).expect("Unable to read all data buffer");

	// add current data to alls data
	all_data.push(current_data);

	// use only latest 20 elements from alls data
	let recent:Vec<BenchResult>;

	if all_data.len() > 20 {
		recent = all_data[all_data.len() - 20..].to_vec();
	} else {
		recent = all_data.clone();
	}

	write_json(
		&electron_data,
		&serde_json::to_value(&all_data).expect("Unable to build final json (alls)"),
	);

	write_json(
		&electron_recent,
		&serde_json::to_value(&recent).expect("Unable to build final json (recent)"),
	);
}
