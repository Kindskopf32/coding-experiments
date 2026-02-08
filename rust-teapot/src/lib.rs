use kiss3d::light::Light;
use kiss3d::window::{Window, State};
use kiss3d::nalgebra as na;
use wasm_bindgen::prelude::*;

// Embed the teapot.obj file at compile time
const TEAPOT_OBJ_DATA: &str = include_str!("../teapot.obj");

// State struct to hold our teapot and handle the render loop
struct TeapotState {
    teapot: kiss3d::scene::SceneNode,
}

impl State for TeapotState {
    fn step(&mut self, _window: &mut Window) {
        // Rotate the teapot
        let rot = na::UnitQuaternion::from_axis_angle(
            &na::Vector3::y_axis(),
            0.01
        );
        self.teapot.prepend_to_local_rotation(&rot);
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    run();
}

fn run() {
    // For WASM, we need to handle the embedded data
    // Write to the virtual filesystem that WASM can access
    let obj_path = std::path::Path::new("/teapot.obj");
    let mtl_path = std::path::Path::new("/");
    
    // Write embedded OBJ data to virtual filesystem
    #[cfg(target_arch = "wasm32")]
    {
        use std::io::Write;
        // In WASM environment, create the file in the virtual filesystem
        let mut file = std::fs::File::create(obj_path).expect("Failed to create OBJ file in WASM FS");
        file.write_all(TEAPOT_OBJ_DATA.as_bytes()).expect("Failed to write OBJ data");
    }
    
    // Create window - in WASM, this sets up a canvas automatically
    let mut window = Window::new("Utah Teapot - Rust WASM Renderer");
    
    // Load the teapot - works for both native and WASM
    let mut teapot = window.add_obj(obj_path, mtl_path, na::Vector3::new(1.0, 1.0, 1.0));
    
    teapot.set_color(1.0, 0.5, 0.31);
    teapot.set_local_scale(0.5, 0.5, 0.5);
    
    window.set_light(Light::StickToCamera);
    
    // Create state and run the render loop
    let state = TeapotState { teapot };
    window.render_loop(state);
}

// Keep main for native builds
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    start();
}
