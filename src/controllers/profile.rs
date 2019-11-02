use super::*;
use crate::application::*;

use std::cell::RefCell;
use std::rc::{Rc};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color},
    lifecycle::{Window},
};

#[allow(unused_imports)]
use tweek::{
    core::{AppState},
    events::*,
    gui::{Button, Scene, ShapeView, Displayable, Responder, Theme},
    tools::DrawShape,
};

#[allow(dead_code)]
pub struct ProfileController {
    pub frame: Rectangle,
    pub scene: Scene,
    events: Rc<RefCell<EventQueue>>,
}

impl ProfileController {
    pub fn new(frame: Rectangle) -> ProfileController {
        let scene = Scene::new(&frame);
        let controller = ProfileController {
            frame,
            scene,
            events: EventQueue::new(),
        };
        controller
    }
}

impl Controller for ProfileController {

    fn view_will_load(&mut self) {
        let frame = Rectangle::new((10.0, 70.0), (self.frame.width() - 20.0, self.frame.height() - 90.0));
        let line_color = Color::from_hex("#8B008B");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 8.0, 0.0);
        let shape = ShapeView::new(frame).with_mesh(&mut mesh);
        self.scene.views.push(Rc::new(RefCell::new(shape)));
    }

    fn screen_title(&self) -> &str {
        "Profile"
    }

    // fn left_nav_items(&self) -> Vec<NavItem> {
    //     let mut items: Vec<NavItem> = Vec::new();
    //     let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Close");
    //     let item = NavItem::new(CLOSE_BUTTON, btn);
    //     items.push(item);
    //     items
    // }

    // fn right_nav_items(&self) -> Vec<NavItem> {
    //     let mut items: Vec<NavItem> = Vec::new();
    //     let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Profile");
    //     let item = NavItem::new(MODAL, btn);
    //     items.push(item);
    //     items
    // }

    fn nav_target_for_event(&mut self, _evt: &NavEvent, _ctx: &mut AppContext) -> Option<NavTarget> {
        // let controller = SettingsCont
        None

    }

    fn update(&mut self, _ctx: &mut AppContext, window: &mut Window) {
        // let mut events = self.events.borrow_mut().queue();
        // (*events).clear();
        // for event in events.drain(..) {

        // }
        // *events;

        let _ = self.scene.update(window, Vector::ZERO);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let _ = self.scene.render(theme, window);
        // let _ = self.navbar.render(theme, window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.scene.handle_mouse_at(pt)

    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        println!(">>> handle_mouse_down");
        // if let Some(ref mut rc) = self.nav.upgrade() {
        //     let mut nav = rc.borrow_mut();
        //     (&mut *nav).notify("Booo");
        //     // rc.borrow_mut().notify("Mouse down");
        // }
        self.scene.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        self.scene.handle_mouse_up(pt, state)
    }

}

