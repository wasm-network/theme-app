/// Tools for parsing and hacking layouts created by the Stretch flexbox crate
///
use quicksilver::{
    geom::{Rectangle},
    graphics::{Background::Col, Color},
    lifecycle::Window
};
#[allow(unused_imports)]
use stretch::{
    geometry::*,
    node::Node,
    result::Layout,
    style::*
};


/// A wrapper for containing the resulting layout including all children
#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub id: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<NodeLayout>,
}

impl NodeLayout {}

/// A utility for calculating absolute positions from a deep copy of a Strech layout
pub struct LayoutSolver {}

impl LayoutSolver {

    /// A helper method to convert a Layout constructed by Stretch to absolute xy coords.
    /// Additional features may be added in the future
    pub fn absolute_layout(&self, layout: &Layout) -> NodeLayout {
        let mut result = NodeLayout {
            id: 0,
            size: layout.size.clone(),
            location: layout.location.clone(),
            children: Vec::new(),
        };
        self.abs_copy_layout(layout, &mut result);
        result
    }

    /// A recursive function for performing a deep copy of a Stretch Layout and changing coordinates from
    /// relative to absolute.
    fn abs_copy_layout(&self, layout: &Layout, result: &mut NodeLayout) {
        for (i, child) in layout.children.iter().enumerate() {

            let pos = Point { x: result.location.x + child.location.x, y: result.location.y + child.location.y };
            let mut item = NodeLayout {
                id: i as u32,
                size: child.size.clone(),
                location: pos,
                children: Vec::new(),
            };
            if child.children.len() > 0 {
                self.abs_copy_layout(&child, &mut item);
            }
            result.children.push(item);
        }
    }

    /// Unused
    pub fn absolute_position(&self, layout: &Layout, path: Vec<usize>) -> Point<f32> {
        let mut position = Point { x: 0.0, y: 0.0 };
        let mut current = layout.clone();

        for i in path {
            if i < current.children.len() {
                current = current.children[i].clone();
                let location = current.location;
                position = Point { x: position.x + location.x, y: position.y + location.y };
            } else {
                return position;
            }
        }
        position
    }

}

// impl Iterator for Layout {
//     type Item = Layout;

//     fn next(&mut self) -> Option<Layout> {

//     }
// }