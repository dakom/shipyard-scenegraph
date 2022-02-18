//These aren't necessary for the scenegraph itself
//But helpful for common cases so it is satisfied
//In the local libs and exported in prelude

pub trait F32Compat {
    fn write_to_vf32(&self, target: &mut [f32]);
}
