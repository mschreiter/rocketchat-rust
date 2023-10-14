use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Group {
    SmileysAndEmotion,
    PeopleAndBody,
    AnimalsAndNature,
    FoodAndDrink,
    TravelAndPlaces,
    Activities,
    Objects,
    Symbols,
    Flags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SkinTone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
    LightAndMediumLight,
    LightAndMedium,
    LightAndMediumDark,
    LightAndDark,
    MediumLightAndLight,
    MediumLightAndMedium,
    MediumLightAndMediumDark,
    MediumLightAndDark,
    MediumAndLight,
    MediumAndMediumLight,
    MediumAndMediumDark,
    MediumAndDark,
    MediumDarkAndLight,
    MediumDarkAndMediumLight,
    MediumDarkAndMedium,
    MediumDarkAndDark,
    DarkAndLight,
    DarkAndMediumLight,
    DarkAndMedium,
    DarkAndMediumDark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct UnicodeVersion {
    major: u32,
    minor: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Emoji {
    emoji: String,
    name: String,
    unicode_version: UnicodeVersion,
    group: Group,
    skin_tone: Option<(u16, u8, SkinTone)>,
}

impl Emoji {
    pub fn new() -> Self {
        Self {
            emoji: String::new(),
            name: String::new(),
            unicode_version: UnicodeVersion {
                major: 0,
                minor: 0,
            },
            group: Group::SmileysAndEmotion,
            skin_tone: None,
        }
    }
    pub fn emoji(&self) -> &str {
        &self.emoji
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn unicode_version(&self) -> &UnicodeVersion {
        &self.unicode_version
    }
    pub fn group(&self) -> &Group {
        &self.group
    }
    pub fn skin_tone(&self) -> Option<&(u16, u8, SkinTone)> {
        self.skin_tone.as_ref()
    }
}

impl Default for Emoji {
    fn default() -> Self {
        Self::new()
    }
}
