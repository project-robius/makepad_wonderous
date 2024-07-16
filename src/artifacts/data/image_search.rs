use crate::artifacts::data::great_wall_search_data::SEARCH_DATA;
use makepad_widgets::{live_id, Cx, HttpMethod, HttpRequest, LiveId};

const BASE_ARTIFACT_URI: &str = "https://www.wonderous.info/met";

pub fn request_search_images(cx: &mut Cx, offset: usize, limit: usize) {
    let request_id = live_id!(image_search);
    for artifact in SEARCH_DATA.iter().skip(offset).take(limit) {
        let url = format!(
            "{}/{}_{}.jpg", // base_uri/id_size.jpg
            BASE_ARTIFACT_URI, artifact.id, 600
        );

        let mut request = HttpRequest::new(url, HttpMethod::GET);
        request.metadata_id = LiveId::from_str(&artifact.id.to_string());
        cx.http_request(request_id, request);
    }
}
