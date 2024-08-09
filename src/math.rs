use std::f32::consts::PI;

use bevy::prelude::*;

/// Given a point and a line segment, get the point on the line segment
/// that is closest to the provided point
pub fn closest_point_on_segment(pos: Vec2, line: [Vec2; 2]) -> Vec2 {
    let l2 = (line[1].x - line[0].x).powi(2) + (line[1].y - line[0].y).powi(2);
    let t = ((pos.x - line[0].x) * (line[1].x - line[0].x)
        + (pos.y - line[0].y) * (line[1].y - line[0].y))
        / l2;
    let t = t.clamp(0.0, 1.0);
    Vec2 {
        x: line[0].x + t * (line[1].x - line[0].x),
        y: line[0].y + t * (line[1].y - line[0].y),
    }
}

/// Calculates the signed distance from a point to a line segment. Also returns the closest point
/// Returns a POSITIVE number if the pos is "OUTSIDE" the line according to our clockwise convention
/// Returns a NEGATIVE number if the pos is "INSIDE" the line according to our clockwise convention.
pub fn signed_distance_to_segment(pos: Vec2, line: [Vec2; 2]) -> (f32, Vec2) {
    let cp = closest_point_on_segment(pos, line);
    let line_diff = line[1] - line[0];
    let normal_pointing = Vec2 {
        x: line_diff.y,
        y: -line_diff.x,
    };
    let diff = line[0] - pos;
    let dotprod = diff.dot(normal_pointing);
    (dotprod.signum() * pos.distance(cp), cp)
}

pub trait ToLines {
    fn to_lines(&self) -> Vec<[Vec2; 2]>;
}

impl ToLines for Vec<Vec2> {
    fn to_lines(&self) -> Vec<[Vec2; 2]> {
        let mut result = vec![[Vec2::ZERO, Vec2::ZERO]; self.len()];
        for ix in 0..self.len() {
            result[ix] = [self[ix], self[(ix + 1).rem_euclid(self.len())]];
        }
        result
    }
}

/// I am small-brain, this is the rotation api I want
pub trait MyRotations {
    fn my_rotate(self, angle: f32) -> Self;
}

impl MyRotations for Vec2 {
    fn my_rotate(self, angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Vec2::new(self.x * c - self.y * s, self.x * s + self.y * c)
    }
}

/// Bevy API is not quite what I want
pub trait MyTranNAngleGetter {
    fn tran_n_angle(&self) -> (Vec2, f32);
}

impl MyTranNAngleGetter for Transform {
    fn tran_n_angle(&self) -> (Vec2, f32) {
        (
            self.translation.truncate(),
            self.rotation.to_euler(EulerRot::XYZ).2,
        )
    }
}

impl MyTranNAngleGetter for GlobalTransform {
    fn tran_n_angle(&self) -> (Vec2, f32) {
        let (_, quat, tran) = self.to_scale_rotation_translation();
        (tran.truncate(), quat.to_euler(EulerRot::XYZ).2)
    }
}

/// Just trying to make it dead-easy to do anything with rotation, that's all
pub trait MyAngleSetter {
    fn set_angle(&mut self, angle: f32);
}

impl MyAngleSetter for Transform {
    fn set_angle(&mut self, angle: f32) {
        self.rotation = Quat::from_rotation_z(angle)
    }
}

/// Helpful function to generate the points of a rectangle, centered at zero, with our clockwise convention
pub fn simple_rect(width: f32, height: f32) -> Vec<Vec2> {
    vec![
        Vec2::new(-width / 2.0, -height / 2.0),
        Vec2::new(-width / 2.0, height / 2.0),
        Vec2::new(width / 2.0, height / 2.0),
        Vec2::new(width / 2.0, -height / 2.0),
    ]
}

/// Given a list of points, return points that retain the same shape, but produce an outline
pub fn outline_points(points: &[Vec2], width: f32) -> Vec<Vec2> {
    let mut new_points = vec![];
    for (ix, point) in points.iter().enumerate() {
        let point_before = points[if ix == 0 { points.len() - 1 } else { ix - 1 }];
        let point_after = points[if ix == points.len() - 1 { 0 } else { ix + 1 }];
        let slope_before = (*point - point_before).normalize_or_zero();
        let slope_after = (point_after - *point).normalize_or_zero();
        let tang = (slope_before + slope_after).normalize_or_zero();
        let perp = Vec2::new(-tang.y, tang.x);
        new_points.push(*point + perp * width);
    }
    new_points
}

