use crate::camera::Camera;
use crate::model::GameObject;

pub trait Scene {
    fn update(&mut self);
    fn camera(&self) -> &Camera;
    fn game_objects(&self) -> &Vec<GameObject>;
}
