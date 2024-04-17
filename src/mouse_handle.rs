use autopilot::mouse::*;
use std::thread::sleep;
use autopilot::geometry::Point;

pub fn move_mouse(default_point: (f64, f64)) {

    let point = Point::new(
        default_point.0,
        default_point.1,
    );

    let _ = move_to(point);
}

pub fn check_location(default_position: (f64, f64), error: f64) -> bool {
    let (mouse_x, mouse_y) = default_position;
    let mouse_error = error;
    let mut l = location();
    if (l.x - mouse_x).abs() > mouse_error || (l.y - mouse_y).abs() > mouse_error{
        sleep(std::time::Duration::new(3, 0));
        l = location();
        if (l.x - mouse_x).abs() > mouse_error || (l.y - mouse_y).abs() > mouse_error {
            return false;
        }
    }
    return true;

}