/// Returns the smallest UVec 2 such that an aabb of that size could cover points
pub fn uvec2_bound(points: &[Vec2]) -> UVec2 {
    let mut mins = Vec2::new(f32::MAX, f32::MAX);
    let mut maxs = Vec2::new(f32::MIN, f32::MIN);
    for vec in points {
        mins = mins.min(*vec);
        maxs = maxs.max(*vec);
    }
    UVec2 {
        x: (maxs.x - mins.x).ceil() as u32,
        y: (maxs.y - mins.y).ceil() as u32,
    }
}

/// God i write this so much
pub fn spat_tran(x: f32, y: f32, z: f32) -> SpatialBundle {
    SpatialBundle::from_transform(Transform::from_translation(Vec3::new(x, y, z)))
}

pub fn regular_polygon(num_sides: u32, mut angle: f32, radius: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec![];
    for _ in 0..num_sides {
        let x = angle.to_radians().cos() * radius;
        let y = angle.to_radians().sin() * radius;
        points.push(Vec2 { x, y });
        angle -= 360.0 / (num_sides as f32);
    }
    points
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Spleen {
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInQuartic,
    EaseOutQuartic,
    EaseInOutQuartic,
    EaseInQuintic,
    EaseOutQuintic,
    EaseInOutQuintic,
}

impl Spleen {
    pub fn interp(&self, x: f32) -> f32 {
        match *self {
            Self::EaseInCubic => ease_in_cubic(x),
            Self::EaseOutCubic => ease_out_cubic(x),
            Self::EaseInOutCubic => ease_in_out_cubic(x),
            Self::EaseInQuad => ease_in_quad(x),
            Self::EaseOutQuad => ease_out_quad(x),
            Self::EaseInOutQuad => ease_in_out_quad(x),
            Self::EaseInQuartic => ease_in_quartic(x),
            Self::EaseOutQuartic => ease_out_quartic(x),
            Self::EaseInOutQuartic => ease_in_out_quartic(x),
            Self::EaseInQuintic => ease_in_quintic(x),
            Self::EaseOutQuintic => ease_out_quintic(x),
            Self::EaseInOutQuintic => ease_in_out_quintic(x),
        }
    }

    /// Given progress x, interps between min and max using this spleen
    pub fn bound_interp(&self, x: f32, min: f32, max: f32) -> f32 {
        min + self.interp(x) * (max - min)
    }
}

fn ease_in_cubic(x: f32) -> f32 {
    x * x * x
}

fn ease_out_cubic(x: f32) -> f32 {
    1.0 - ease_in_cubic(1.0 - x)
}

fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - ((0.0 - 2.0) * x + 2.0).powf(3.0) / 2.0
    }
}

fn ease_in_quad(x: f32) -> f32 {
    x * x
}

fn ease_out_quad(x: f32) -> f32 {
    1.0 - ease_in_quad(1.0 - x)
}

fn ease_in_out_quad(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - ((0.0 - 2.0) * x + 2.0).powf(2.0) / 2.0
    }
}

fn ease_in_quartic(x: f32) -> f32 {
    x * x * x * x
}

fn ease_out_quartic(x: f32) -> f32 {
    1.0 - ease_in_quartic(1.0 - x)
}

fn ease_in_out_quartic(x: f32) -> f32 {
    if x < 0.5 {
        8.0 * x.powi(4)
    } else {
        1.0 - ((-2.0 * x + 2.0).powi(4)) / 2.0
    }
}

fn ease_in_quintic(x: f32) -> f32 {
    x * x * x * x * x
}

fn ease_out_quintic(x: f32) -> f32 {
    1.0 - ease_in_quintic(1.0 - x)
}

fn ease_in_out_quintic(x: f32) -> f32 {
    if x < 0.5 {
        16.0 * x.powi(5)
    } else {
        1.0 - ((-2.0 * x + 2.0).powi(5)) / 2.0
    }
}

/// Returns the shortest direction to rotate `angle1` to get closer to `angle2`.
/// If the result is positive, rotate clockwise; if negative, rotate counterclockwise.
pub fn shortest_rotation(angle1: f32, angle2: f32) -> f32 {
    // Normalize the angles to be between 0 and 2π
    let angle1 = angle1.rem_euclid(2.0 * PI);
    let angle2 = angle2.rem_euclid(2.0 * PI);

    // Calculate the difference
    let mut difference = angle2 - angle1;

    // Normalize the difference to be between -π and π
    if difference > PI {
        difference -= 2.0 * PI;
    } else if difference < -PI {
        difference += 2.0 * PI;
    }

    difference
}
