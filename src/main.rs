use tweek_theme::application::*;

use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, Settings},
};

fn main() {
    let screen = Vector::new(1200, 900);
    run_with("Theme Edit", screen, Settings::default(), || Application::new(screen));
}
