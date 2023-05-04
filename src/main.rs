use self::app::App;

mod app;
mod camera;
mod model;
mod render;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 32;

fn main() {
    let app = App::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();
}
