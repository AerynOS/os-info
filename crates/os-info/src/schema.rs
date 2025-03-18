// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

use crate::{OSInfo, TechnologyCapabilities};
use std::fs;
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Loads and parses an os-info.json string
pub fn load_os_info(content: &str) -> Result<OSInfo, Error> {
    Ok(serde_json::from_str(content)?)
}

/// Loads and parses a technology capabilities json string
pub fn load_technology(content: &str) -> Result<TechnologyCapabilities, Error> {
    Ok(serde_json::from_str(content)?)
}

/// Loads and parses an os-info.json file from a path
pub fn load_os_info_from_path<P: AsRef<Path>>(path: P) -> Result<OSInfo, Error> {
    let content = fs::read_to_string(path)?;
    load_os_info(&content)
}

/// Loads and parses a technology capabilities json file from a path
pub fn load_technology_from_path<P: AsRef<Path>>(path: P) -> Result<TechnologyCapabilities, Error> {
    let content = fs::read_to_string(path)?;
    load_technology(&content)
}
