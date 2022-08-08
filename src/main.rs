use std::time::Instant;

use glium::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::{Fullscreen, WindowBuilder};
use glium::glutin::ContextBuilder;
use glium::index::PrimitiveType;
use glium::{
    draw_parameters, Depth, Display, DrawParameters, IndexBuffer, Program, Surface,
    VertexBuffer,
};
use lava::camera;
use lava::floor;
use lava::shaders;
use lava::teapot;

use glium::{self, uniform};

fn main() {
    let light_dir = (0.0, -1.0, 0.0f32);
    let camera_fov = 45.0;
    let fps = 60;
    let mut camera = camera::Camera::new();

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Teapot")
        .with_fullscreen(Some(Fullscreen::Borderless(None)));
    let context_builder = ContextBuilder::new().with_depth_buffer(24);
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();
    display.gl_window().window().set_cursor_visible(false);

    let teapot_shader_program = Program::from_source(
        &display,
        shaders::TEAPOT_VERTEX_SHADER_SRC,
        shaders::TEAPOT_FRAGMENT_SHADER_SRC,
        None,
    )
    .unwrap();
    let teapot_positions = VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let teapot_normals = VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let teapot_indices =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let floor_shader_program = Program::from_source(
        &display,
        shaders::FLOOR_VERTEX_SHADER_SRC,
        shaders::FLOOR_FRAGMENT_SHADER_SRC,
        None,
    )
    .unwrap();
    let floor_vertices = VertexBuffer::new(&display, &floor::FLOOR_VERTICES).unwrap();
    let floor_indices = IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &floor::FLOOR_INDICES,
    )
    .unwrap();

    let mut prev_frame_t = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        _ => (),
                    }
                }
                _ => return,
            },
            Event::DeviceEvent { event, .. } => camera.process_device_event(&event),
            _ => (),
        }

        let curr_frame_t = Instant::now();
        let frame_dt = (curr_frame_t - prev_frame_t).as_nanos();
        if frame_dt >= 1_000_000_000 / fps {
            camera.update(frame_dt as f32 / 1_000_000_000f32);
            let mut frame = display.draw();
            frame.clear_color_and_depth((0.1, 0.1, 0.5, 1.0), 1.0);

            let (screen_height, screen_width) = display.get_framebuffer_dimensions();

            let params = DrawParameters {
                depth: Depth {
                    test: draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            frame
                .draw(
                    (&teapot_positions, &teapot_normals),
                    &teapot_indices,
                    &teapot_shader_program,
                    &uniform! {
                        model: [
                            [0.002, 0.0, 0.0, 0.0],
                            [0.0, 0.002, 0.0, 0.0],
                            [0.0, 0.0, 0.002, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ],
                        view: camera.get_view_mat(),
                        perspective: camera::get_perspective(camera_fov, screen_width, screen_height),
                        light_dir: light_dir,

                    },
                    &params,
                )
                .unwrap();

            frame
                .draw(
                    &floor_vertices,
                    &floor_indices,
                    &floor_shader_program,
                    &uniform! {
                        model: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, -0.08, 0.0, 1.0f32],
                        ],
                        view: camera.get_view_mat(),
                        perspective: camera::get_perspective(camera_fov, screen_width, screen_height),
                        light_dir: light_dir,

                    },
                    &params,
                )
                .unwrap();

            frame.finish().unwrap();
            prev_frame_t = Instant::now();
        }
    });
}
