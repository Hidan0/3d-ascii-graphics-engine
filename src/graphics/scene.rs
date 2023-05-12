use crate::camera::Camera;

pub trait Scene {
    fn camera() -> Option<Camera> {
        todo!()
    }
}
