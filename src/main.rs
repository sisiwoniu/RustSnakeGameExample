mod app;
use app::hosting::*;
use opengl_graphics::*;
use piston::*;
use piston_window::*;
use std::collections::LinkedList;

const WINDOW_TITLE: &str = "Snake Game";
const WINDOW_SIZE: Size = Size {
    width: 400.0,
    height: 400.0,
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let mut app = App {
        gl: GlGraphics::new(OpenGL::V3_2),
        snake: Snake {
            body: LinkedList::from_iter(vec![(0, 0), (0, 1)].into_iter()),
            dir: Direction::None,
        },
    };

    window.events.set_max_fps(60);
    window.events.set_ups(8);

    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(arg)) => {
                app.render(&arg);
            }

            Event::Loop(Loop::Update(_)) => {
                app.update();
            }
            Event::Input(i, _) => {
                if let Input::Button(key) = i {
                    if key.state == ButtonState::Press {
                        let mut dir: Direction = Direction::None;
                        match key.button {
                            Button::Keyboard(Key::Up) => {
                                dir = Direction::Up;
                            }
                            Button::Keyboard(Key::Down) => {
                                dir = Direction::Down;
                            }
                            Button::Keyboard(Key::Left) => {
                                dir = Direction::Left;
                            }
                            Button::Keyboard(Key::Right) => {
                                dir = Direction::Right;
                            }
                            _ => {}
                        }

                        app.pressed(&dir);
                    }
                }
            }
            _ => {}
        }
    }
}
