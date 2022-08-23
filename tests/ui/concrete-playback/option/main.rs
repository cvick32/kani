// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// kani-flags: --harness harness --enable-unstable --concrete-playback=Print

#[kani::proof]
pub fn harness() {
    let option_1: Option<u8> = kani::any();
    let option_2: Option<u8> = kani::any();
    assert!(!(option_1 == Some(101) && option_2 == None));
}