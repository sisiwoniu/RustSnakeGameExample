use opengl_graphics::*;
use piston::*;
use std::collections::LinkedList;

pub struct App {
	pub gl: GlGraphics,
	pub snake: Snake,
}

impl App {
	pub fn render(&mut self, arg: &RenderArgs) {
		let COLOR: [f32; 4] = [0.1, 0.5, 0.2, 1.0];
		self.gl
			.draw(arg.viewport(), |c, gl| graphics::clear(COLOR, gl));
		self.snake.render(&mut self.gl, arg);
	}

	pub fn update(&mut self) {
		self.snake.update();
	}

	pub fn pressed(&mut self, inputDir: &Direction) {
		self.snake.dir = inputDir.clone();
	}
}

pub struct Snake {
	pub body: LinkedList<(i32, i32)>,
	pub dir: Direction,
}

impl Snake {
	fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
		let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let square: Vec<graphics::types::Rectangle> = self
			.body
			.iter()
			.map(|(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
			.collect();

		gl.draw(arg.viewport(), |c, g| {
			let transform = c.transform;
			//
			square
				.into_iter()
				.for_each(|square| graphics::rectangle(RED, square, transform, g))
		});
	}

	fn update(&mut self) {
		let mut new_Head = (self.body.front().expect("snake ha no body")).clone();

		match self.dir {
			Direction::Right => new_Head.0 += 1,
			Direction::Left => new_Head.0 -= 1,
			Direction::Up => new_Head.1 -= 1,
			Direction::Down => new_Head.1 += 1,
			_ => {}
		}

		self.body.push_front(new_Head);
		self.body.pop_back().unwrap();
	}
}

#[derive(Clone)]
pub enum Direction {
	Right,
	Left,
	Up,
	Down,
	None,
}
