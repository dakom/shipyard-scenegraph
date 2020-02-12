use crate::renderer::SceneRenderer;
use crate::world::init_world;
use crate::config::get_media_href;
use crate::geometry::*;
use crate::components::*;
use crate::systems::{self, TICK};
use crate::input;
use shipyard_scenegraph as sg;

use std::rc::{Rc};
use std::cell::{RefCell};
use gloo_events::{EventListener};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use shipyard::prelude::*;
use web_sys::{HtmlElement, HtmlCanvasElement};
use wasm_bindgen_futures::future_to_promise;
use awsm_web::window::{get_window_size};
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    WebGl1Renderer,
    get_texture_size,
    WebGlTextureSource
};

pub fn start() -> Result<js_sys::Promise, JsValue> {

    let window = web_sys::window().ok_or("should have a Window")?;
    let document = window.document().ok_or("should have a Document")?;
    let body = document.body().ok_or("should have a Body")?;

    let loading: HtmlElement = document.create_element("div")?.dyn_into()?;
    loading.set_class_name("loading");
    loading.set_text_content(Some("Loading..."));
    body.append_child(&loading)?;


    let future = async move {
        let vertex = fetch::text(&get_media_href("vertex.glsl")).await?;
        let fragment = fetch::text(&get_media_href("fragment.glsl")).await?;

        let (stage_width, stage_height) = get_window_size(&window).unwrap();

        body.remove_child(&loading)?;
        let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;

        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let renderer = WebGl1Renderer::new(gl)?;

        let scene_renderer = SceneRenderer::new(renderer, &vertex, &fragment)?;

        let world = Rc::new(init_world(
            Area{width: stage_width, height: stage_height},
            scene_renderer
        ));

        create_squares(&world, stage_width as f64, stage_height as f64);

        systems::register_workloads(&world);
        let on_resize = {
            let window = window.clone();
            let world = Rc::clone(&world);
            move |_: &web_sys::Event| {
                let (width, height) = get_window_size(&window).unwrap();
                world.borrow::<Unique<NonSendSync<&mut SceneRenderer>>>().renderer.resize(width, height);
                let mut stage_area = world.borrow::<Unique<&mut StageArea>>();
                stage_area.0.width = width;
                stage_area.0.height = height;
            }
        };

        on_resize(&web_sys::Event::new("").unwrap());

        EventListener::new(&window, "resize", on_resize).forget();



        //start the game loop!
        let tick = Raf::new({
            let world = Rc::clone(&world);

            move |timestamp| {
                world.run_workload(TICK);
            }
        });

        input::start(world.clone(), &canvas);

        std::mem::forget(Box::new(tick));
        Ok(JsValue::null())
    };

    Ok(future_to_promise(future))
}

fn create_squares(world:&World, stage_width: f64, stage_height: f64) {


    let mut depth = 0.0;
    let mut create_square = |width: u32, height: u32, r: f64, g: f64, b: f64| {

        let entity = sg::create_entity(
            world, 
            Some(sg::Vec3::new(
                    0.5,
                    0.5,
                    depth
            )),
            /*
            Some(sg::Vec3::new(
                    0.5 * (stage_width - (width as f64)), 
                    0.5 * (stage_height - (height as f64)),
                    depth
            )),
            */
            None,
            Some(sg::Vec3::new(width as f64, height as f64, 1.0))
        );

        {
            let (entities, mut areas, mut colors) = world.borrow::<(EntitiesMut, &mut ImageArea, &mut Color)>();
            entities.add_component(
                (&mut areas, &mut colors), 
                (ImageArea (Area { width, height}), Color (r,g,b, 1.0)),
                entity
            );
        }

        depth += 1.0;
    };

    create_square(400, 400, 1.0, 0.0, 0.0);
    //create_square(200, 200, 0.0, 1.0, 0.0);
    //create_square(100, 100, 0.0, 0.0, 1.0);
    //sg::sort_pack_by_depth_back_to_front(world);
}

/// Until Raf is availble in gloo...
struct Raf {
    state: Rc<RefCell<Option<RafState>>>,
}

struct RafState {
    id: i32,
    closure: Closure<dyn FnMut(f64)>,
}

impl Raf {
    fn new<F>(mut callback: F) -> Self where F: FnMut(f64) + 'static {
        let state: Rc<RefCell<Option<RafState>>> = Rc::new(RefCell::new(None));

        fn schedule(callback: &Closure<dyn FnMut(f64)>) -> i32 {
            web_sys::window()
                .unwrap_throw()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap_throw()
        }

        let closure = {
            let state = state.clone();

            Closure::wrap(Box::new(move |time| {
                {
                    let mut state = state.borrow_mut();
                    let state = state.as_mut().unwrap_throw();
                    state.id = schedule(&state.closure);
                }

                callback(time);
            }) as Box<dyn FnMut(f64)>)
        };

        *state.borrow_mut() = Some(RafState {
            id: schedule(&closure),
            closure
        });

        Self { state }
    }
}

impl Drop for Raf {
    fn drop(&mut self) {
        // The take is necessary in order to prevent an Rc leak
        let state = self.state.borrow_mut().take().unwrap_throw();

        web_sys::window()
            .unwrap_throw()
            .cancel_animation_frame(state.id)
            .unwrap_throw();
    }
}