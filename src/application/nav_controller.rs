use super::*;

use std::cell::RefCell;
use std::rc::Rc;

use quicksilver::{
    geom::{Rectangle, Vector},
    // graphics::Color,
    lifecycle::Window
};

use tweek::{
    core::{AppState},
    events::*,
    gui::{Button, Displayable, Responder, Theme},
};

// Magic numbers for different nav commands
pub const MODAL: u32 = 1;
pub const BACK_BUTTON: u32 = 10;
pub const CLOSE_BUTTON: u32 = 11;
pub const NEXT_BUTTON: u32 = 20;

pub struct NavTarget {
    // pub nav_event: NavEvent,
    pub controller: Rc<RefCell<dyn Controller>>,
}

pub struct NavItem {
    /// A custom id value used to uniquely identify the button click
    pub tag: u32,
    /// A simple button which has been pre-sized to fit within the narrow space of navbar items
    pub button: Button,
    /// A tuple of pixel padding values in the format: left, top, right, bottom
    pub padding: (f32, f32, f32, f32),
}

impl NavItem {
    pub fn new(tag: u32, button: Button) -> Self {
        NavItem {
            tag,
            button,
            padding: (0.0, 0.0, 0.0, 0.0)
        }
    }
}
#[allow(dead_code)]
pub struct NavController {
    frame: Rectangle,
    /// The controllers in the navigation stack.
    controllers: Vec<Rc<RefCell<dyn Controller>>>,
    /// Optional controller that can appear above this NavController
    modal_controller: Option<Rc<RefCell<dyn Controller>>>,
    /// The index of the front view controller in the stack. Usually the last one, but not always.
    front_idx: usize,
    /// The standard nav bar which has buttons on left and right side. Should be optional later
    // navbar: NavBar,
    // events: Rc<RefCell<EventQueue>>,
    // next_target: Option<NavTarget>,
    transition: TransitionState,
    event_layer: EventLayer,
}

impl NavController {
    pub fn new(frame: Rectangle) -> Self {
        let nav_frame = Rectangle::new((0.0, 0.0), (frame.width(), 50.0));
        // let navbar = NavBar::new(&nav_frame);

        let nav = NavController {
            frame: frame,
            controllers: Vec::new(),
            modal_controller: None,
            front_idx: 0,
            // navbar,
            // events: EventQueue::new(),
            // next_target: None,
            transition: TransitionState::None,
            event_layer: EventLayer::new(),
        };
        nav
    }

    pub fn push_controller(&mut self, controller: Rc<RefCell<dyn Controller>>) {
        self.controllers.push(controller);
        self.front_idx = self.controllers.len() - 1;
        self.view_will_load();
        // TODO: Transition
    }

    pub fn pop_controller(&mut self) {
        if self.controllers.len() > 1 {
            let _ = self.controllers.pop();
            self.front_idx = self.controllers.len() - 1;
        }
    }

    pub fn present_controller(&mut self, controller: Rc<RefCell<dyn Controller>>, style: ModalDisplayStyle) {
        match style {
            ModalDisplayStyle::None => {
                controller.borrow_mut().view_will_load();
                self.modal_controller = Some(controller);
                self.transition = TransitionState::Starting;
            }
            _ => {}
        }
    }

    pub fn notify(&self, message: &str) {
        eprintln!("nav message={:?}", message);
    }

}

impl Controller for NavController {


    fn view_will_load(&mut self) {
        // FIXME: Stop creating copies
        // self.navbar.reset();
        // let theme = ThemeManager::nav_theme();
        // self.navbar.color = Some(theme.bg_color);
        // let cell = Rc::new(RefCell::new(self.event_layer));
        // self.events.borrow_mut().add_handler(cell);
        if self.front_idx >= self.controllers.len() {
            return;
        }
        let mut controller = self.controllers[self.front_idx].borrow_mut();

        // for item in controller.left_nav_items() {
        //     let events = self.events.clone();
        //     let mut btn = item.button;
        //     let tag = item.tag;
        //     btn.set_onclick(move |_action, _tk| {
        //         let mut notifier = Notifier::new();
        //         events.borrow().register_to(&mut notifier);
        //         let evt = Event::new(Action::Button(tag));
        //         notifier.notify(evt);
        //     });
        //     self.navbar.add_left_button(btn);
        // }

        // for item in controller.right_nav_items() {
        //     let events = self.events.clone();
        //     let mut btn = item.button;
        //     let tag = item.tag;
        //     btn.set_onclick(move |_action, _tk| {
        //         let mut notifier = Notifier::new();
        //         events.borrow().register_to(&mut notifier);
        //         let evt = Event::new(Action::Button(tag));
        //         notifier.notify(evt);
        //     });
        //     self.navbar.add_right_button(btn);
        // }

        // self.navbar.set_title(controller.screen_title());
        // self.navbar.layout_views();
        (&mut *controller).view_will_load();
    }

