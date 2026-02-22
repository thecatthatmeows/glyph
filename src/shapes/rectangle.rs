use std::{thread::sleep, time::Duration};

use crate::{
    shapes::{Orientation, Shape, triangle::Triangle},
    types::vec2::Vec2,
};
use crossterm::style::Color;

pub struct Rectangle {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    pub orientation: Orientation,
    pub color: Color,
    pub z_index: i32,
    triangles: [Triangle; 2],
    pub children: Vec<Box<dyn Shape>>,
}

impl Rectangle {
    pub fn new(pos: Vec2<f32>, mut size: Vec2<f32>, color: Color) -> Self {
        let orientation = Orientation::Left;
        size.swap();
        let upper = Triangle::new(pos, orientation, size, color);
        let bottom = Triangle::new(pos, orientation.opposite(), size, color);
        // upper.vertices.bottom_right += size;
        // bottom.vertices.bottom_right += size;

        Self {
            pos,
            size,
            orientation,
            color,
            z_index: 0,
            triangles: [upper, bottom],
            children: vec![],
        }
    }

    pub fn push(&mut self, child: Box<dyn Shape>) {
        self.children.push(child);
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        // Recreate the triangles rather than attempting to clone Triangle directly
        // (Triangle contains a StdoutLock which is not Clone). Children are cloned
        // via the Box<dyn Shape> Clone implementation (requires each concrete
        // shape to implement `box_clone`).
        let upper = Triangle::new(self.pos, self.orientation, self.size, self.color);
        let bottom = Triangle::new(self.pos, self.orientation.opposite(), self.size, self.color);

        Self {
            pos: self.pos,
            size: self.size,
            orientation: self.orientation,
            color: self.color,
            z_index: self.z_index,
            triangles: [upper, bottom],
            children: self.children.iter().map(|c| c.clone()).collect(),
        }
    }
}

impl Shape for Rectangle {
    fn update(&mut self) {
        let upper = Triangle::new(self.pos, self.orientation, self.size, self.color);
        let bottom = Triangle::new(self.pos, self.orientation.opposite(), self.size, self.color);

        self.triangles = [upper, bottom];
        self.triangles.sort_by_key(|triangle| triangle.z_index);

        for triangle in &mut self.triangles {
            triangle.update();
        }

        for child in &mut self.children {
            child.update();
        }
    }

    fn draw(&mut self) {
        for triangle in &mut self.triangles {
            triangle.draw();
        }

        // is this considered recursive or..??
        for child in &mut self.children {
            child.draw();
        }
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}
