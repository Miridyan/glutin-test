#![feature(dotdot_in_tuple_patterns)]
#[macro_use]
extern crate glium;

use glium::glutin::Event;
use glium::index::PrimitiveType;
use glium::glutin::VirtualKeyCode;
use glium::{Surface, DisplayBuild};

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}
impl Vertex {
	fn new(x: f32, y: f32) -> Vertex {
		Vertex {
			position: [x, y],
		}
	}
}
implement_vertex!(Vertex, position);

fn main() {
	let display = glium::glutin::WindowBuilder::new()
		.with_dimensions(640, 640)
		.with_title(format!("Hello This is Window"))
		.build_glium()
		.unwrap();

	let vertex1 = Vertex::new(-0.5, 0.5);
	let vertex2 = Vertex::new(0.5, 0.5);
	let vertex3 = Vertex::new(0.5, -0.5);
	let vertex4 = Vertex::new(-0.5, -0.5);
	let square = vec![vertex1, vertex2, vertex3, vertex4];

	let index_list: [u16; 6] = [
		0, 1, 2,
		0, 2, 3,
	];

	let vert_buffer = glium::VertexBuffer::new(&display, &square).unwrap();
	let indices = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &index_list)
		.unwrap();

	let mut t: f32 = 0.0;
	let t_step: f32 = 0.0006;

	let vertex_shader = r#"
		#version 140
		in vec2 position;
		out vec4 my_attr;

		uniform mat4 matrix;

		void main() {
			my_attr = matrix * vec4(position, 0.0, 1.0);
			gl_Position = my_attr;
		}
	"#;
	let fragment_shader = r#"
		#version 140
		in vec4 my_attr;
		out vec4 color;

		void main() {
			color = my_attr;
		}
	"#;

	let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None)
		.unwrap();

	'main: loop {
		let mut target = display.draw();

		t += t_step;
		if t >= 2.0 * std::f32::consts::PI {
			t = 0.0;
		}

		let uniforms = uniform! {
			matrix: [
				[t.cos(), t.sin(), 0.0, 0.0],
				[-t.sin(), t.cos(), 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0f32],
			]
		};

		target.clear_color(1.0, 1.0, 1.0, 1.0);
		target.draw(&vert_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
		target.finish().unwrap();

		for events in display.poll_events() {
			match events {
				Event::Closed |
				Event::KeyboardInput(.., Some(VirtualKeyCode::Escape)) => {
					break 'main;
				},
				_ => (),
			}
		}
	}
}
