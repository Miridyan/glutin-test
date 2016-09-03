#![feature(dotdot_in_tuple_patterns)]
#[macro_use]
extern crate glium;

use glium::glutin::Event;
use glium::index::PrimitiveType;
use glium::glutin::VirtualKeyCode;
use glium::{Surface, DisplayBuild};

#[derive(Copy, Clone, Debug)]
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
	let t_step: f32 = 0.006;

    let vertex_shader = include_str!("../shaders/vertex.glsl");
    let fragment_shader = include_str!("../shaders/fragment.glsl");

	let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None)
		.unwrap();

	'main: loop {
		let mut target = display.draw();

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0,                   0.0                 , 0.0],
                [       0.0      ,   f,                   0.0                 , 0.0],
                [       0.0      , 0.0,     (zfar + znear) / (zfar - znear)   , 0.0],
                [       0.0      , 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };
        let matrix = [
            [t.cos(), t.sin(), 0.0, 0.0],
            [-t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

		t += t_step;
		if t >= 2.0 * std::f32::consts::PI {
			t = 0.0;
		}

		target.clear_color(1.0, 1.0, 1.0, 1.0);
		target.draw(&vert_buffer, &indices, &program, &uniform! {matrix: matrix, perspective: perspective},
            &Default::default()).unwrap();
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
