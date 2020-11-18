// mod point;
// use self::point::Point;
use super::point::Point2D;
use super::point::Point3D;

#[derive(Debug)]
pub struct Plane3D {
    pub points: [Point3D; 4],
}

impl Plane3D {
    pub fn new(bl: Point3D, tl: Point3D, tr: Point3D, br: Point3D) -> Self {
        Self {
            points: [bl, tl, tr, br],
        }
    }
    pub fn points_as_array(&self) -> Vec<f32> {
        let mut returnable: Vec<f32> = vec![];

        self.points.iter().for_each(|point| {
            point.as_array().iter().for_each(|coord| {
                returnable.push(coord.clone());
            })
        });
        returnable
    }
}
#[derive(Debug)]
pub struct Plane2D {
    pub points: [Point2D; 4],
}

impl Plane2D {
    pub fn new(bl: Point2D, tl: Point2D, tr: Point2D, br: Point2D) -> Self {
        Self {
            points: [bl, tl, tr, br],
        }
    }
    pub fn points_as_array(&self) -> Vec<f32> {
        let mut returnable: Vec<f32> = vec![];

        self.points.iter().for_each(|point| {
            point.as_array().iter().for_each(|coord| {
                returnable.push(coord.clone());
            })
        });
        returnable
    }
}
