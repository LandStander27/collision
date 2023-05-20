#![allow(non_upper_case_globals)]

use macroquad::prelude::*;

const density: f32 = 2.0;

#[derive(Copy, Clone)]
struct Ball {
	pos: Vec2,
	r: f32,
	color: Color,
	v: Vec2,
	mass: f32,
	dragging: bool,
	paused: bool,
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
			pos: Vec2::from((x, y)),
			r: radius as f32,
			color: color,
			v: Vec2::from((0.0, 0.0)),
			mass: mass,
			dragging: false,
			paused: false,
		};

	}

	fn new_random(balls: &Vec<Ball>) -> Self {

		let colors: Vec<Color> = Vec::from([RED, BLUE, YELLOW, GREEN, BLACK]);
		let color = colors[rand::gen_range(0, colors.len())];

		let radius = rand::gen_range(10, 20);
		// let radius = 10;

		let mut x = rand::gen_range(radius, screen_width().round() as i32 - radius);
		let mut y = rand::gen_range(radius, screen_height().round() as i32 - radius);

		loop {
			let mut is_in = false;
			for i in balls {
				if i.pos.distance(vec2(x as f32, y as f32)) < i.r + radius as f32 {
					is_in = true;
					x = rand::gen_range(radius, screen_width().round() as i32 - radius);
					y = rand::gen_range(radius, screen_height().round() as i32 - radius);
					break;
				}
			}
			if !is_in {
				break;
			}
		}

		let velocity = Vec2::from((rand::gen_range(-100, 100) as f32 / 100.0, rand::gen_range(-100, 100) as f32 / 100.0));

		let mass = density * radius as f32;

		return Self {
			pos: Vec2::from((x as f32, y as f32)),
			r: radius as f32,
			color: color,
			v: velocity,
			mass: mass,
			dragging: false,
			paused: false,
		};

	}

	fn draw(&self) {
		draw_circle(self.pos.x, self.pos.y, self.r, self.color);
	}

	fn next_frame(&mut self) {
		if self.dragging || self.paused {
			return;
		}
		self.pos.x += self.v.x;
		self.pos.y += self.v.y;
	}

	fn check_collision(&mut self, other: &Ball) -> bool {

		if self.paused {
			return false;
		}

		if Vec2::from((self.pos.x, self.pos.y)).distance(Vec2::from((other.pos.x, other.pos.y))) <= self.r + other.r {
			self.calculate_collision(&other.clone());
			return true;
		}

		return false;

	}

	fn overlapping(&self, x: f32, y: f32, cushion: f32) -> bool {
		if Vec2::from((self.pos.x, self.pos.y)).distance(Vec2::from((x, y))) <= self.r + cushion {
			return true;
		}

		return false;
	}

	fn calculate_collision(&mut self, other: &Ball) {

		if self.paused {
			return;
		}

		let pos_diff = self.pos - other.pos;

		let normal = pos_diff.normalize();
		let tangent = Vec2::from((-normal.y, normal.x));

		let new_vel = self.v.dot(normal);
		let new_tangent = self.v.dot(tangent);
		let other_new_tangent = other.v.dot(normal);

		let mass_calc = (new_vel * (self.mass - other.mass) + 2.0 * other.mass * other_new_tangent) / (self.mass + other.mass);

		let final_vel = mass_calc * normal;
		let final_tangent = new_tangent * tangent;

		let final_v = final_vel + final_tangent;

		if (self.v - other.v).dot(self.pos - other.pos) < 0.0 {
			self.v = final_v;
		}
		
	}

	fn check_wall_collision(&mut self) {
		
		if self.pos.x <= self.r {
			self.v.x *= -1.0;
			self.pos.x = self.r + 1.0;
		}
		if self.pos.x >= screen_width() - self.r {
			self.v.x *= -1.0;
			self.pos.x = screen_width() - self.r - 1.0;
		}
		if self.pos.y <= self.r {
			self.v.y *= -1.0;
			self.pos.y = self.r + 1.0;
		}
		if self.pos.y >= screen_height() - self.r {
			self.v.y *= -1.0;
			self.pos.y = screen_height() - self.r - 1.0;
		}

	}

	fn drag(&mut self, x: f32, y: f32) {

		self.pos.x += (x - self.pos.x) / 10.0;
		self.pos.y += (y - self.pos.y) / 10.0;

	}

}

struct Rectangle {
	x: f32,
	y: f32,
	w: f32,
	h: f32,
	color: Color
}

