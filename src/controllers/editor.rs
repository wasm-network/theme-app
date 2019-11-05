use super::*;
use crate::application::*;

use std::cell::RefCell;
use std::rc::Rc;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color},
    lifecycle::{Window},
};

use tweek::{
    core::{AppState},
    events::*,
    gui::*,
    tools::DrawShape,
};

#[allow(dead_code)]
pub struct ThemeEditor {
    frame: Rectangle,
    stage: Stage
}

impl ThemeEditor {
    pub fn new(frame: Rectangle) -> ThemeEditor {
        let stage = Stage::new(frame.clone());
        let controller = ThemeEditor {
            frame,
            stage,
        };
        controller
    }

    fn build_stage(frame: Rectangle) -> Stage {
        let mut stage = Stage::new(frame.clone());
        stage.title = "Theme Builder".to_string();

        let mut scene = Scene::new(frame);

        let numbers: Vec<u32> = (0..21).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        let frame = Rectangle::new((100.0, 200.0), (300.0, 200.0));
        let mut listbox = ListBox::new(frame);
        listbox.set_datasource(ds);
        listbox.row_border_style = BorderStyle::SolidLine(Color::from_hex("#EEEEEE"), 1.0);
        scene.add_control(Box::new(listbox));

        stage.add_scene(scene);
        stage
    }
}

impl Controller for ThemeEditor {

    fn view_will_load(&mut self) {
        self.stage = ThemeEditor::build_stage(self.frame.clone());
    }

    fn screen_title(&self) -> &str {
        "Theme Builder"
    }

    fn left_nav_items(&self) -> Vec<NavItem> {
        let mut items: Vec<NavItem> = Vec::new();
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Back");
        let item = NavItem::new(BACK_BUTTON, btn);
        items.push(item);
        items
    }

    fn right_nav_items(&self) -> Vec<NavItem> {
        let mut items: Vec<NavItem> = Vec::new();
        let btn = Button::new(Rectangle::new((0.0, 0.0), (40.0, 30.0))).with_text("Next");
        let item = NavItem::new(NEXT_BUTTON, btn);
        items.push(item);
        items
    }

    // fn nav_target_for_event(&mut self, event: &NavEvent, _ctx: &mut AppContext) -> Option<NavTarget> {
    //     match event {
    //         NavEvent::Next => {
    //             let controller = SettingsController::new(self.frame.clone());
    //             let target = NavTarget {
    //                 nav_event: event.clone(),
    //                 controller: Rc::new(RefCell::new(controller))
    //             };
    //             return Some(target);
    //         }
    //         _ => ()
    //     }
    //     None
    // }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        // This is just placeholder code for future consideration of what kinds of events
        // might get queued within this controller.
        // let mut nav_event: Option<NavEvent> = None;
        // if let Some(event) = self.events.borrow_mut().queue().first() {
        //     match event.action {
        //         Action::Button(tag) => {
        //             match tag {
        //                 BACK_BUTTON => { nav_event = Some(NavEvent::Back) },
        //                 NEXT_BUTTON => { nav_event = Some(NavEvent::Next) },
        //                 _ => {}
        //             }
        //         },
        //         Action::Selected(idx) => { nav_event = Some(NavEvent::Selected(idx)) },
        //         // _ => {}
        //     }
        // }
        // if let Some(evt) = nav_event {
        //     ctx.event_bus.register_event(evt);
        // }

        let _ = self.stage.update(window, state);

    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let _ = self.stage.render(theme, window);
        // let _ = self.navbar.render(theme, window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        self.stage.handle_mouse_at(pt, window)

    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        println!(">>> handle_mouse_down");
        // if let Some(ref mut rc) = self.nav.upgrade() {
        //     let mut nav = rc.borrow_mut();
        //     (&mut *nav).notify("Booo");
        //     // rc.borrow_mut().notify("Mouse down");
        // }
        self.stage.handle_mouse_down(pt, state)
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        self.stage.handle_mouse_up(pt, state)
    }

}

