extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct Game {
    rotation: f64
}

impl Game {
    fn new() -> Game {
        Game { rotation: 0.0 }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
    }

    fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let center = c.transform.trans((ren.width/2) as f64,
                                            (ren.height/2) as f64);
            let square = rectangle::square(0.0,0.0,100.0);

            rectangle(RED, square, center.rot_rad(self.rotation).trans(-25.0, -25.0), g);
        })
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    for e in window {
        match e.event {
            Some(Event::Update(upd)) => {game.on_update(upd);},
            Some(Event::Render(ren)) => {game.on_draw(ren, e);},
            _ => {}
        }
    }
}