    fn view_will_transition(&mut self, event: NavEvent) {
        println!(">>> view_will_transition {:?}", event);
        // match event {
        //     NavEvent::Back => {
        //         self.pop_controller();
        //     }
        //     NavEvent::Next => {
        //         if let Some(target) = &self.next_target {
        //             // Clone it first to avoid this problem:
        //             // https://github.com/rust-lang/rust/issues/59159
        //             // Note this is a Rc-Refcell clone.
        //             let mc = target.controller.clone();
        //             self.push_controller(mc);
        //             self.next_target = None;
        //         }
        //     }
        //     NavEvent::Modal => {
        //         if let Some(target) = &self.next_target {
        //             let mc = target.controller.clone();
        //             self.present_controller(mc, ModalDisplayStyle::None);
        //             self.next_target = None;
        //         }
        //     }
        //     _ => {}
        // }
    }

    #[allow(dead_code)]
    #[allow(unreachable_patterns)]
    fn update(&mut self, _window: &mut Window) {
        // let mut nav_event: Option<NavEvent> = None;
        // Only handle one event per run loop cycle.
        // if let Some(event) = self.events.borrow_mut().queue().pop() {
        //     match event.action {
        //         Action::Button(tag) => {
        //             match tag {
        //                 BACK_BUTTON => { nav_event = Some(NavEvent::Back) },
        //                 NEXT_BUTTON => { nav_event = Some(NavEvent::Next) },
        //                 MODAL => { nav_event = Some(NavEvent::Modal) },
        //                 _ => {}
        //             }
        //         },
        //         Action::Selected(idx) => { nav_event = Some(NavEvent::Selected(idx)) },
        //         _ => {}
        //     }
        // }

        // if let Some(evt) = nav_event {
        //     if let Some(controller) = &mut self.controllers.get_mut(self.front_idx) {
        //         if let Some(target) = controller.borrow_mut().nav_target_for_event(&evt, ctx) {
        //             self.next_target = Some(target);
        //         }
        //     }
        //     ctx.event_bus.register_event(evt);
        // }

        // if let Some(modal) = &mut self.modal_controller {
        //     // eprintln!("update modal");
        //     modal.borrow_mut().update(ctx, window);
        // }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // let _ = self.scene.render(theme, window);
        // let _ = self.navbar.render(theme, window);
        if let Some(cell) = &mut self.controllers.get_mut(self.front_idx) {
            (cell.borrow_mut()).render(theme, window);
        }
        if let Some(modal) = &mut self.modal_controller {
            // eprintln!("render modal");
            modal.borrow_mut().render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        // self.navbar.scene.handle_mouse_at(pt)
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        println!(">>> NAV handle_mouse_down");
        // self.navbar.scene.handle_mouse_down(pt, state);
        if let Some(cell) = &mut self.controllers.get_mut(self.front_idx) {
            (cell.borrow_mut()).handle_mouse_down(pt, state);
        }
        false
        // self.scene.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut AppState) -> bool {
        // self.navbar.scene.handle_mouse_up(pt, state)
        // self.scene.handle_mouse_up(pt, state)
        false
    }

}

#[derive(Debug, Clone, Copy)]
pub struct EventLayer {
    pub id: u32,
    // controller:
}

impl EventLayer {
    pub fn new() -> Self {
        EventLayer {
            id: 0,
        }
    }
}

// impl Copy for EventLayer { }

// impl Clone for EventLayer {
//     fn clone(&self) -> EventLayer {
//         *self
//     }
// }

// impl EventDelegate for EventLayer {
//     fn handle_event(&mut self, event: Event) {
//         eprintln!("$$$$ EventLayer handle_event: {:?}", event);

//         match event {
//             NAV_BACK => {
//                 // self.parent().
//                 // tell
//             }
//         }
//     }
// }
