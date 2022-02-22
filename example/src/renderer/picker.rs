use awsm_web::webgl::{
    WebGl1Renderer,
    Id,
    TextureTarget,
    RenderBufferFormat,
    FrameBufferTarget,
    FrameBufferAttachment,
    FrameBufferTextureTarget,
    ReadPixelFormat,
    ReadPixelDataType,
    TextureMinFilter,
    TextureMagFilter,
    WebGlTextureSource,
    SimpleTextureOptions,
    PixelFormat
};
use awsm_web::errors::Error;
use wasm_bindgen::prelude::*;
use shipyard::*;

#[derive(Component)]
pub struct ScenePicker {
    pub width: u32,
    pub height: u32,
    pub framebuffer_id: Id,
    pub depthbuffer_id: Id,
    pub texture_id: Id,
}

impl ScenePicker {
    pub fn destroy(self, renderer:&mut WebGl1Renderer) -> Result<(), Error> {
        renderer.delete_framebuffer(self.framebuffer_id)?;
        renderer.delete_renderbuffer(self.depthbuffer_id)?;
        renderer.delete_texture(self.texture_id)?;
        Ok(())
    }
}
impl ScenePicker {
    pub fn new(renderer:&mut WebGl1Renderer, width: u32, height: u32) -> Result<Self, Error> {

        //Framebuffer
        let framebuffer_id = renderer.create_framebuffer()?;

        //Depth
        let depthbuffer_id = renderer.create_renderbuffer()?;
        renderer.assign_renderbuffer_storage(depthbuffer_id, RenderBufferFormat::DepthComponent16, width, height)?;
        renderer.assign_framebuffer_renderbuffer(framebuffer_id, depthbuffer_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Depth)?;

        //Texture
        let texture_id = renderer.create_texture()?;
        renderer.assign_simple_texture(
            texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                flip_y: Some(false),
                filter_min: Some(TextureMinFilter::Linear),
                filter_mag: Some(TextureMagFilter::Linear),
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        ).unwrap_throw();
        
        renderer.assign_framebuffer_texture_2d(framebuffer_id, texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

        //make sure we're all good
        renderer.check_framebuffer_status(FrameBufferTarget::FrameBuffer)?;

        //unbind everything
        renderer.release_renderbuffer();
        renderer.release_framebuffer(FrameBufferTarget::FrameBuffer);
        renderer.release_texture_target(TextureTarget::Texture2d);


        Ok(Self{
            width,
            height,
            framebuffer_id,
            depthbuffer_id,
            texture_id,
        })
    }

    pub fn start(&self, renderer:&mut WebGl1Renderer) -> Result<(), awsm_web::errors::Error> {
        renderer.bind_framebuffer(self.framebuffer_id, FrameBufferTarget::FrameBuffer)?;
        Ok(())
    }
    
    pub fn finish(&self, renderer:&mut WebGl1Renderer) -> Result<(), awsm_web::errors::Error> {
        renderer.release_framebuffer(FrameBufferTarget::FrameBuffer);
        Ok(())
    }


    //x and y should already be translated to bottom-left
    pub fn get_color(&self, renderer:&mut WebGl1Renderer, x: u32, y: u32) -> Result<[u8;4], awsm_web::errors::Error> {

        let mut data:[u8;4] = [0;4];

        //bind the read buffer which contains the hidden texture
        renderer.bind_framebuffer(self.framebuffer_id, FrameBufferTarget::FrameBuffer)?;

        renderer.read_pixels_u8(x, y, 1, 1, ReadPixelFormat::Rgba, ReadPixelDataType::UnsignedByte, &mut data)?;
        ////release
        renderer.release_framebuffer(FrameBufferTarget::FrameBuffer);

        Ok(data)
    }

}

