use super::{Frame, camera::Projection, frame::DrawCommand};

pub(crate) struct Geometry<'a> {
    pub(crate) command: &'a DrawCommand,
    pub(crate) left: f32,
    pub(crate) right: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
    pub(crate) u_left: f32,
    pub(crate) u_right: f32,
    pub(crate) v_top: f32,
    pub(crate) v_bottom: f32,
}

pub(crate) fn project(frame: &Frame) -> impl Iterator<Item = Geometry<'_>> {
    frame.draws.iter().filter_map(move |draw| {
        project_command(&draw.command, draw.projection, frame.window_size())
    })
}

fn project_command(
    command: &DrawCommand,
    projection: Projection,
    size: (f64, f64),
) -> Option<Geometry<'_>> {
    let (x, y, width, height) = match command {
        DrawCommand::Rectangle(rectangle) => {
            (rectangle.x, rectangle.y, rectangle.width, rectangle.height)
        }
        DrawCommand::Sprite { image, x, y } => {
            (*x, *y, image.data.width as f32, image.data.height as f32)
        }
    };
    let origin_x = (f64::from(x) - projection.origin.0) * projection.scale + projection.offset.0;
    let origin_y = (f64::from(y) - projection.origin.1) * projection.scale + projection.offset.1;
    let projected_width = f64::from(width) * projection.scale;
    let projected_height = f64::from(height) * projection.scale;
    if ![
        origin_x,
        origin_y,
        projected_width,
        projected_height,
        origin_x + projected_width,
        origin_y + projected_height,
    ]
    .into_iter()
    .all(f64::is_finite)
        || projected_width <= 0.0
        || projected_height <= 0.0
    {
        return None;
    }
    let clip = projection.clip;
    let left = origin_x.clamp(clip.left, clip.right);
    let top = origin_y.clamp(clip.top, clip.bottom);
    let right = (origin_x + projected_width).clamp(clip.left, clip.right);
    let bottom = (origin_y + projected_height).clamp(clip.top, clip.bottom);
    if left >= right || top >= bottom {
        return None;
    }
    // Crop UVs with the geometry to avoid stretching sprites.
    let u_left = ((left - origin_x) / projected_width) as f32;
    let u_right = ((right - origin_x) / projected_width) as f32;
    let v_top = ((top - origin_y) / projected_height) as f32;
    let v_bottom = ((bottom - origin_y) / projected_height) as f32;
    let left = (2.0 * (left / size.0) - 1.0) as f32;
    let right = (2.0 * (right / size.0) - 1.0) as f32;
    let top = (1.0 - 2.0 * (top / size.1)) as f32;
    let bottom = (1.0 - 2.0 * (bottom / size.1)) as f32;
    Some(Geometry {
        command,
        left,
        right,
        top,
        bottom,
        u_left,
        u_right,
        v_top,
        v_bottom,
    })
}
