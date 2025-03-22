use non_empty_string::NonEmptyString;
use serde::{Deserialize, Serialize};

mod game_2025;

use game_2025::GameData2025;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "game")]
pub enum GameSpecificData {
    #[default]
    None,
    Reefscape2025(GameData2025),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum AlliancePosition {
    #[default]
    None,
    #[serde(rename = "Red 1")]
    Red1,
    #[serde(rename = "Red 2")]
    Red2,
    #[serde(rename = "Red 3")]
    Red3,
    #[serde(rename = "Blue 1")]
    Blue1,
    #[serde(rename = "Blue 2")]
    Blue2,
    #[serde(rename = "Blue 3")]
    Blue3,
}

impl Into<String> for AlliancePosition {
    fn into(self) -> String {
        match self {
            AlliancePosition::None => String::from("None"),
            AlliancePosition::Red1 => String::from("Red1"),
            AlliancePosition::Red2 => String::from("Red2"),
            AlliancePosition::Red3 => String::from("Red3"),
            AlliancePosition::Blue1 => String::from("Blue1"),
            AlliancePosition::Blue2 => String::from("Blue2"),
            AlliancePosition::Blue3 => String::from("Blue3"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(into = "Vec<String>")]
pub struct GameData {
    pub scouter: NonEmptyString,
    pub team: NonEmptyString,
    pub match_number: NonEmptyString,
    pub alliance_position: AlliancePosition,
    pub commentary: String,
    pub is_test: bool,
    pub game_data: GameSpecificData,
}

impl Into<Vec<String>> for GameData {
    fn into(self) -> Vec<String> {
        let mut fields = vec![
            self.scouter.into(),
            self.team.into(),
            self.match_number.into(),
            self.alliance_position.into(),
            self.commentary,
        ];
        match self.game_data {
            GameSpecificData::None => {}
            GameSpecificData::Reefscape2025(data) => fields.append(&mut data.into())
        };

        fields
    }
}
