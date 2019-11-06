/// Tools for parsing and hacking layouts created by the Stretch flexbox crate
///
///
#[allow(unused_imports)]
use stretch::{
    geometry::*,
    node::{Node, Stretch},
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

pub struct LayoutBuilder {
    root: Node,
    stretch: Stretch,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        let mut stretch = Stretch::new();
        let root = stretch.new_node(
            Style::default(),
            vec![]
        ).unwrap();
        LayoutBuilder { root, stretch }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.stretch.set_style(self.root, style);
        self
    }

    pub fn add_row(&mut self, height: f32) {

    }
}


/// A utility for calculating absolute positions from a deep copy of a Strech layout
pub struct LayoutSolver {
    // stretch: Stretch
}

impl LayoutSolver {

    pub fn new() -> Self {
        // let stretch = Stretch::new();
        LayoutSolver {}
    }

    /// A helper method to convert a Layout constructed by Stretch to absolute xy coords.
    /// Additional features may be added in the future
    pub fn absolute_layout(&mut self, node: Node, stretch: &mut Stretch) -> NodeLayout {
        // if let Ok(layout) =
        stretch.compute_layout(node, Size::undefined());

        let layout = stretch.layout(node).unwrap();
        let mut result = NodeLayout {
            id: 0,
            size: layout.size.clone(),
            location: layout.location.clone(),
            children: Vec::new(),
        };
        self.abs_copy_layout(node, &mut result, stretch);
        result
    }

    /// A recursive function for performing a deep copy of a Stretch Layout and changing coordinates from
    /// relative to absolute.
    fn abs_copy_layout(&mut self, node: Node, result: &mut NodeLayout, stretch: &mut Stretch) {
        let children = stretch.children(node).unwrap();
        for (i, child) in children.iter().enumerate() {
            let layout = stretch.layout(*child).unwrap();
            let pos = Point { x: result.location.x + layout.location.x, y: result.location.y + layout.location.y };
            let mut item = NodeLayout {
                id: i as u32,
                size: layout.size.clone(),
                location: pos,
                children: Vec::new(),
            };
            if stretch.child_count(*child).unwrap() > 0 {
                self.abs_copy_layout(*child, &mut item, stretch);
            }
            result.children.push(item);
        }
    }

    /// Unused
    pub fn absolute_position(&self, layout: &Layout, path: Vec<usize>) -> Point<f32> {
        let position = Point { x: 0.0, y: 0.0 };
        // let mut current = layout.clone();

        // for i in path {
        //     if i < current.children.len() {
        //         current = current.children[i].clone();
        //         let location = current.location;
        //         position = Point { x: position.x + location.x, y: position.y + location.y };
        //     } else {
        //         return position;
        //     }
        // }
        position
    }

}

// impl Iterator for Layout {
//     type Item = Layout;

//     fn next(&mut self) -> Option<Layout> {

//     }
// }