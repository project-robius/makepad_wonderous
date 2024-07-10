use makepad_widgets::{Cx, HttpMethod, HttpRequest, LiveId};

use crate::artifacts::data::great_wall_search_data::SEARCH_DATA;

// use crate::timeline::wonders_data::WonderType;

// const BASE_ARTIFACT_URI: &str = "https://collectionapi.metmuseum.org/public/collection/v1/objects/";
// ! as first char indicates a priority query
// const BASE_QUERY_URI: &str = "https://collectionapi.metmuseum.org/public/collection/v1/search?hasImage=true&";

const BASE_ARTIFACT_URI: &str = "https://www.wonderous.info/met";

const GREAT_WALL_QUERY: (&str, &str) = (
    // -700 1650
    "!dateBegin=-700&dateEnd=1650&artistOrCulture=true&q=china", // 4540
    "geolocation=china&artistOrCulture=true&q=china", // 14181
);

// pub struct Artifact {
//     pub id: String,
//     pub title: String,
//     pub wonder: WonderType,
//     pub culture: String,
//     pub date: String,
// }

// pub fn highlight_data() -> Vec<Artifact> {
//     vec![
//         Artifact {
//             id: "79091".to_string(),
//             title: "Cape".to_string(),
//             wonder: WonderType::GreatWall,
//             culture: "French".to_string(),
//             date: "second half 16th century".to_string(),
//         },
//         Artifact {
//             id: "781812".to_string(),
//             title: "Censer in the form of a mythical beast".to_string(),
//             wonder: WonderType::GreatWall,
//             culture: "China".to_string(),
//             date: "early 17th century".to_string(),
//         },
//         Artifact {
//             id: "40213".to_string(),
//             title: "Dish with peafowls and peonies".to_string(),
//             wonder: WonderType::GreatWall,
//             culture: "China".to_string(),
//             date: "early 15th century".to_string(),
//         },
//         Artifact {
//             id: "40765".to_string(),
//             title: "Base for a mandala".to_string(),
//             wonder: WonderType::GreatWall,
//             culture: "China".to_string(),
//             date: "15th century".to_string(),
//         },
//         Artifact {
//             id: "57612".to_string(),
//             title: "Bodhisattva Manjushri as Tikshna-Manjushri (Minjie Wenshu)".to_string(),
//             wonder: WonderType::GreatWall,
//             culture: "China".to_string(),
//             date: "".to_string(),
//         },
//         Artifact {
//             id: "666573".to_string(),
//             title: "Tripod incense burner with lid".to_string(),
//             wonder: WonderType::GreatWall,
//             culture: "China".to_string(),
//             date: "early 15th century".to_string(),
//         },
//     ]
// }

pub fn request_search_images(cx: &mut Cx, offset: usize, limit: usize) {
    println!("Requesting at {offset}");
    for artifact in SEARCH_DATA.iter().skip(offset).take(limit) {
        let url = format!(
            "{}/{}_{}.jpg", // base_uri/id_size.jpg
            BASE_ARTIFACT_URI, artifact.id, 600
        );
        let request_id = LiveId::from_str(&artifact.id.to_string());
        // println!("Requesting: {}", url);

        let request = HttpRequest::new(url, HttpMethod::GET);
        cx.http_request(request_id, request);
    }
}
