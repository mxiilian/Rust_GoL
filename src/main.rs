extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    // Art Klasse
    gl: GlGraphics, //opengl drawing backend
    rotation: f64,  // für die rotation
}

impl App {
    //hier kommen die Funktionen für die Klasse rein
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
    }
}
