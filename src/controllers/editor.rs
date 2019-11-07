use crate::application::*;
use crate::utils::*;

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
    node::{Node, Stretch},
    result::Layout,
    style::*
};

const COL1_WIDTH: f32 = 200.0;
const COL2_WIDTH: f32 = 600.0;
const COL3_WIDTH: f32 = 200.0;
const MINI_BUTTON_SIZE: f32 = 32.0;

#[allow(dead_code)]
pub struct ThemeEditor {
    frame: Rectangle,
    stage: Stage,
    theme_picker: ThemePicker,
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
        let scene = self.explorer_scene(&col1);
        stage.add_scene(scene);

        // Make Scene for Column 2
        let scene = self.main_scene(&col2);
        stage.add_scene(scene);

        // Make Scene for Column 3
        let mut scene = Scene::new(col3).with_id(3, "Properties");
        scene.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#000000"), 1.0);
        stage.add_scene(scene);

        stage
    }

    /// Define the first column layout
    /// V:|-[themes title]-|-[listbox]-|-
    ///     H:|-[add button]-|-[remove button]-|
    /// -|-[empty space]-|
    ///
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    fn explorer_scene(&mut self, frame: &Rectangle) -> Scene {

        let mut scene = Scene::new(frame.clone()).with_id(1, "Explorer");
        scene.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);

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

        // Toolbar buttons to add/remove
        let subframe = scene.sub_frame((0.0, 250.0), (MINI_BUTTON_SIZE, MINI_BUTTON_SIZE));
        let mut button = Button::new(subframe).with_text("+");
        button.layer.font_style = FontStyle::new(20.0, Color::BLACK);
        button.layer.lock_style = true;
        button.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);
        scene.add_control(Box::new(button));

        let subframe = scene.sub_frame((MINI_BUTTON_SIZE, 250.0), (MINI_BUTTON_SIZE, MINI_BUTTON_SIZE));
        let mut button = Button::new(subframe).with_text("–");
        button.layer.font_style = FontStyle::new(20.0, Color::BLACK);
        button.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);
        button.layer.lock_style = true;
        scene.add_control(Box::new(button));

        scene
    }

    /// Layout spec:
    ///
    /// V:|-[Text]-|-
    ///     H:|-[add button]-|-[remove button]-|
    /// -|-[empty space]-|
    ///
    fn main_scene(&self, frame: &Rectangle) -> Scene {
        let mut scene = Scene::new(frame.clone()).with_id(2, "Main");
        scene.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#999999"), 1.0);

        self.main_scene_layout(&frame);
        scene
    }

    /// Main Scene layout :
    ///
    ///
    /// Column 1:
    /// * Text
    /// * Buttons: normal and primary
    /// * Text Field
    /// * Text Area
    /// Column 2:
    /// * ListBox
    /// * Checkbox
    /// * OptionGroup with radio buttons
    /// See: https://vislyhq.github.io/stretch/docs/rust/
    fn main_scene_layout(&self, frame: &Rectangle) {

        const HEADER_H: f32 = 50.0;
        let body_padding = Rect {
            start: Dimension::Points(10.0),
            end: Dimension::Points(10.0),
            top: Dimension::Points(10.0),
            bottom: Dimension::Points(10.0),
            ..Default::default()
        };
        let item_padding = Rect {
            start: Dimension::Points(10.0),
            end: Dimension::Points(10.0),
            top: Dimension::Points(10.0),
            bottom: Dimension::Points(10.0),
            ..Default::default()
        };

        let mut builder = LayoutBuilder::new().with_style(Style {
                size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(frame.height()) },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
        });
        let header_node = builder.add_row(builder.root, HEADER_H, None);
        let column_w = frame.width()/2.0;
//         let mut stretch = Stretch::new().with_style(style);

//         let mut tree = stretch.new_node(
// ,
//             vec![]
//         ).unwrap();

//         // Create header row.
//         let node = stretch.new_node(
//             Style {
//                 size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(HEADER_H) },
//                 ..Default::default()
//             },
//             vec![],
//         ).unwrap();
//         stretch.add_child(tree, node);
        let body_node = builder.add_row(builder.root, frame.height() - HEADER_H, None);
        // Body container for GUI components
        // let mut body_node = stretch.new_node(
        //     Style {
        //         size: Size { width: Dimension::Points(frame.width()), height: Dimension::Points(frame.height() - HEADER_H) },
        //         flex_direction: FlexDirection::Row,
        //         justify_content: JustifyContent::FlexStart,
        //         padding: body_padding,
        //         ..Default::default()
        //     },
        //     vec![],
        // ).unwrap();
        let column1 = builder.add_column(body_node, frame.width()/2.0, None);
        // let mut column1 = stretch.new_node(
        //     Style {
        //         size: Size { width: Dimension::Auto, height: Dimension::Auto },
        //         flex_direction: FlexDirection::Column,
        //         justify_content: JustifyContent::FlexStart,
        //         padding: item_padding,
        //         ..Default::default()
        //     },
        //     vec![],
        // ).unwrap();

        // // let leaf = Node::new_leaf(Style::default(), Box::new(move |_| Ok(node_size)));
        let node = builder.add_object(column1, Size { width: column_w, height: 50.0 });
        let node = builder.add_object(column1, Size { width: column_w, height: 50.0 });
        let node = builder.add_object(column1, Size { width: column_w, height: 200.0 });
        // let node_size = Size { width: frame.width()/2.0, height: 50.0 };
        // let mut thin_row = stretch.new_leaf(
        //     Style {
        //         size: Size { width: Dimension::Auto, height: Dimension::Points(50.0) },
        //         ..Default::default()
        //     },
        //     Box::new(move |_| Ok(node_size)),
        // ).unwrap();
        // stretch.add_child(column1, thin_row);

        // let mut thin_row = stretch.new_leaf(
        //     Style {
        //         size: Size { width: Dimension::Auto, height: Dimension::Points(50.0) },
        //         ..Default::default()
        //     },
        //     Box::new(move |_| Ok(node_size)),
        // ).unwrap();
        // stretch.add_child(column1, thin_row);

        // let node_size = Size { width: frame.width()/2.0, height: 200.0 };
        // let mut fat_row = stretch.new_leaf(
        //     Style {
        //         size: Size { width: Dimension::Auto, height: Dimension::Points(100.0) },
        //         ..Default::default()
        //     },
        //     Box::new(move |_| Ok(node_size)),
        // ).unwrap();
        // stretch.add_child(column1, fat_row);

        // stretch.add_child(body_node, column1);
        // stretch.add_child(tree, body_node);

        // let mut solver = LayoutSolver::new();
        let abs_layout = builder.absolute_layout(builder.root);
        eprintln!("node_layout={:#?}", abs_layout);

        // Ok(*layout)
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

