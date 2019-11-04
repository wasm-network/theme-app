use super::*;

use tweek::{
    gui::{Stage, Scene, Theme},
};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{Event, State, Window},
    Error, Result,
};

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Application {
    screen: Vector,
    theme: Theme,
    delegate: AppDelegate,
    front_controller: Option<Rc<RefCell<dyn Controller>>>,
}

impl Application {
    pub fn new(screen: Vector) -> Result<Application> {
        std::env::set_var("RUST_LOG", "main=trace,tweek=trace");

        #[cfg(not(target_arch = "wasm32"))]
        env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
        #[cfg(not(target_arch = "wasm32"))]
        color_backtrace::install();

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut nav = NavController::new(frame);

        // let frame = Rectangle::new((0.0, 50.0), (screen.x, screen.y - 50.0));
        // let home = HomeController::new(frame.clone());
        // nav.push_controller(Rc::new(RefCell::new(home)));

        nav.view_will_load();
        let delegate = AppDelegate::new(screen.clone());
        let s = Application {
            screen,
            theme: Theme::default(),
            delegate,
            front_controller: Some(Rc::new(RefCell::new(nav))),
        };
        Ok(s)
    }
    fn build_stage(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);
        let mut stage = Stage::new(frame.clone());
        stage.title = "Theme Edit".to_string();

        let mut scene = Scene::new(frame);

        // let numbers: Vec<u32> = (0..21).collect();
        // let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        // let frame = Rectangle::new((100.0, 200.0), (300.0, 200.0));
        // let mut listbox = ListBox::new(frame);
        // listbox.set_datasource(ds);
        // listbox.row_border_style = BorderStyle::SolidLine(Color::from_hex("#EEEEEE"), 1.0);
        // scene.add_control(Box::new(listbox));

        /* Ignore: This is just an experiment in text clipping */
        // let frame = Rectangle::new((500.0, 200.0), (200.0, 30.0));
        // let mut text = Text::new(frame, "Clip this title");
        // text.layer.font_style = FontStyle::new(20.0, Color::BLACK);
        // text.layer.lock_style = true;
        // text.text_align = TextAlign::Left;
        // text.vert_align = VertAlign::Bottom;
        // text.layer.debug = true;
        // text.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#CCCCCC"), 0.5);
        // let subframe = Rectangle::new((500.0, 220.0), (200.0, 10.0));
        // text.subframe = Some(subframe);
        // scene.add_control(Box::new(text));

        stage.add_scene(scene);
        stage
    }
}

impl State for Application {
    // Initialize the struct
    fn new() -> Result<Application> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if let Some(cell) = &mut self.front_controller {
            // let mut controller = cell.borrow_mut();
            // (&mut *controller).update(&mut self.context, window);

            // TODO: Read EventBus
        }


        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Repaint the entire screen
        window.clear(Color::from_hex("#EEEEEE"))?;
        // Nav controller rendering: If top view controller is a navcontroller,
        // render its subviews
        if let Some(cell) = &mut self.front_controller {
            (cell.borrow_mut()).render(&mut self.theme, window);
        }
        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("size={:?} y={:?}", window.screen_size(), 0);
            }
            Event::MouseMoved(pt) => {
                if let Some(cell) = &mut self.front_controller {
                    let hover = (cell.borrow_mut()).handle_mouse_at(pt);
                    if hover {
                        window.set_cursor(MouseCursor::Hand);
                    } else {
                        window.set_cursor(MouseCursor::Default);
                    }
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                // if let Some(cell) = &mut self.front_controller {
                //     (cell.borrow_mut()).handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
                // }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                // if let Some(cell) = &mut self.front_controller {
                    // (cell.borrow_mut()).handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
                // }
            }
            Event::MouseWheel(_xy) => {
                // self.scene.handle_mouse_scroll(xy, &mut self.app_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    // self.scene.handle_key_command(key, window);
                }
            },
            Event::Typed(_c) => {
                // self.scene.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}
