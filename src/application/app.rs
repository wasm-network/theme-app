use super::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    lifecycle::{Event, State, Window},
    Error, Result,
};

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Application {
    screen: Vector,
    delegate: AppDelegate,
}

impl Application {
    pub fn new(screen: Vector) -> Result<Application> {
        std::env::set_var("RUST_LOG", "main=trace,tweek=trace");

        #[cfg(not(target_arch = "wasm32"))]
        env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
        #[cfg(not(target_arch = "wasm32"))]
        color_backtrace::install();

        let delegate = AppDelegate::new(screen.clone());
        let mut app = Application {
            screen,
            delegate,
        };

        app.delegate.application_ready();

        Ok(app)
    }
}

impl State for Application {
    // Initialize the struct
    fn new() -> Result<Application> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // if let Some(cell) = &mut self.front_controller {
        //     let mut controller = cell.borrow_mut();
        //     (&mut *controller).update(&mut self.context, window);

        //     TODO: Read EventBus
        // }
        self.delegate.update(window)?;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Repaint the entire screen
        window.clear(Color::from_hex("#EEEEEE"))?;
        // Nav controller rendering: If top view controller is a navcontroller,
        // render its subviews
        // if let Some(cell) = &mut self.front_controller {
        //     (cell.borrow_mut()).render(&mut self.theme, window);
        // }
        self.delegate.draw(window)?;
        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.delegate.event(event, window)?;
        Ok(())
    }
}
