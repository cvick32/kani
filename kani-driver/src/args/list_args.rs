// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Implements the subcommand handling of the list subcommand

use std::path::PathBuf;

use clap::Parser;

/// List harness information for the current package.
#[derive(Debug, Parser)]
pub struct CargoListArgs {
    // Only list Kani data for the given file.
    #[arg(long)]
    pub specific_file: Option<PathBuf>,
}