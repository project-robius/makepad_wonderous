use makepad_widgets::{live_id, Cx, HttpMethod, HttpRequest, LiveId};

use super::great_wall_search_data::SearchData;

const BASE_ARTIFACT_URI: &str = "https://www.wonderous.info/met";

pub fn request_search_images(cx: &mut Cx, search_data: &Vec<SearchData>, offset: usize, limit: usize) {
    let request_id = live_id!(image_search);
    for artifact in search_data.iter().skip(offset).take(limit) {
        let url = format!(
            "{}/{}_{}.jpg", // base_uri/id_size.jpg
            BASE_ARTIFACT_URI, artifact.id, 600
        );

        let mut request = HttpRequest::new(url, HttpMethod::GET);
        request.metadata_id = LiveId::from_str(&artifact.id.to_string());
        cx.http_request(request_id, request);
    }
}
