use std::f32::consts::FRAC_PI_4;

use nalgebra_glm::Vec3;

use crate::camera::Camera;
use crate::graphics::pipeline::Pipeline;
use crate::graphics::scene::Scene;
use crate::model::{GameObject, Model};

pub struct App {
    pipeline: Pipeline,
}

impl App {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            pipeline: Pipeline::new(screen_width, screen_height),
        }
    }

    pub fn run(&self) {
        let mut scene = SimpleScene::new(self.pipeline.aspect_ratio);

        for _ in 0..500 {
            self.pipeline.draw(&scene);
            scene.update();
        }
    }
}

pub struct SimpleScene {
    camera: Camera,
    game_objects: Vec<GameObject>,
}

impl SimpleScene {
    fn new(aspect_ratio: f32) -> Self {
        let camera = Camera::new_perspective_camera(FRAC_PI_4, aspect_ratio, 0.1, 10.);

        let mut obj = GameObject::new(Model::square());

        obj.transform.scale = Vec3::new(4., 4., 1.);

        Self {
            camera,
            game_objects: vec![obj],
        }
    }
}

impl Scene for SimpleScene {
    fn update(&mut self) {
        self.game_objects[0].transform.rotation.z += 0.8_f32.to_radians();
    }

    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn game_objects(&self) -> &Vec<GameObject> {
        &self.game_objects
    }
}
