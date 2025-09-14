// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![allow(long_running_const_eval)]

use std::fmt::{Debug, Display};
use std::str::FromStr;

use crevice::std430::AsStd430;
use strum::{EnumCount, IntoEnumIterator};

pub mod camera;
pub mod geometry;
pub mod lights;
pub mod materials;
pub mod node_graph;
pub mod render_passes;
pub mod scene;
pub mod shaders;
pub mod textures;

pub trait DualDevice<G: Copy + Clone + AsStd430<Output = S>, S>:
    Default + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>
{
    fn to_gpu(&self) -> G;

    fn as_std430(&self) -> S {
        self.to_gpu().as_std430()
    }
}

pub trait Enumerator: IntoEnumIterator + EnumCount + Default + Display + FromStr {
    fn variant(self) -> String {
        format!("{}", self)
    }

    fn variants() -> Vec<String> {
        Self::iter().map(|variant| format!("{}", variant)).collect()
    }
}

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Enum {
    pub variant: String,
    pub variants: Vec<String>,
}

impl Enum {
    pub fn as_enumerator<E: Enumerator>(&self) -> E {
        match E::from_str(&self.variant) {
            Ok(variant) => variant,
            _ => E::default(),
        }
    }

    pub fn to_enumerator<E: Enumerator>(self) -> E {
        match E::from_str(&self.variant) {
            Ok(variant) => variant,
            _ => E::default(),
        }
    }
}

impl<E: Enumerator> From<E> for Enum {
    fn from(enumerator: E) -> Self {
        Self {
            variant: enumerator.variant(),
            variants: E::variants(),
        }
    }
}

pub trait Error: Clone + Debug + Display {
    fn as_err<E>(self) -> std::result::Result<E, Self> {
        Err(self)
    }
}
