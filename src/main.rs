extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::Button::{ Keyboard };
use piston::input::Input;
use piston::input::keyboard::Key;

pub struct Paddle {
    xpos: f64,
    ypos: f64,
    height: f64,
    width: f64
}

impl Paddle {
    fn mv_up(&mut self) {
        self.ypos -= 10.0;
    }

    fn mv_down(&mut self) {
        self.ypos += 10.0;
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

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(self.ball.xpos, self.ball.ypos, self.ball.width);
        let rec1 = [self.player1.xpos, self.player1.ypos, self.player1.width, self.player1.height];
        let rec2 = [self.player2.xpos, self.player2.ypos, self.player2.width, self.player2.height];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // Player 1 paddle
            rectangle(BLUE, rec1, c.transform, gl);
            // Player 2 paddle
            rectangle(BLUE, rec2, c.transform, gl);
            // Ball
            rectangle(BLUE, square, c.transform, gl);

        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let window = Window::new(
        WindowSettings::new(
            "Pong Clone",
            [800, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
    );

    let mut p1 = Paddle {
        xpos: 0.0,
        ypos: 250.0,
        height: 100.0,
        width: 20.0
    };
    let mut p2 = Paddle {
        xpos: 780.0,
        ypos: 250.0,
        height: 100.0,
        width: 20.0
    };
    let mut ball = Ball {
        xpos: 390.0,
        ypos: 290.0,
        xvel: 0.0,
        yvel: 0.0,
        width: 10.0
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: p1,
        player2: p2,
        ball: ball
    };

    for e in window.events() {
        match e {
            Event::Render(args) => {
                app.render(&args);
            }
            _ => {}
        }
    }
}
