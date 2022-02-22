pub mod item;
use item::*;
mod picker;
use picker::ScenePicker;

use crate::media::Media;
use shipyard_scenegraph::prelude::*;
use shipyard::*;
use nalgebra::Matrix4;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;
use std::ops::{Deref, DerefMut};
use awsm_web::webgl::{
    BufferMask,
    WebGl1Renderer,
    AttributeOptions,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    Id,
    BeginMode,
    GlToggle,
    BlendFactor,
    ShaderType,
    ResizeStrategy
};

pub type RendererViewMut<'a> = NonSendSync<UniqueViewMut<'a, SceneRenderer>>;

#[derive(Component)]
pub struct SceneRenderer {
    renderer: WebGl1Renderer,
    picker_program_id: Id,
    forward_program_id: Id,
    picker: Option<ScenePicker>
}

impl Deref for SceneRenderer {
    type Target = WebGl1Renderer;

    fn deref(&self) -> &WebGl1Renderer {
        &self.renderer
    }
}

impl DerefMut for SceneRenderer {
    fn deref_mut(&mut self) -> &mut WebGl1Renderer {
        &mut self.renderer
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pass {
    Picker,
    Forward
}

impl SceneRenderer {
    pub fn new (ctx: WebGlRenderingContext, media: &Media) -> Result<Self, awsm_web::errors::Error> {
        let mut renderer = WebGl1Renderer::new(ctx).unwrap_throw();

        //This demo is specifically using webgl1, which needs to register the extension
        //Everything else is the same API as webgl2 :)
        renderer.register_extension_instanced_arrays()?;
        
        let vertex_id = renderer.compile_shader(&media.vertex_shader, ShaderType::Vertex)?;
        let forward_fragment_id = renderer.compile_shader(&media.forward_fragment_shader, ShaderType::Fragment)?;
        let picker_fragment_id = renderer.compile_shader(&media.picker_fragment_shader, ShaderType::Fragment)?;
        
        let forward_program_id = renderer.compile_program(&[vertex_id, forward_fragment_id])?;
        let picker_program_id = renderer.compile_program(&[vertex_id, picker_fragment_id])?;

        //create quad data and get a buffer id
        let geom_id = renderer.create_buffer()?;

        renderer.upload_buffer_to_attribute_name(
            geom_id,
            BufferData::new(
                &QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
                ),
                "a_vertex",
                &AttributeOptions::new(2, DataType::Float),
                )?;



        let _texture_id = renderer.create_texture()?;

        Ok(Self { renderer, forward_program_id, picker_program_id, picker: None} )
    }

    pub fn resize(&mut self, strategy: ResizeStrategy) {
        if self.renderer.resize(strategy) {
            if let Some(picker) = self.picker.take() {
                picker.destroy(&mut self.renderer).unwrap_throw();
            }
            let (_, _, width, height) = self.renderer.get_viewport();
            self.picker = Some(ScenePicker::new(&mut self.renderer, width, height).unwrap());
        }
    }

    pub fn get_picker_index(&mut self, x: u32, y: u32) -> Result<Option<u32>, awsm_web::errors::Error> {
        match self.picker.as_ref() {
            None => Ok(None),
            Some(picker) => {
                let color = picker.get_color(&mut self.renderer, x, y)?;
                let index = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32);
                if index > 0 {
                    Ok(Some(index - 1))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn draw_scene(&mut self, 
        world_transforms: &View<WorldTransform>,
        img_areas: &View<ImageArea>, 
        colors: &View<Color>, 
        interactables: &View<Interactable>,
        pass: Pass,
    ) -> Result<(), awsm_web::errors::Error> {

        let program_id = match pass {
            Pass::Picker => {
                if let Some(picker) = &mut self.picker {
                    picker.start(&mut self.renderer)?;
                    self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
                    self.picker_program_id
                } else {
                    return Ok(());
                }
            },
            Pass::Forward => {
                self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
                self.forward_program_id
            }
        };

        self.clear(&[
            BufferMask::ColorBufferBit,
            BufferMask::DepthBufferBit,
        ]);

        self.toggle(GlToggle::CullFace, true);
        self.toggle(GlToggle::Blend, true);
        self.toggle(GlToggle::DepthTest, true);
        self.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);


        //will already be activated but internally that's a noop if true
        self.activate_program(program_id)?;

        //Build our matrices (must cast to f32)
        let camera_mat = Matrix4::new_orthographic( 0.0, self.canvas.width() as f32, 0.0, self.canvas.height() as f32, -100.0, 100.0);

        //Upload them to the GPU
        self.upload_uniform_mat_4_name("u_camera", &camera_mat.as_slice())?;


        //renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;

        if pass == Pass::Forward {
            for (transform, img_area, color) in (world_transforms, img_areas, colors).iter() {
                self.draw_geom(transform, img_area)?;
                self.upload_uniform_fvals_4_name("u_color", color.get_shader_tuple())?;
                self.draw_arrays(BeginMode::TriangleStrip, 0, 4);
            }
        }

        for (transform, img_area, color, interactable) in (world_transforms, img_areas, colors, interactables).iter() {

            self.draw_geom(transform, img_area)?;
            match pass {
                Pass::Picker => {
                    let index = interactable.0 + 1;
                    let divisor = 0xFF as f32;
                    let r = (0xFF & (index >> 16)) as f32 / divisor;
                    let g = (0xFF & (index >> 8)) as f32 / divisor;
                    let b = (0xFF & index) as f32 / divisor; 
                    self.upload_uniform_fvals_4_name("u_color", (r,g,b,1.0))?;
                },
                Pass::Forward => {
                    self.upload_uniform_fvals_4_name("u_color", color.get_shader_tuple())?;
                }
            }

            self.draw_arrays(BeginMode::TriangleStrip, 0, 4);
        }


        if pass == Pass::Picker {
            if let Some(picker) = &mut self.picker {
                picker.finish(&mut self.renderer)?;
            }
        }
        Ok(())
    }


    fn draw_geom(&mut self, transform: &WorldTransform, img_area: &ImageArea) -> Result<(), awsm_web::errors::Error> {
        let mut scratch:[f32;16] = [0.0;16];

        transform.write_to_vf32(&mut scratch);
        self.upload_uniform_mat_4_name("u_model", &scratch)?;
        self.upload_uniform_fvals_2_name("u_size", img_area.get_shader_tuple())?;

        Ok(())
    }
}


pub fn render_sys(
    mut renderer: RendererViewMut, 
    world_transforms: View<WorldTransform>, 
    img_areas:View<ImageArea>,
    colors:View<Color>,
    interactables:View<Interactable>,
) {
    renderer.draw_scene(&world_transforms, &img_areas, &colors, &interactables, Pass::Picker).unwrap_throw();
    renderer.draw_scene(&world_transforms, &img_areas, &colors, &interactables, Pass::Forward).unwrap_throw();
}

