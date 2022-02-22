use shipyard::Component;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement, WebGlRenderingContext};
use wasm_bindgen::prelude::*;
use awsm_web::window::get_window_size;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Component)]
pub struct Dom {
    pub window: Window,
    pub document: Document,
    pub body: HtmlElement,
    pub canvas: HtmlCanvasElement,
    header: RefCell<Option<HtmlElement>>
}

impl Dom {
    pub fn new() -> Rc<Self> {
        let window = web_sys::window().expect_throw("should have a Window");
        let document = window.document().expect_throw("should have a Document");
        let body = document.body().expect_throw("should have a Body");

        let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap_throw().dyn_into().unwrap_throw();

        Rc::new(Self {
            window,
            document,
            body,
            canvas,
            header: RefCell::new(None)
        })
    }

    pub fn _clear_ui(&self) {
        if let Some(header) = self.header.borrow_mut().take() {
            self.body.remove_child(&header.unchecked_into()).unwrap();
        }
    }

    pub fn set_header_text(&self, text: &str) {
        if self.header.borrow().is_none() {

            let header: HtmlElement = self.document.create_element("div").unwrap_throw().dyn_into().unwrap_throw();
            header.set_class_name("header");
            self.body.append_child(&header).unwrap_throw();
            *self.header.borrow_mut() = Some(header);
        }
        self.header.borrow().as_ref().unwrap_throw().set_text_content(Some(text));
    }

    pub fn window_size(&self) -> (u32, u32) {
        get_window_size(&self.window).unwrap()
    }

    pub fn create_gl_context(&self) -> WebGlRenderingContext {
        //not using any webgl2 features so might as well stick with v1
        get_webgl_context_1(&self.canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw()
    }

}
