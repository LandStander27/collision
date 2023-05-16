use macroquad::prelude::*;

const density: f32 = 2.0;

#[derive(Copy, Clone)]
struct Ball {
	x: f32,
	y: f32,
	r: f32,
	color: Color,
	v: Vec2,
	mass: f32,
	dragging: bool,
}

impl Ball {
	fn new(x: f32, y: f32) -> Self {

		let colors: Vec<Color> = Vec::from([RED, BLUE, YELLOW, GREEN, BLACK]);
		let color = colors[rand::gen_range(0, colors.len())];

		let radius = rand::gen_range(10, 20);

		// let x = rand::gen_range(radius, screen_width().round() as i32 - radius);
		// let y = rand::gen_range(radius, screen_height().round() as i32 - radius);

		// let velocity = Vec2::from((rand::gen_range(-100, 100) as f32 / 100.0, rand::gen_range(-100, 100) as f32 / 100.0));

		let mass = density * radius as f32;

		return Self {
			x: x,
			y: y,
			r: radius as f32,
			color: color,
			v: Vec2::from((0.0, 0.0)),
			mass: mass,
			dragging: false,
		};

	}

	fn draw(&self) {
		draw_circle(self.x, self.y, self.r, self.color);
	}

	fn next_frame(&mut self) {
		if self.dragging {
			return;
		}
		self.x += self.v.x;
		self.y += self.v.y;
	}

	fn check_collision(&mut self, other: &Ball) -> bool {

		if Vec2::from((self.x, self.y)).distance(Vec2::from((other.x, other.y))) <= self.r + other.r {
			self.calculate_collision(&other.clone());
			return true;
		}

		return false;

	}

	fn overlapping(&self, x: f32, y: f32, cushion: f32) -> bool {
		if Vec2::from((self.x, self.y)).distance(Vec2::from((x, y))) <= self.r + cushion {
			return true;
		}

		return false;
	}

	fn calculate_collision(&mut self, other: &Ball) {

		let pos = Vec2::from((self.x, self.y));
		let other_pos = Vec2::from((other.x, other.y));

		let pos_diff = pos - other_pos;

		let normal = pos_diff.normalize();
		let tangent = Vec2::from((-normal.y, normal.x));

		let new_vel = self.v.dot(normal);
		let new_tangent = self.v.dot(tangent);
		let other_new_tangent = other.v.dot(normal);

		let mass_calc = (new_vel * (self.mass - other.mass) + 2.0 * other.mass * other_new_tangent) / (self.mass + other.mass);

		let final_vel = mass_calc * normal;
		let final_tangent = new_tangent * tangent;

		let final_v = final_vel + final_tangent;

		if (self.v - other.v).dot(pos - other_pos) < 0.0 {
			self.v = final_v;
		}
		
	}

	fn check_wall_collision(&mut self) {
		if self.x <= self.r {
			self.v.x *= -1.0;
			self.x = self.r + 1.0;
		}
		if self.x >= screen_width() - self.r {
			self.v.x *= -1.0;
			self.x = screen_width() - self.r - 1.0;
		}
		if self.y <= self.r {
			self.v.y *= -1.0;
			self.y = self.r + 1.0;
		}
		if self.y >= screen_height() - self.r {
			self.v.y *= -1.0;
			self.y = screen_height() - self.r - 1.0;
		}
	}

	fn drag(&mut self, x: f32, y: f32) {

		let x_off = x - self.x;
		let y_off = y - self.y;

		self.x += x_off / 10.0;
		self.y += y_off / 10.0;

	}

}

fn window_conf() -> Conf {
	return Conf {
		window_title: "Collision Simulator".to_string(),
		fullscreen: true,
		..Default::default()
	};
}

// fn ease(x: f64, max: f64) -> f64 {
// 	return 1.0 - ((((x / max) * std::f64::consts::PI) / 2.0).cos());
// }

#[macroquad::main(window_conf)]
async fn main() {

	let mut circles: Vec<Ball> = Vec::new();

	let mut chosen_ball: Option<usize> = None;
	let mut created_ball: Option<usize> = None;

	loop {

		clear_background(WHITE);

		let pos = mouse_position();
		if is_mouse_button_pressed(MouseButton::Left) {
			if chosen_ball.is_none() {
				let mut selecting = false;
				for i in 0..circles.len() {
					if circles[i].overlapping(pos.0, pos.1, 5.0) {
						selecting = true;
						chosen_ball = Some(i);
						circles[i].dragging = true;
					}
				}

				if !selecting {
					circles.push(Ball::new(pos.0, pos.1));
					circles.last_mut().unwrap().dragging = true;
					created_ball = Some(circles.len()-1);
				}

			}
		}

		if is_mouse_button_down(MouseButton::Left) && chosen_ball.is_some() {
			circles[chosen_ball.unwrap()].drag(pos.0, pos.1);
		}

		if is_mouse_button_released(MouseButton::Left) && chosen_ball.is_some() {
			circles[chosen_ball.unwrap()].dragging = false;
			chosen_ball = None;
		}
		
		if is_mouse_button_released(MouseButton::Left) && created_ball.is_some() {
			let mut ball = &mut circles[created_ball.unwrap()];
			
			ball.v.x = -(pos.0 - ball.x) * 0.01;
			ball.v.y = -(pos.1 - ball.y) * 0.01;
			ball.dragging = false;
.
			created_ball = None;
		}

		let cloned = &circles.clone();
		let cloned_slice = cloned.as_slice();

		for i in 0..circles.len() {
			for j in 0..cloned_slice.len() {

				if i == j {
					continue;
				}

				if chosen_ball.is_some() {
					if chosen_ball.unwrap() == i || chosen_ball.unwrap() == j {
						continue;
					}
				}

				if created_ball.is_some() {
					if created_ball.unwrap() == i || created_ball.unwrap() == j {
						continue;
					}
				}

				circles[i].check_collision(&cloned_slice[j]);

			}
		}

		for i in &mut circles {
			i.check_wall_collision();
			i.next_frame();
			i.draw();
		}

		next_frame().await;

	}

}
