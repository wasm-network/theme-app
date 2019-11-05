/// A custom AppDelegate
///
///
///
use super::*;
use crate::controllers::*;

use tweek::{
    core::*,
    // events::*,
    gui::*,
};

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::{ButtonState, Key, MouseButton},
    lifecycle::{Event, State, Window},
    Error, Result,
};

// // Misc
// pub const BG_SCENE_ID: u32 = 100;
// const FPS_INTERVAL: usize = 40;
// const FPS_TAG: u32 = 901;
// const TITLE_TAG: u32 = 902;

//-- Main -----------------------------------------------------------------------

/// AppDelegate serves as a layer between the backend runloop and Tweek UI.
///
pub struct AppDelegate {
    frame: Rectangle,
    theme: Theme,
    app_state: AppState,
    front_controller: NavController,
    frames: usize,
    did_launch: bool,
}

impl AppDelegate {
    /// Constructor
    pub fn new(screen: Vector) -> Self {

        let mut theme = Theme::default();
        theme.font_size = 18.0;
        theme.bg_color = Color::from_hex("#FFFFEE");

        // let mut theme_picker = ThemePicker::new();
        // theme_picker.add_theme(LIGHT_THEME, "Light theme", || {
        //     let theme = ThemeBuilder::light_owl();
        //     theme
        // });
        // theme_picker.add_theme(DARK_THEME, "Dark theme", || {
        //     let theme = ThemeBuilder::night_owl();
        //     theme
        // });
        // let frame = Rectangle::new_sized(screen);
        // let nav_scene = Scene::new(frame);
        // let stage = Stage::new(frame);
        let mut app_state = AppState::new();
        app_state.window_size = (screen.x, screen.y);

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut nav_controller = NavController::new(frame.clone());

        let app = AppDelegate {
            frame,
            theme,
            app_state,
            front_controller: nav_controller,
            frames: 0,
            did_launch: false,
        };
        app
    }

    /// Application lifecycle event called before runloop starts
    pub fn application_ready(&mut self) {

        let controller = ThemeEditor::new(self.frame.clone());
        self.front_controller.push_controller(Box::new(controller));
        // self.front_controller.set
        self.front_controller.view_will_load();
        self.front_controller.set_theme(&mut self.theme);
    }

}

// ************************************************************************************
// ************************************************************************************

#[allow(dead_code)]
#[allow(unused_variables)]
impl State for AppDelegate {
    fn new() -> Result<AppDelegate> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        for event in self.app_state.event_bus.into_iter() {
            // if let Ok(evt) = event.downcast_ref::<NavEvent>() {
            //     log::debug!("NavEvent={:?}", evt);
            //     match evt {
            //         NavEvent::Next => {
            //             self.view_index += 1;
            //             if self.view_index == self.stage_builders.len() {
            //                 self.view_index = 0;
            //             }
            //             self.load_scene();
            //             return Ok(());
            //         }
            //         NavEvent::Back => {
            //             if self.view_index == 0 {
            //                 self.view_index = self.stage_builders.len() - 1;
            //             } else {
            //                 self.view_index -= 1;
            //             }
            //             self.load_scene();
            //             return Ok(());
            //         }
            //         _ => (),
            //     }
            // }
            // if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
            //     log::debug!("SceneEvent={:?}", evt);
            //     log::debug!("Source={:?}", event.event_info());
            //     match evt {
            //         SceneEvent::Show(_) => {
            //             self.nav_scene.is_interactive = false;
            //         }
            //         SceneEvent::Hide(_) => {
            //             self.nav_scene.is_interactive = true;
            //         }
            //         _ => (),
            //     }
            //     self.stage.handle_event(evt, &event.event_info());
            // }
            // if let Ok(evt) = event.downcast_ref::<ThemeEvent>() {
            //     log::debug!("ThemeEvent={:?}", evt);
            //     match evt {
            //         ThemeEvent::Change(id) => {
            //             if let Some(theme) = self.theme_picker.load_theme(*id) {
            //                 self.theme = theme;
            //                 self.stage.set_theme(&mut self.theme);
            //             }
            //         } // _ => ()
            //     }
            // }
        }
        self.front_controller.update(window, &mut self.app_state);

        // self.frames += 1;
        // if (self.frames % FPS_INTERVAL) == 0 {
        //     self.frames = 0;
        //     let out = format!("FPS: {:.2}", window.current_fps());
        //     self.nav_scene.set_field_value(&FieldValue::Text(out), TypeId::of::<Text>(), FPS_TAG);
        // }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(self.theme.bg_color)?;
        self.front_controller.render(&mut self.theme, window);
        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("event={:?}", event);
            }
            Event::MouseMoved(pt) => {
                let mut hover: bool = false;
                self.front_controller.handle_mouse_at(pt, window);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.front_controller.handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                // if self.nav_scene.is_interactive {
                //     self.nav_scene.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
                // }
                self.front_controller.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseWheel(xy) => {
                self.front_controller.handle_mouse_scroll(xy, &mut self.app_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    // self.front_controller.handle_key_command(key, window);
                }
            },
            Event::Typed(c) => {
                // self.front_controller.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}
