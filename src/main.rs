extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, Texture };
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
        if (self.ypos > 10.0) {
            self.ypos -= 10.0;
        }
    }

    fn mv_down(&mut self) {
        if (self.ypos < 490.0) {
            self.ypos += 10.0;
        }
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
    fn update(&mut self, p1: f64, p2: f64) {
        // Full collision detection for ball
        if (self.ypos <= 10.0 || self.ypos >= 580.0) {
            self.yvel *= -1.0;
            self.ypos += self.yvel;
        } else if (self.xpos == 20.0 && self.ypos >= p1 && self.ypos <= (p1+100.0)) {
            self.xvel *= -1.0;
            self.xpos += self.xvel;
        } else if (self.xpos == 770.0 && self.ypos >= p2 && self.ypos <= (p2+100.0)) {
            self.xvel *= -1.0;
            self.xpos += self.xvel;
        } else if (self.xpos <= 5.0 || self.xpos >= 785.0) {
            self.xpos = 390.0;
            self.ypos = 290.0;
        } else {
            self.xpos += self.xvel;
            self.ypos += self.yvel;
        }
    }
}

pub struct App {
    gl: GlGraphics,
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    logo: Texture
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(self.ball.xpos, self.ball.ypos, self.ball.width);
        let rec1 = [self.player1.xpos, self.player1.ypos, self.player1.width, self.player1.height];
        let rec2 = [self.player2.xpos, self.player2.ypos, self.player2.width, self.player2.height];
        let logo = &self.logo;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // Rust logo
            image(logo, c.transform, gl);
            // Player 1 paddle
            rectangle(BLUE, rec1, c.transform, gl);
            // Player 2 paddle
            rectangle(BLUE, rec2, c.transform, gl);
            // Ball
            rectangle(BLUE, square, c.transform, gl);
        });
    }

    fn handle_input(&mut self, i: Input) {
        match i {
            Input::Press(Keyboard(Key::F)) => {
                self.player1.mv_up();
            }
            Input::Press(Keyboard(Key::D)) => {
                self.player1.mv_down();
            }
            Input::Press(Keyboard(Key::J)) => {
                self.player2.mv_up();
            }
            Input::Press(Keyboard(Key::K)) => {
                self.player2.mv_down();
            }
           _ => {}
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.update(self.player1.ypos, self.player2.ypos);
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
        xvel: -0.5,
        yvel: 1.0,
        width: 10.0
    };

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rust.png");
    let rust_logo = Texture::from_path(rust_logo).unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        player1: p1,
        player2: p2,
        ball: ball,
        logo: rust_logo
    };

    for e in window.events() {
        match e {
            Event::Render(args) => {
                app.render(&args);
            }
            Event::Input(i) => {
                app.handle_input(i);
            }
            Event::Update(args) => {
                app.update(&args);
            }
            _ => {}
        }
    }
}
