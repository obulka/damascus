// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![allow(long_running_const_eval)]

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crevice::std430::AsStd430;
use glam::Mat4;
use macro_rules_attribute::derive_alias;
use strum::{EnumCount, IntoEnumIterator};

pub mod camera;
pub mod geometry;
pub mod gpu;
pub mod graph;
pub mod lights;
pub mod materials;
pub mod textures;
pub mod time;

pub trait DualDevice<
    G: Copy
        + Clone
        + PartialEq
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + AsStd430<Output = S>,
    S,
>: Default + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>
{
    fn to_gpu(&self) -> G;

    fn as_std430(&self) -> S {
        self.to_gpu().as_std430()
    }
}

pub trait Transformable {
    fn transform(&mut self, local_to_world: &Mat4);
}

pub trait Enumerator:
    IntoEnumIterator + EnumCount + Default + Display + FromStr + PartialEq
{
    fn variant(self) -> String {
        self.to_string()
    }

    fn variants() -> Vec<String> {
        Self::iter().map(|variant| variant.to_string()).collect()
    }

    fn variant_matches(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn variant_split_words(&self) -> Vec<String> {
        let mut words = Vec::<String>::new();
        let mut word = String::new();

        for character in self.to_string().chars() {
            if character.is_uppercase() && !word.is_empty() {
                words.push(word.clone());
                word.clear();
            }
            word.push_str(&character.to_lowercase().to_string());
        }

        if !word.is_empty() {
            words.push(word);
        }

        words
    }

    fn variant_snake_case(&self) -> String {
        self.variant_split_words().join("_")
    }

    fn variant_label(&self) -> String {
        self.variant_split_words().join(" ")
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

pub trait Errors: Enumerator {}

derive_alias! {
    #[derive(BaseEnumTraits!)] = #[derive(
        Debug,
        Clone,
        strum::EnumIter,
        strum::EnumCount,
        strum::EnumString,
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        crate::EnumTrait!,
    )];
    #[derive(EnumTraits!)] = #[derive(
        strum::Display,
        crate::BaseEnumTraits!,
    )];
    #[derive(EnumBaseHashTraits!)] = #[derive(
        Eq,
        Hash,
        Ord,
        PartialOrd,
        crate::BaseEnumTraits!,
    )];
    #[derive(EnumHashTraits!)] = #[derive(
        strum::Display,
        crate::EnumBaseHashTraits!,
    )];
    #[derive(PreprocessorDirectivesBaseTraits!)] = #[derive(
        crate::EnumBaseHashTraits!,
        crate::gpu::PreprocessorDirectivesTrait!,
    )];
    #[derive(PreprocessorDirectivesTraits!)] = #[derive(
        strum::Display,
        crate::PreprocessorDirectivesBaseTraits!,
    )];
    #[derive(ErrorTraits!)] = #[derive(crate::BaseEnumTraits!, crate::ErrorTrait!)];
}

macro_rules! EnumTrait {
    (
        $( #[$attr:meta] )*
        $pub:vis
        enum $type:ident {
            $($variants:tt)*
        }
    ) => {
        impl crate::Enumerator for $type {}
    };
}

pub(crate) use EnumTrait;

macro_rules! ErrorTrait {
    (
    $( #[$attr:meta] )*
    $pub:vis
    enum $type:ident {
        $($variants:tt)*
    }
) => {
        impl crate::Errors for $type {}
    };
}

pub(crate) use ErrorTrait;

macro_rules! impl_slot_map_indexing {
    ($graph:ty, $id_type:ty, $output_type:ty, $arena:ident) => {
        impl std::ops::Index<$id_type> for $graph {
            type Output = $output_type;

            fn index(&self, index: $id_type) -> &Self::Output {
                self.$arena.get(index).unwrap_or_else(|| {
                    panic!(
                        "{} index error for {}[{:?}]",
                        stringify!($id_type),
                        stringify!($arena),
                        index
                    )
                })
            }
        }

        impl std::ops::IndexMut<$id_type> for $graph {
            fn index_mut(&mut self, index: $id_type) -> &mut Self::Output {
                self.$arena.get_mut(index).unwrap_or_else(|| {
                    panic!(
                        "{} index error for {}[{:?}]",
                        stringify!($id_type),
                        stringify!($arena),
                        index
                    )
                })
            }
        }
    };
}

pub(crate) use impl_slot_map_indexing;
