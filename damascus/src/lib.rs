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
    AsRef<str> + Clone + IntoEnumIterator + EnumCount + Default + Debug + Display + FromStr + PartialEq
{
    fn variant(&self) -> String {
        self.as_ref().to_string()
    }

    fn variants() -> Vec<String> {
        Self::iter().map(|variant| variant.variant()).collect()
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
            word.push_str(&character.to_string());
        }

        if !word.is_empty() {
            words.push(word);
        }

        words
    }

    fn variant_pascal_case(&self) -> String {
        self.to_string()
    }

    fn variant_screaming_snake_case(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_uppercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    fn variant_pascal_snake_case(&self) -> String {
        self.variant_split_words().join("_")
    }

    fn variant_snake_case(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    fn variant_kebab_case(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn variant_camel_case(&self) -> String {
        let mut words: Vec<String> = self.variant_split_words();
        if !words.is_empty() {
            words[0] = words[0].to_lowercase();
        }
        words.join("")
    }

    fn variant_flat_case(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join("")
    }

    fn variant_upper_flat_case(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_uppercase())
            .collect::<Vec<_>>()
            .join("")
    }

    fn variant_camel_snake_case(&self) -> String {
        let mut words: Vec<String> = self.variant_split_words();
        if !words.is_empty() {
            words[0] = words[0].to_lowercase();
        }
        words.join("_")
    }

    fn variant_train_case(&self) -> String {
        self.variant_split_words().join("-")
    }

    fn variant_cobol_case(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_uppercase())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn variant_label(&self) -> String {
        self.variant_split_words()
            .into_iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn variant_pascal_label(&self) -> String {
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
    #[derive(EnumBaseTraits!)] = #[derive(
        Debug,
        Clone,
        strum::AsRefStr,
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
        crate::EnumBaseTraits!,
    )];
    #[derive(EnumHashBaseTraits!)] = #[derive(
        Eq,
        Hash,
        Ord,
        PartialOrd,
        crate::EnumBaseTraits!,
    )];
    #[derive(EnumHashTraits!)] = #[derive(
        strum::Display,
        crate::EnumHashBaseTraits!,
    )];
    #[derive(PreprocessorDirectivesBaseTraits!)] = #[derive(
        crate::EnumHashBaseTraits!,
        crate::gpu::PreprocessorDirectivesTrait!,
    )];
    #[derive(PreprocessorDirectivesTraits!)] = #[derive(
        strum::Display,
        crate::PreprocessorDirectivesBaseTraits!,
    )];
    #[derive(ErrorTraits!)] = #[derive(crate::EnumBaseTraits!, crate::ErrorTrait!)];
}

#[macro_export]
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

#[macro_export]
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

#[cfg(test)]
mod tests {
    use crate::{Enumerator, geometry::primitives::Shapes};

    #[test]
    fn test_variant_conventions() {
        assert_eq!(
            Shapes::RectangularPrismFrame.variant(),
            "RectangularPrismFrame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_pascal_case(),
            "RectangularPrismFrame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_screaming_snake_case(),
            "RECTANGULAR_PRISM_FRAME"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_pascal_snake_case(),
            "Rectangular_Prism_Frame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_kebab_case(),
            "rectangular-prism-frame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_camel_case(),
            "rectangularPrismFrame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_flat_case(),
            "rectangularprismframe"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_upper_flat_case(),
            "RECTANGULARPRISMFRAME"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_camel_snake_case(),
            "rectangular_Prism_Frame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_train_case(),
            "Rectangular-Prism-Frame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_cobol_case(),
            "RECTANGULAR-PRISM-FRAME"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_label(),
            "rectangular prism frame"
        );
        assert_eq!(
            Shapes::RectangularPrismFrame.variant_pascal_label(),
            "Rectangular Prism Frame"
        );
    }
}
