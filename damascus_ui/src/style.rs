// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;
use macro_rules_attribute::derive;

use crate::{EnumTraits, icons::Icons};

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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Preferences {
    pub border_width: f32,
    pub theme: Theme,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            border_width: 2.0,
            theme: Theme::default(),
        }
    }
}

pub fn title_bar(preferences: &Preferences) -> iced::widget::container::Style {
    let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(preferences.theme);
    let palette: &iced::theme::palette::Extended = theme.extended_palette();

    iced::widget::container::Style {
        text_color: Some(palette.background.strong.text),
        background: Some(palette.background.strong.color.into()),
        ..Default::default()
    }
}

pub fn title_bar_focused(preferences: &Preferences) -> iced::widget::container::Style {
    let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(preferences.theme);
    let palette: &iced::theme::palette::Extended = theme.extended_palette();

    let mut style = title_bar(preferences);

    style.border = iced::Border {
        width: preferences.border_width,
        color: iced::Color::TRANSPARENT,
        ..iced::Border::default()
    };

    style
}

pub fn pane(preferences: &Preferences) -> iced::widget::container::Style {
    let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(preferences.theme);
    let palette: &iced::theme::palette::Extended = theme.extended_palette();

    iced::widget::container::Style {
        background: Some(palette.background.weak.color.into()),
        border: iced::Border {
            width: preferences.border_width,
            color: palette.background.strong.color,
            ..iced::Border::default()
        },
        ..Default::default()
    }
}

pub fn pane_focused(preferences: &Preferences) -> iced::widget::container::Style {
    let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(preferences.theme);
    let palette: &iced::theme::palette::Extended = theme.extended_palette();

    let mut style = pane(preferences);

    style.border = iced::Border {
        width: preferences.border_width,
        color: palette.primary.strong.color,
        ..iced::Border::default()
    };

    style
}

pub fn close_button<Message>() -> iced::widget::Button<'static, Message> {
    iced::widget::button(Icons::Close.as_svg().width(16).height(16))
        .style(iced::widget::button::secondary)
        .padding(3)
}
