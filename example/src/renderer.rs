use crate::geometry::*;
use crate::components::Color;
use shipyard::*;
use nalgebra::Matrix4;

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
};

#[derive(Component)]
pub struct SceneRenderer {
    pub renderer: WebGl1Renderer,
    ids: SceneIds,
}

struct SceneIds {
    program_id: Id,
}
impl SceneRenderer {
    pub fn new (mut renderer:WebGl1Renderer, vertex:&str, fragment:&str) -> Result<Self, awsm_web::errors::Error> {
        let ids = {
            //This demo is specifically using webgl1, which needs to register the extension
            //Everything else is the same API as webgl2 :)
            renderer.register_extension_instanced_arrays()?;
            
            let shaders = vec![
                    renderer.compile_shader(vertex, ShaderType::Vertex).unwrap(),
                    renderer.compile_shader(fragment, ShaderType::Fragment).unwrap(),
            ];
            let program_id = renderer.compile_program(&shaders)?;

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

            SceneIds {program_id}
        };

        renderer.gl.clear_color(0.3, 0.3, 0.3, 1.0);

        Ok(Self { renderer, ids} )
    }

    pub fn clear(&mut self) {
        self.renderer.clear(&[
            BufferMask::ColorBufferBit,
            BufferMask::DepthBufferBit,
        ]);
    }
    pub fn pre_render(&mut self, stage_area:&Area) -> Result<(), awsm_web::errors::Error> {
        self.clear();

        let renderer = &mut self.renderer;
        let SceneIds {program_id} = self.ids;

        renderer.toggle(GlToggle::Blend, true);
        renderer.toggle(GlToggle::DepthTest, true);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

        //will already be activated but internally that's a noop if true
        renderer.activate_program(program_id)?;

        //Build our matrices (must cast to f32)
        let camera_mat = Matrix4::new_orthographic( 0.0, stage_area.width as f32, 0.0, stage_area.height as f32, -100.0, 100.0);

        //Upload them to the GPU
        renderer.upload_uniform_mat_4_name("u_camera", &camera_mat.as_slice())?;

        Ok(())
    }

    pub fn draw_square(&mut self, model_mat:&[f32], img_area:&Area, color:&Color) -> Result<(), awsm_web::errors::Error> {

        let renderer = &mut self.renderer;
        /*
        let model_mat = Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, 0.0));
        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(img_area.width as f32, img_area.height as f32, 0.0));

        let complete_model = model_mat * scaling_mat;
        */
        renderer.upload_uniform_mat_4_name("u_model", &model_mat)?;
        renderer.upload_uniform_fvals_2_name("u_size", img_area.get_tuple())?;
        //renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;
        renderer.upload_uniform_fvals_4_name("u_color", color.get_tuple())?;

        renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);

        Ok(())
    }

}
