// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Recipe parsing and handling library

mod monitoring;
mod parser;

use monitoring::Monitoring;
pub use parser::*;

#[cfg(feature = "stone")]
mod stone;

#[cfg(feature = "ypkg")]
mod ypkg;

// Source recipe details
#[derive(Debug)]
pub struct Recipe {
    // Name of the recipe source
    pub name: String,

    // Version of the recipe
    pub version: String,

    // Monitoring data
    pub monitoring: Option<Monitoring>,
}
