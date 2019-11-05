use crate::application::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color},
    lifecycle::{Window},
};

use tweek::{
    core::{AppState},
    events::*,
    gui::*,
    tools::*,
};

use stretch::{
    geometry::*,
    node::Node,
    result::Layout,
    style::*
};

const COL1_WIDTH: f32 = 200.0;
const COL2_WIDTH: f32 = 600.0;
const COL3_WIDTH: f32 = 200.0;
const MINI_BUTTON_SIZE: f32 = 40.0;

#[allow(dead_code)]
pub struct ThemeEditor {
    frame: Rectangle,
    stage: Stage,
    theme_picker: ThemePicker,
    layout: Option<Layout>
}

impl ThemeEditor {
    pub fn new(frame: Rectangle) -> ThemeEditor {
        let stage = Stage::new(frame.clone());
        let mut theme_picker = ThemePicker::new();
        theme_picker.add_theme(LIGHT_THEME, "Light theme", || {
            let theme = ThemeBuilder::light_owl();
            theme
        });
        theme_picker.add_theme(DARK_THEME, "Dark theme", || {
            let theme = ThemeBuilder::night_owl();
            theme
        });

        let controller = ThemeEditor {
            frame,
            stage,
            theme_picker,
            layout: None,
        };
        controller
    }

    fn build_stage(&mut self, frame: Rectangle) -> Stage {
        let mut stage = Stage::new(frame.clone());
        stage.title = "Theme Builder".to_string();

        let col1 = Rectangle::new((0.0, 0.0), (COL1_WIDTH, frame.height()));
        let col2 = Rectangle::new((COL1_WIDTH, 0.0), (COL2_WIDTH, frame.height()));
        let col3 = Rectangle::new((COL1_WIDTH + COL2_WIDTH, 0.0), (COL3_WIDTH, frame.height()));

        // Make Scene for Column 1
        let scene = self.layout_explorer_scene(&col1);
        stage.add_scene(scene);

        // Make Scene for Column 2
        let scene = self.layout_main_scene(&col2);
        stage.add_scene(scene);

        // Make Scene for Column 3
        let mut scene = Scene::new(col3).with_id(3, "Properties");
        scene.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#000000"), 1.0);
        stage.add_scene(scene);

        stage
    }

    /// Define the first column layout
    /// V:|<listbox>|<
    ///     H:|<add button>|<remove button>|
    /// >|<empty space>|
    ///
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    fn layout_explorer_scene(&mut self, frame: &Rectangle) -> Scene {

        let mut scene = Scene::new(frame.clone()).with_id(1, "Explorer");
        scene.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);

        let mut layout = Node::new(
            Style {
                size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(frame.height()) },
                ..Default::default()
            },
            vec![]
        );
        let node1 = Node::new(
            Style {
                size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(250.0) },
                ..Default::default()
            },
            vec![],
        );
        layout.add_child(&node1);

        let btn_node = Node::new(
            Style {
                size: Size { width: Dimension::Points(MINI_BUTTON_SIZE), height: Dimension::Points(MINI_BUTTON_SIZE) },
                ..Default::default()
            },
            vec![],
        );

        let node2 = Node::new(
            Style {
                size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(MINI_BUTTON_SIZE) },
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            vec![&btn_node, &btn_node],
        );
        layout.add_child(&node2);

        // Add Themes title
        let subframe = scene.sub_frame((0.0, 0.0), (frame.width(), 50.0));
        let mut text = Text::new(subframe, "Themes");
        text.layer.font_style = FontStyle::new(12.0, Color::RED);
        text.text_align(TextAlign::Center);
        scene.add_control(Box::new(text));

        // Add listbox
        let subframe = scene.sub_frame((0.0, 50.0), (200.0, 200.0));
        let mut listbox = ListBox::new(subframe);
        listbox.row_border_style = BorderStyle::SolidLine(Color::from_hex("#EEEEEE"), 1.0);

        let numbers: Vec<u32> = (0..21).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();
        listbox.set_datasource(ds);
        scene.add_control(Box::new(listbox));
        scene
    }

    fn layout_main_scene(&mut self, frame: &Rectangle) -> Scene {
        let mut scene = Scene::new(frame.clone()).with_id(2, "Main");
        scene.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#999999"), 1.0);
        scene
    }
}

impl Controller for ThemeEditor {

    fn view_will_load(&mut self) {
        self.stage = self.build_stage(self.frame.clone());
        self.stage.notify(&DisplayEvent::Ready);
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        self.stage.set_theme(theme);
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

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut AppState) {
        self.stage.handle_mouse_scroll(pt, state);
    }
}

