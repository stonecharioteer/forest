// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_shared3::bigint::bigint_ser;
pub use fvm_shared3::bigint::bigint_ser::{BigIntDe, BigIntSer};
use fvm_shared3::bigint::BigInt as BigInt_v3;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BigInt(#[serde(with = "bigint_ser")] BigInt_v3);

impl Deref for BigInt {
    type Target = BigInt_v3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BigInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<BigInt_v3> for BigInt {
    fn from(other: BigInt_v3) -> Self {
        BigInt(other)
    }
}
