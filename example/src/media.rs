use awsm_web::loaders::fetch::fetch_url;
use wasm_bindgen::UnwrapThrowExt;
use crate::config::get_media_href;
pub struct Media {
    pub vertex_shader: String,
    pub forward_fragment_shader: String,
    pub picker_fragment_shader: String,
}

impl Media {
    pub async fn load() -> Self {

        let vertex_shader = fetch_url(&get_media_href("vertex.glsl")).await.unwrap_throw().text().await.unwrap_throw();
        let forward_fragment_shader = fetch_url(&get_media_href("forward-fragment.glsl")).await.unwrap_throw().text().await.unwrap_throw();
        let picker_fragment_shader = fetch_url(&get_media_href("picker-fragment.glsl")).await.unwrap_throw().text().await.unwrap_throw();

        Self {
            vertex_shader,
            forward_fragment_shader,
            picker_fragment_shader,
        }
    }
}
