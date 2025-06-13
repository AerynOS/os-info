// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

use crate::{OsInfo, TechnologyCapabilities};
use std::fs;
use std::path::Path;

use snafu::{ResultExt as _, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to parse JSON"))]
    JsonParse { source: serde_json::Error },

    #[snafu(display("failed to read file"))]
    FsRead { source: std::io::Error },
}

/// Loads and parses an os-info.json string
pub fn load_os_info(content: &str) -> Result<OsInfo, Error> {
    serde_json::from_str(content).context(JsonParseSnafu)
}

/// Loads and parses a technology capabilities json string
pub fn load_technology(content: &str) -> Result<TechnologyCapabilities, Error> {
    serde_json::from_str(content).context(JsonParseSnafu)
}

/// Loads and parses an os-info.json file from a path
pub fn load_os_info_from_path<P: AsRef<Path>>(path: P) -> Result<OsInfo, Error> {
    let content = fs::read_to_string(path).context(FsReadSnafu)?;
    load_os_info(&content)
}

/// Loads and parses a technology capabilities json file from a path
pub fn load_technology_from_path<P: AsRef<Path>>(path: P) -> Result<TechnologyCapabilities, Error> {
    let content = fs::read_to_string(path).context(FsReadSnafu)?;
    load_technology(&content)
}
