use crate::{
    dom::Dom,
    controller::listeners::InputListeners,
    mainloop,
    media::Media,
    renderer::SceneRenderer,
    world::init_world,
};
use std::rc::Rc;
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use shipyard::*;
use awsm_web::tick::{Raf, MainLoop, MainLoopOptions};
use awsm_web::webgl::ResizeStrategy;

pub async fn init() -> Result<(), JsValue> {

    init_logger();


    let dom = Dom::new();

    dom.set_header_text("loading...");

    let media = Media::load().await;

    dom.set_header_text("prepping...");

    let scene_renderer = SceneRenderer::new(dom.create_gl_context(), &media)?;

    let (stage_width, stage_height) = dom.window_size();

    let world = Rc::new(init_world(
        scene_renderer
    ));

    let on_resize = {
        let dom = Rc::clone(&dom);
        let world = Rc::clone(&world);
        move |_: &web_sys::Event| {
            let (width, height) = dom.window_size();
            world.borrow::<NonSendSync<UniqueViewMut<SceneRenderer>>>().unwrap().resize(ResizeStrategy::All(width, height));
        }
    };

    on_resize(&web_sys::Event::new("").unwrap());

    EventListener::new(&dom.window, "resize", on_resize).forget();

    //start the game loop!
    dom.set_header_text("Drag the squares around. Children should move with the parent");

    let mut main_loop = MainLoop::new(
        MainLoopOptions::default(),
        {
            let world = Rc::clone(&world);
            move |time, delta| mainloop::begin(&world, time, delta)
        },
        {
            let world = Rc::clone(&world);
            move |delta| mainloop::update(&world, delta)
        },
        {
            let world = Rc::clone(&world);
            move |interpolation| mainloop::draw(&world, interpolation)
        },
        {
            let world = Rc::clone(&world);
            move |fps, abort| mainloop::end(&world, fps, abort)
        },
    );

    let tick = Raf::new({
        move |ts| {
            main_loop.tick(ts);
        }
    });

    crate::squares::create(&world, stage_width as f64, stage_height as f64);

    // these just run forever
    std::mem::forget(Box::new(tick));
    std::mem::forget(Box::new(InputListeners::new(dom.clone(), world)));

    Ok(())
}

// enable logging and panic hook only during debug builds
cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn init_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

