// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

pub fn split_pascal_words(pascal: String) -> Vec<String> {
    let mut words = Vec::<String>::new();
    let mut word = String::new();

    for character in pascal.chars() {
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
