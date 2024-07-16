pub const YEAR_TO_POSITION_RATIO: f64 = 0.5;
pub const TIMELINE_YEARS_LIMITS: [i32; 2] = [-3000, 2200];
pub const fn total_years_range() -> i32 {
    TIMELINE_YEARS_LIMITS[1] - TIMELINE_YEARS_LIMITS[0]
}

pub fn years_to_pixels(year: i32) -> f64 {
    year as f64 * YEAR_TO_POSITION_RATIO
}

use makepad_widgets::{vec4, Vec4};

#[derive(Default, Clone, PartialEq)]
pub enum WonderType {
    #[default]
    GreatWall,
    Petra,
    Colosseum,
    ChichenItza,
    MachuPichu,
    TajMahal,
    ChristTheRedeemer,
    PyramidsOfGiza,
}

impl WonderType {
    pub fn bg_color(&self) -> Vec4 {
        match *self {
            Self::GreatWall => vec4(0.404, 0.522, 0.32, 1.0), // #678551
            Self::Petra => vec4(0.102, 0.102, 0.384, 1.0),    // #1A1A62
            Self::Colosseum => vec4(0.286, 0.643, 0.624, 1.0), // #49A49F
            Self::ChichenItza => vec4(0.882, 0.812, 0.733, 1.0), // #E1CFBB
            Self::MachuPichu => vec4(0.765, 0.855, 0.824, 1.0), // #C3DAD2
            Self::TajMahal => vec4(0.392, 0.157, 0.153, 1.0), // #642827
            Self::ChristTheRedeemer => vec4(0.957, 0.463, 0.404, 1.0), // #F47667
            Self::PyramidsOfGiza => vec4(0.271, 0.29, 0.612, 1.0), // #454A9C
        }
    }

    pub fn horizontal_column(&self) -> usize {
        match *self {
            WonderType::GreatWall => 2,
            WonderType::Petra => 1,
            WonderType::Colosseum => 0,
            WonderType::ChichenItza => 0,
            WonderType::MachuPichu => 1,
            WonderType::TajMahal => 0,
            WonderType::ChristTheRedeemer => 2,
            WonderType::PyramidsOfGiza => 2,
        }
    }

    pub fn vertical_column(&self) -> usize {
        match *self {
            WonderType::GreatWall => 0,
            WonderType::Petra => 1,
            WonderType::Colosseum => 2,
            WonderType::ChichenItza => 2,
            WonderType::MachuPichu => 1,
            WonderType::TajMahal => 2,
            WonderType::ChristTheRedeemer => 0,
            WonderType::PyramidsOfGiza => 0,
        }
    }
}

#[allow(unused)]
pub struct WonderData {
    pub wonder_type: WonderType,
    pub start_year: i32,
    pub end_year: i32,
    pub image_path: &'static str,
}

pub const GREAT_WALL_DATA: WonderData = WonderData {
    wonder_type: WonderType::GreatWall,
    start_year: -700,
    end_year: 1644,
    image_path: "crate://self/resources/images/great-wall-flattened.jpg",
};
pub const PETRA_DATA: WonderData = WonderData {
    wonder_type: WonderType::Petra,
    start_year: -312,
    end_year: 100,
    image_path: "crate://self/resources/images/petra-flattened.jpg",
};
pub const COLOSSEUM_DATA: WonderData = WonderData {
    wonder_type: WonderType::Colosseum,
    start_year: 70,
    end_year: 80,
    image_path: "crate://self/resources/images/colosseum-flattened.jpg",
};
pub const CHICHENITZA_DATA: WonderData = WonderData {
    wonder_type: WonderType::ChichenItza,
    start_year: 550,
    end_year: 1550,
    image_path: "crate://self/resources/images/chichen-itza-flattened.jpg",
};
pub const MACHU_PICHU_DATA: WonderData = WonderData {
    wonder_type: WonderType::MachuPichu,
    start_year: 1450,
    end_year: 1572,
    image_path: "crate://self/resources/images/machu-picchu-flattened.jpg",
};
pub const TAJ_MAHAL_DATA: WonderData = WonderData {
    wonder_type: WonderType::TajMahal,
    start_year: 1632,
    end_year: 1653,
    image_path: "crate://self/resources/images/taj-mahal-flattened.jpg",
};
pub const CHRIST_THE_REDEEMER_DATA: WonderData = WonderData {
    wonder_type: WonderType::ChristTheRedeemer,
    start_year: 1922,
    end_year: 1931,
    image_path: "crate://self/resources/images/christ-the-redeemer-flattened.jpg",
};
pub const PYRAMIDS_OF_GIZA_DATA: WonderData = WonderData {
    wonder_type: WonderType::PyramidsOfGiza,
    start_year: -2600,
    end_year: -2500,
    image_path: "crate://self/resources/images/pyramids-of-giza-flattened.jpg",
};

// This array keeps the same order as the image dependency ones
pub const ALL_WONDERS_DATA: [WonderData; 8] = [
    GREAT_WALL_DATA,
    PETRA_DATA,
    COLOSSEUM_DATA,
    CHICHENITZA_DATA,
    MACHU_PICHU_DATA,
    TAJ_MAHAL_DATA,
    CHRIST_THE_REDEEMER_DATA,
    PYRAMIDS_OF_GIZA_DATA,
];

pub fn era_label(year: i32) -> &'static str {
    match year {
        i32::MIN..=-600 => "Prehistory",
        -601..=480 => "Classical Era",
        481..=1450 => "Early Modern Era",
        1451..=i32::MAX => "Modern Era",
    }
}