// SPDX-FileCopyrightText: Copyright © 2025 AerynOS Developers
//
// SPDX-License-Identifier: MPL-2.0

use crate::{OSInfo, TechnologyCapabilities};
use serde_json::Result as JsonResult;

/// Loads and parses an os-info.json string
pub fn load_os_info(content: &str) -> JsonResult<OSInfo> {
    serde_json::from_str(content)
}

/// Loads and parses a technology capabilities json string
pub fn load_technology(content: &str) -> JsonResult<TechnologyCapabilities> {
    serde_json::from_str(content)
}
