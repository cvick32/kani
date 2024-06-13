// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Implements the subcommand handling of the playback subcommand

use crate::args::cargo::CargoTestArgs;
use clap::Parser;

/// List all harnesses in current package.
#[derive(Debug, Parser)]
pub struct CargoListArgs {
    #[command(flatten)]
    pub cargo: CargoTestArgs,
}