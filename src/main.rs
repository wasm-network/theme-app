// use tweek::prelude::*;
// use prelude::*;
// use theme_lib::application::*;
use tweek_theme::application::*;

use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    let screen = Vector::new(400, 600);
    run_with("Theme Edit", screen, Settings::default(), || Application::new(screen));
}
