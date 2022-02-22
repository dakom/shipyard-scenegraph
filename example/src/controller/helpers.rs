use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn get_canvas_x_y(canvas:&web_sys::HtmlCanvasElement, event:&web_sys::Event) -> (i32, i32) {
    let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
    let rect = canvas.get_bounding_client_rect();
    let (client_x, client_y) = (event.client_x(), event.client_y());
    let (x, y) = (client_x - (rect.left().round() as i32), client_y - (rect.top().round() as i32)); 

    let y = (canvas.height() as i32) - y;
    (x, y)
}

