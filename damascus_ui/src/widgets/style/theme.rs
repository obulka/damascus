// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;
use macro_rules_attribute::derive;

use crate::EnumTraits;

// TODO Support Custom Themes
#[derive(Copy, Default, EnumTraits!)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl From<iced::Theme> for Theme {
    fn from(theme: iced::Theme) -> Self {
        match theme {
            iced::Theme::Light => Theme::Light,
            iced::Theme::Dark => Theme::Dark,
            iced::Theme::Dracula => Theme::Dracula,
            iced::Theme::Nord => Theme::Nord,
            iced::Theme::SolarizedLight => Theme::SolarizedLight,
            iced::Theme::SolarizedDark => Theme::SolarizedDark,
            iced::Theme::GruvboxLight => Theme::GruvboxLight,
            iced::Theme::GruvboxDark => Theme::GruvboxDark,
            iced::Theme::CatppuccinLatte => Theme::CatppuccinLatte,
            iced::Theme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            iced::Theme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            iced::Theme::CatppuccinMocha => Theme::CatppuccinMocha,
            iced::Theme::TokyoNight => Theme::TokyoNight,
            iced::Theme::TokyoNightStorm => Theme::TokyoNightStorm,
            iced::Theme::TokyoNightLight => Theme::TokyoNightLight,
            iced::Theme::KanagawaWave => Theme::KanagawaWave,
            iced::Theme::KanagawaDragon => Theme::KanagawaDragon,
            iced::Theme::KanagawaLotus => Theme::KanagawaLotus,
            iced::Theme::Moonfly => Theme::Moonfly,
            iced::Theme::Nightfly => Theme::Nightfly,
            iced::Theme::Oxocarbon => Theme::Oxocarbon,
            iced::Theme::Ferra => Theme::Ferra,
            _ => Theme::Dark,
        }
    }
}

impl From<Theme> for iced::Theme {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
            Theme::Dracula => iced::Theme::Dracula,
            Theme::Nord => iced::Theme::Nord,
            Theme::SolarizedLight => iced::Theme::SolarizedLight,
            Theme::SolarizedDark => iced::Theme::SolarizedDark,
            Theme::GruvboxLight => iced::Theme::GruvboxLight,
            Theme::GruvboxDark => iced::Theme::GruvboxDark,
            Theme::CatppuccinLatte => iced::Theme::CatppuccinLatte,
            Theme::CatppuccinFrappe => iced::Theme::CatppuccinFrappe,
            Theme::CatppuccinMacchiato => iced::Theme::CatppuccinMacchiato,
            Theme::CatppuccinMocha => iced::Theme::CatppuccinMocha,
            Theme::TokyoNight => iced::Theme::TokyoNight,
            Theme::TokyoNightStorm => iced::Theme::TokyoNightStorm,
            Theme::TokyoNightLight => iced::Theme::TokyoNightLight,
            Theme::KanagawaWave => iced::Theme::KanagawaWave,
            Theme::KanagawaDragon => iced::Theme::KanagawaDragon,
            Theme::KanagawaLotus => iced::Theme::KanagawaLotus,
            Theme::Moonfly => iced::Theme::Moonfly,
            Theme::Nightfly => iced::Theme::Nightfly,
            Theme::Oxocarbon => iced::Theme::Oxocarbon,
            Theme::Ferra => iced::Theme::Ferra,
        }
    }
}
