extern crate kiss3d;
// CHANGE THIS LINE:
// OLD: extern crate nalgebra as na;
// NEW: Use the nalgebra built into kiss3d
use kiss3d::nalgebra as na; 

use kiss3d::light::Light;
use kiss3d::window::Window;
use std::path::Path;

fn main() {
    let mut window = Window::new("Utah Teapot - Rust Renderer");

    let teapot_path = Path::new("teapot.obj");
    
    if !teapot_path.exists() {
        eprintln!("Error: 'teapot.obj' not found in project root.");
        std::process::exit(1);
    }

    // Now this Vector3 matches the one kiss3d expects because they come from the same place
    let mut teapot = window.add_obj(
        teapot_path, 
        Path::new("."), 
        na::Vector3::new(1.0, 1.0, 1.0)
    );

    teapot.set_color(1.0, 0.5, 0.31); 
    teapot.set_local_scale(0.5, 0.5, 0.5);

    window.set_light(Light::StickToCamera);

    while window.render() {
        // This UnitQuaternion now also matches perfectly
        let rot = na::UnitQuaternion::from_axis_angle(
            &na::Vector3::y_axis(), 
            0.01
        );
        teapot.prepend_to_local_rotation(&rot);
    }
}