/// navbar
// use crate::controllers::*;
use crate::utils::*;

use std::cell::RefCell;
use std::rc::{Rc};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::Window
};
use stretch::{
    geometry::*,
    node::Node,
    result::Layout,
    style::*
};

use tweek::{
    // core::{AppState},
    gui::{Button, Label, Scene, Displayable, Theme},
};

/// This is a simple nav bar that supports a left button, right button and title label in the middle.
/// It does not yet support multiple buttons in the left and right side. And nor does it support
/// toolbar-style nav bars which have collections of buttons (like in Material Design)
pub struct NavBar {
    pub frame: Rectangle,
    pub scene: Scene,
    pub color: Option<Color>,
    title_ptr: Option<usize>, // todo: allow for stacked title
    left_btns: Vec<usize>,
    right_btns: Vec<usize>,
    layout: Option<Layout>,
}

impl NavBar {
    pub fn new(frame: &Rectangle) -> Self {
        let scene = Scene::new(frame);

        NavBar {
            frame: frame.clone(),
            scene,
            color: None,
            title_ptr: None,
            left_btns: Vec::new(),
            right_btns: Vec::new(),
            layout: None
        }
    }

    pub fn reset(&mut self) {
        self.scene.views.clear();
        self.scene.controls.clear();
        self.left_btns.clear();
        self.right_btns.clear();
        self.title_ptr = None;
    }

    pub fn set_title(&mut self, title: &str) {
        let label = Label::new(&self.frame, title);
        if let Some(idx) = &self.title_ptr {
            self.scene.views[*idx] = Rc::new(RefCell::new(label));
        } else {
            self.scene.views.push(Rc::new(RefCell::new(label)));
            self.title_ptr = Some(self.scene.views.len() - 1);
        }
    }

    pub fn add_left_button(&mut self, button: Button) {
        self.scene.controls.push(Rc::new(RefCell::new(button)));
        self.left_btns.push(self.scene.controls.len() - 1);
    }

    pub fn add_right_button(&mut self, button: Button) {
        self.scene.controls.push(Rc::new(RefCell::new(button)));
        self.right_btns.push(self.scene.controls.len() - 1);
    }

    /// This layout defines a % split of 20-60-20 for the 3 sections. Each section has children nodes and
    /// only one node leaf is defined in each. Others could be added later.
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    pub fn layout_views(&mut self) {

        let cell_padding = Rect {
            start: Dimension::Points(8.0),
            end: Dimension::Points(8.0),
            top: Dimension::Points(5.0),
            bottom: Dimension::Points(5.0),
            ..Default::default()
        };

        let node = Node::new(
            Style {
                size: Size { width: Dimension::Points(self.frame.size.x), height: Dimension::Points(50.0) },
                ..Default::default()
            },
            vec![
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.2), height: Dimension::Auto },
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        padding: cell_padding,
                        ..Default::default()
                    },
                    vec![],
                ),
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.6), height: Dimension::Auto },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: cell_padding,
                        ..Default::default()
                    },
                    vec![],
                ),
                &Node::new(
                    Style {
                        size: Size { width: Dimension::Percent(0.2), height: Dimension::Auto },
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        padding: cell_padding,
                        ..Default::default()
                    },
                    vec![],
                ),
            ],
        );
        const LEFT_NODE: usize = 0;
        const TITLE_NODE: usize = 1;
        const RIGHT_NODE: usize = 2;

        // Iterate through the matching scene.buttons and create placeholders in the layout
        for idx in &self.left_btns {
            let cell = &mut self.scene.controls[*idx];
            let size = (cell.borrow()).get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[LEFT_NODE].add_child(&leaf);
        }
        for idx in &self.right_btns {
            let cell = &mut self.scene.controls[*idx];
            let size = (cell.borrow()).get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[RIGHT_NODE].add_child(&leaf);
        }
        // Add the title node
        if let Some(idx) = &self.title_ptr {
            let cell = &mut self.scene.views[*idx];
            let size = (cell.borrow()).get_content_size();
            let node_size = Size { width: size.x, height: size.y };
            let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
            node.children()[TITLE_NODE].add_child(&leaf);
        }

        let layout = node.compute_layout(Size::undefined()).unwrap();

        let solver = LayoutSolver {};
        let abs_layout = solver.absolute_layout(&layout);
        eprintln!("node_layout={:#?}", abs_layout);

        // Now that layout has been computed, reposition all the scene.buttons
        for (i, idx) in self.left_btns.iter().enumerate() {
            let item = &abs_layout.children[LEFT_NODE].children[i];
            eprintln!("[{}] left={:?}", i, item.location);
            let cell = &mut self.scene.controls[*idx];
            (cell.borrow_mut()).set_origin(&Vector::new(item.location.x, item.location.y));
        }
        for (i, idx) in self.right_btns.iter().enumerate() {
            let item = &abs_layout.children[RIGHT_NODE].children[i];
            eprintln!("[{}] right={:?}", i, item.location);
            let cell = &mut self.scene.controls[*idx];
            (cell.borrow_mut()).set_origin(&Vector::new(item.location.x, item.location.y));
        }

        self.layout = Some(layout);
    }

    /// First renders the background and then the scene content
    pub fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        if let Some(color) = &self.color {
            window.draw(&self.frame, Col(*color));
        }
        let _ = self.scene.render(theme, window);
    }
}

