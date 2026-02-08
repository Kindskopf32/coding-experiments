extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use std::path::Path;

fn main() {
    // 1. Create a window
    let mut window = Window::new("Utah Teapot - Rust Renderer");

    // 2. Load the teapot model
    // Ensure "teapot.obj" is in the root directory of your project (next to Cargo.toml)
    // The second and third arguments are for the MTL path and scale, which we handle manually here.
    let teapot_path = Path::new("teapot.obj");
    
    // Check if file exists to prevent a confusing panic
    if !teapot_path.exists() {
        eprintln!("Error: 'teapot.obj' not found in project root.");
        std::process::exit(1);
    }

    // add_obj loads the mesh into the scene. 
    // We pass "." as the MTL path (dummy) and a scale vector of 1.0.
    let mut teapot = window.add_obj(
        teapot_path, 
        Path::new("."), 
        na::Vector3::new(1.0, 1.0, 1.0)
    );

    // 3. Configure the Teapot Appearance
    // Set color to a nice ceramic white or the classic reddish clay
    teapot.set_color(1.0, 0.5, 0.31); 
    
    // Scale it if the OBJ file is too small/big
    teapot.set_local_scale(0.5, 0.5, 0.5);

    // 4. Setup Lighting
    // "StickToCamera" ensures the light moves with you so you can see the object
    window.set_light(Light::StickToCamera);

    // 5. The Render Loop
    while window.render() {
        // Rotate the teapot slightly every frame
        let rot = na::UnitQuaternion::from_axis_angle(
            &na::Vector3::y_axis(), 
            0.01
        );
        teapot.prepend_to_local_rotation(&rot);
    }
}