// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use itertools::Itertools;

pub fn split_pascal_words(pascal: String) -> Vec<String> {
    let mut words = Vec::<String>::new();
    let mut word = String::new();

    let characters: Vec<(char, bool)> = pascal
        .chars()
        .map(|character| (character, character.is_uppercase()))
        .collect();

    let mut previous_was_lowercase = false;
    for (index, (character, is_uppercase)) in characters.iter().enumerate() {
        if *is_uppercase {
            let next_index: usize = index + 1;
            if index != 0
                && (previous_was_lowercase
                    || characters.len() > next_index && !characters[next_index].1)
            {
                words.push(word.clone());
                word.clear();
            }
            previous_was_lowercase = false;
        } else {
            previous_was_lowercase = true;
        }
        word.push_str(&character.to_string());
    }

    if !word.is_empty() {
        words.push(word);
    }

    words
}

pub fn screaming_snake_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}

pub fn pascal_snake_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal).join("_")
}

pub fn snake_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

pub fn kebab_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn camel_from_pascal(pascal: String) -> String {
    let mut words: Vec<String> = split_pascal_words(pascal);
    if !words.is_empty() {
        words[0] = words[0].to_lowercase();
    }
    words.join("")
}

pub fn flat_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("")
}

pub fn upper_flat_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<_>>()
        .join("")
}

pub fn camel_snake_from_pascal(pascal: String) -> String {
    let mut words: Vec<String> = split_pascal_words(pascal);
    if !words.is_empty() {
        words[0] = words[0].to_lowercase();
    }
    words.join("_")
}

pub fn train_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal).join("-")
}

pub fn cobol_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn label_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal)
        .into_iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn pascal_label_from_pascal(pascal: String) -> String {
    split_pascal_words(pascal).join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_conventions() {
        assert_eq!(snake_from_pascal("InputAOV".to_string()), "input_aov");
        assert_eq!(
            snake_from_pascal("InputAOVTest".to_string()),
            "input_aov_test"
        );
        assert_eq!(
            snake_from_pascal("InputAOVTestAOVTestTest".to_string()),
            "input_aov_test_aov_test_test"
        );
        assert_eq!(
            screaming_snake_from_pascal("RectangularPrismFrame".to_string()),
            "RECTANGULAR_PRISM_FRAME"
        );
        assert_eq!(
            pascal_snake_from_pascal("RectangularPrismFrame".to_string()),
            "Rectangular_Prism_Frame"
        );
        assert_eq!(
            kebab_from_pascal("RectangularPrismFrame".to_string()),
            "rectangular-prism-frame"
        );
        assert_eq!(
            camel_from_pascal("RectangularPrismFrame".to_string()),
            "rectangularPrismFrame"
        );
        assert_eq!(
            flat_from_pascal("RectangularPrismFrame".to_string()),
            "rectangularprismframe"
        );
        assert_eq!(
            upper_flat_from_pascal("RectangularPrismFrame".to_string()),
            "RECTANGULARPRISMFRAME"
        );
        assert_eq!(
            camel_snake_from_pascal("RectangularPrismFrame".to_string()),
            "rectangular_Prism_Frame"
        );
        assert_eq!(
            train_from_pascal("RectangularPrismFrame".to_string()),
            "Rectangular-Prism-Frame"
        );
        assert_eq!(
            cobol_from_pascal("RectangularPrismFrame".to_string()),
            "RECTANGULAR-PRISM-FRAME"
        );
        assert_eq!(
            label_from_pascal("RectangularPrismFrame".to_string()),
            "rectangular prism frame"
        );
        assert_eq!(
            pascal_label_from_pascal("RectangularPrismFrame".to_string()),
            "Rectangular Prism Frame"
        );
    }
}
