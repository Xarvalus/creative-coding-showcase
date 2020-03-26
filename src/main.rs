use nannou::prelude::*;

fn main() {
    nannou::app(model)
//        .update(update)
//        .simple_window(view)
        .run();
}

struct Model {
    window_id: window::Id,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(1024, 1024)
        .view(view).build().unwrap();

    // Load the image from disk and upload it to a GPU texture.
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("nature_1.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model { window_id, texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

// TODO: webassembly is there? https://www.figma.com/

// Based on: https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_textured_mesh.rs
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DIMGRAY); // BLACK
    let window = app.window(model.window_id).unwrap();
    let win_rect = window.rect();
    let draw = app.draw();

    let centre = pt3(0.0, 1.0, 0.0);
    let size = vec3(2.0, 1.0, 0.0);
    generate_cuboid(app, model, win_rect, &draw, centre, size);

    // TODO: rework to struct / method?
    let centre = pt3(0.0, 0.0, 0.0);
    let size = vec3(1.0, 0.5, 1.0);
    generate_cuboid(app, model, win_rect, &draw, centre, size);

    let centre = pt3(0.0, -1.0, 0.0);
    let size = vec3(2.0, 1.0, 0.0);
    generate_cuboid(app, model, win_rect, &draw, centre, size);

    // Draw to the frame!
    draw.to_frame(app, &frame).unwrap();
}

fn generate_cuboid(app: &App, model: &Model, win_rect: Rect<f32>, draw: &Draw,
                   centre: Point3<f32>, size: Vector3<f32>) {
    // Generate the triangulated points for a cuboid to use for out mesh.
    let cuboid = geom::Cuboid::from_xyz_whd(centre, size);

    let points = cuboid
        .triangles_iter()
        .flat_map(geom::Tri::vertices)
        .map(|point| {
            // Tex coords should be in range (0.0, 0.0) to (1.0, 1.0);
            // This will have the logo show on the front and back faces.
            let tex_coords = [point.x + 0.5, 1.0 - (point.y + 0.5)];
            (point, tex_coords)
        });

    // Scale the points up to half the window size.
    let cube_side = win_rect.w().min(win_rect.h()) * 0.5;
    draw.scale(cube_side)
        .mesh()
        .points_textured(&model.texture, points)
        .z_radians(app.time * 0.33)
        .x_radians(app.time * 0.166 + -app.mouse.y / 100.0)
        .y_radians(app.time * 0.25 + app.mouse.x / 100.0);
}