impl Rectangle {
	fn is_in(&self, x: f32, y: f32) -> bool {
		if x > self.x && x < self.x + self.w {
			if y > self.y && y < self.y + self.h {
				return true;
			}
		}
		return false;
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

	let mut speed: f32 = 1.0;

	let open_text = measure_text(">", None, 100, 1.0);
	let mut open_button = Rectangle {
		x: 5.0,
		y: 2.0,
		w: open_text.width + 10.0,
		h: open_text.height + 10.0,
		color: Color { r: 255.0, g: 255.0, b: 255.0, a: 0.5 }
	};

	let menu_text = "Up/Down arrows for speeding up and slowing down time\nSpace to pause\nQ to generate random circles\nR to erase all\nLeft mouse on circle to drag circle\nLeft mouse and drag to shoot circle in direction\nRight mouse to delete circles";
	let mut menu_text_size = (0.0, 0.0);
	for i in menu_text.split("\n") {
		let m = measure_text(i, None, 25, 1.0);
		if m.width > menu_text_size.0 {
			menu_text_size.0 = m.width;
		}
		menu_text_size.1 += m.height * 1.5;
	}
	let mut menu_rect = Rectangle {
		x: -menu_text_size.0 - 10.0,
		y: 2.0,
		w: menu_text_size.0 + 10.0,
		h: menu_text_size.1,
		color: Color { r: 255.0, g: 255.0, b: 255.0, a: 0.5 }
	};

	let mut menu_wanted_pos = menu_rect.x;

	// let particles = Texture2D::from_image(&Image::gen_image_color(1, 1, BLACK));

	// let mut emitter = Emitter::new(EmitterConfig {
	// 	one_shot: true,
	// 	emitting: true,
	// 	lifetime: 0.2,
	// 	lifetime_randomness: 0.7,
	// 	explosiveness: 0.95,
	// 	amount: 0,
	// 	initial_direction_spread: 2.0 * std::f32::consts::PI,
	// 	initial_velocity: 100.0,
	// 	size: 1.0,
	// 	texture: Some(particles),
	// 	..Default::default()
	// });

	// let mut particles_queue: Vec<Vec2> = Vec::new();

	loop {

		clear_background(WHITE);

		let pos = mouse_position();

		open_button.x = menu_rect.w + menu_rect.x + 5.0;
		menu_rect.x += (menu_wanted_pos - menu_rect.x) * 0.12;

		if is_mouse_button_pressed(MouseButton::Left) {

			if open_button.is_in(pos.0, pos.1) {
				if menu_wanted_pos == 4.0 {
					menu_wanted_pos = -menu_text_size.0 - 10.0;
				} else {
					menu_wanted_pos = 4.0;
				}
				
			} else {
				if chosen_ball.is_none() {
					let mut selecting = false;
					for i in 0..circles.len() {
						if circles[i].overlapping(pos.0, pos.1, 7.5) {
							selecting = true;
							chosen_ball = Some(i);
							circles[i].dragging = true;
						}
					}
	
					if !selecting {
						circles.push(Ball::new(pos.0, pos.1));
						circles.last_mut().unwrap().dragging = true;
						circles.last_mut().unwrap().paused = circles[0].paused.clone();
						created_ball = Some(circles.len()-1);
					}
	
				}
			}


		}

		if is_mouse_button_down(MouseButton::Right) {
			if chosen_ball.is_none() && created_ball.is_none() {
				for i in 0..circles.len() {

					if circles[i].overlapping(pos.0, pos.1, 2.5) {
						circles.remove(i);
						break;
					}

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

			ball.v.x = -(pos.0 - ball.pos.x) * 0.01;
			ball.v.y = -(pos.1 - ball.pos.y) * 0.01;
			ball.v *= speed;
			ball.dragging = false;

			created_ball = None;
		}

		if is_key_pressed(KeyCode::Space) {
			if circles.last().is_some() {
				if circles.last().unwrap().paused {
					for i in &mut circles {
						i.paused = false;
					}
				} else {
					for i in &mut circles {
						i.paused = true;
					}
				}
			}
		}

		if is_key_pressed(KeyCode::Up) {
			speed *= 1.5;
			for i in &mut circles {
				i.v *= 1.5;
			}
		}

		if is_key_pressed(KeyCode::Down) {
			speed /= 1.5;
			for i in &mut circles {
				i.v /= 1.5;
			}
		}

		if is_key_down(KeyCode::Q) {
			circles.push(Ball::new_random(&circles));
			circles.last_mut().unwrap().v *= speed;
			circles.last_mut().unwrap().paused = circles[0].paused;
		}

		if is_key_down(KeyCode::R) {
			circles.clear();
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

				if circles[i].check_collision(&cloned_slice[j]) {

					// let radians = ((circles[i].pos.y - cloned_slice[j].pos.y) / (circles[i].pos.x - cloned_slice[j].pos.x)).atan();

					// let degrees = radians * 180.0 / std::f32::consts::PI;

					// let x_off = circles[i].r * (degrees * std::f32::consts::PI / 180.0).cos();
					// let y_off = circles[i].r * (degrees * std::f32::consts::PI / 180.0).sin();


					// println!("{}, {}, {}, {:?}, {}, {}", x_off, y_off, degrees, circles[i].color, circles[i].r, cloned_slice[j].r);

					// let final_pos = if vec2(circles[i].pos.x + x_off, circles[i].pos.y + y_off).distance(cloned_slice[j].pos) < vec2(circles[i].pos.x - x_off, circles[i].pos.y - y_off).distance(cloned_slice[j].pos) {
					// 	vec2(circles[i].pos.x + x_off, circles[i].pos.y + y_off)
					// } else {
					// 	vec2(circles[i].pos.x - x_off, circles[i].pos.y - y_off)
					// };

				}

			}
		}

		for i in &mut circles {
			i.check_wall_collision();
			i.next_frame();
			i.draw();
		}

		draw_rectangle(open_button.x, open_button.y, open_button.w, open_button.h, open_button.color);
		draw_rectangle_lines(open_button.x, open_button.y, open_button.w, open_button.h, 5.0, BLACK);

		if open_button.x > menu_rect.w/2.0 {
			draw_text("<", open_button.x+5.0, open_button.y + open_button.h, 100.0, BLACK);
		} else {
			draw_text(">", open_button.x, open_button.y + open_button.h, 100.0, BLACK);
		}
		


		draw_rectangle(menu_rect.x, menu_rect.y, menu_rect.w, menu_rect.h, menu_rect.color);
		draw_rectangle_lines(menu_rect.x, menu_rect.y, menu_rect.w, menu_rect.h, 4.0, BLACK);

		let mut last = 0.0;
		for i in menu_text.split("\n") {
			let m = measure_text(i, None, 25, 1.0);
			draw_text(i, menu_rect.x + 5.0, menu_rect.y + last + m.height, 25.0, BLACK);
			last += m.height*1.5;
		}

		next_frame().await;

	}

}
