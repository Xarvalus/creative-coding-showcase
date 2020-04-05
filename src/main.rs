use nannou::prelude::*;

use nannou::color::IntoLinSrgba;
use rand::Rng;
use std::collections::HashSet;
use rand::seq::SliceRandom;

fn main() {
    nannou::app(model)
//        .update(update)
//        .simple_window(view)
        .run();
}

struct Model {
    window_id: window::Id,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(1024, 1024)
        .view(view).build().unwrap();

    Model { window_id }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

// Based on: https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_textured_mesh.rs
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DIMGRAY); // BLACK
    let window = app.window(model.window_id).unwrap();
    let win_rect = window.rect();
    let draw = app.draw();

    // TODO: webassembly is there? https://www.figma.com/


    let centre = pt3(0.0, 1.0, 0.0);
    let size = vec3(2.0, 1.0, 0.5);
    let rotate = vec3(0.0, 60.0, 0.0);
    generate_cuboid(app, model, win_rect, &draw, centre, size, rotate);

    // TODO: rework to struct / method?
    let centre = pt3(0.0, 0.0, 0.0);
    let size = vec3(1.0, 0.5, 0.25);
    let rotate = vec3(60.0, 0.0, 0.0);
    generate_cuboid(app, model, win_rect, &draw, centre, size, rotate);

    let centre = pt3(0.0, -1.0, 0.0);
    let size = vec3(2.0, 1.0, 0.5);
    let rotate = vec3(0.0, -30.0, 0.0);
    generate_cuboid(app, model, win_rect, &draw, centre, size, rotate);

    // Draw to the frame!
    draw.to_frame(app, &frame).unwrap();
}

fn generate_cuboid(app: &App, model: &Model, win_rect: Rect<f32>, draw: &Draw,
                   centre: Point3<f32>, size: Vector3<f32>, rotate: Vector3<f32>) {
    // Generate the triangulated points for a cuboid to use for out mesh.
    let cuboid = geom::Cuboid::from_xyz_whd(centre, size);

    let colors = vec![RED, GREEN, BLUE];

    let points = cuboid
        .triangles_iter()
        .flat_map(geom::Tri::vertices)
        .map(|point| {
            (point, colors.choose(&mut rand::thread_rng()).unwrap())
        });

    // Scale the points up to half the window size.
    let cube_side = win_rect.w().min(win_rect.h()) * 0.5;
    draw.scale(cube_side)
        .mesh()
        .points_colored(points)
        .x_degrees(app.time * 0.75 * rotate.x)
        .y_degrees(app.time * 0.75 * rotate.y)
        .z_degrees(rotate.z);
}
