extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::even::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::Button::{ Keyboard };
use piston::input::Input;
use piston::input::keyboard::Key;

pub struct Paddle {
    xpos: f64,
    ypos: f64
    height: f64,
    width: f64
}

impl Paddle {
    fn mv_up(&mut self) {
        self.ypos -= 10;
    }

    fn mv_down(&mut self) {
        self.ypos += 10;
    }
}

pub struct Ball {
    xpos: f64,
    ypos: f64,
    xvel: f64,
    yvel: f64,
    width: f64
}

impl Ball {
    fn update(&mut self) {
        self.ypos += self.yvel;
        self.xpos += self.xvel;
    }
}

pub struct App {
    gl: GlGraphics,
    player1: Paddle,
    player2: Paddle,
    ball: Ball
}
