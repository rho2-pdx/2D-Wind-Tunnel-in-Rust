mod sim;
use sim::Simulation;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const SIM_WIDTH: u32 = 256;
const SIM_HEIGHT: u32 = 256;
const PIXEL_SCALE: u32 = 4; // adjust based on your screen size
const WINDOW_WIDTH: u32 = SIM_WIDTH * PIXEL_SCALE;
const WINDOW_HEIGHT: u32 = SIM_HEIGHT * PIXEL_SCALE;

fn main() {
    
    let window_width: u32 = WINDOW_WIDTH;
    let window_height: u32 = WINDOW_HEIGHT;
    let sim_width: u32 = SIM_WIDTH;
    let sim_height: u32 = SIM_HEIGHT;
    
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(window_width as f64, window_height as f64);
        WindowBuilder::new()
            .with_title("2D Wind Tunnel Simulation")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(sim_width, sim_height, surface_texture).unwrap();
    
    let mut sim = Simulation::new(sim_width as usize, sim_height as usize);
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                sim.step();
                display_sim(&sim, pixels.frame_mut(), sim_width as usize, sim_height as usize);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let Err(e) = pixels.render() {
                    eprintln!("pixels.render() failed: {}", e);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _=>{}
        }
    });
}

fn display_sim(sim: &Simulation, frame: &mut [u8], width: usize, height: usize) {
    // First pass: compute average density over fluid cells to use as a baseline.
    let mut sum_density = 0.0;
    let mut count = 0usize;

    for y in 0..height {
        for x in 0..width {
            if sim.is_solid(x, y) {
                continue;
            }
            let d = sim.density_at(x, y);
            sum_density += d;
            count += 1;
        }
    }

    let base_density = if count > 0 {
        sum_density / (count as f64)
    } else {
        1.0
    };

    // Scale factor controls how visible the "smoke" is.
    let scale = 800.0;

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 4;

            if sim.is_solid(x, y) {
                // Draw the car/walls as dark gray.
                frame[index] = 40;
                frame[index + 1] = 40;
                frame[index + 2] = 40;
                frame[index + 3] = 0xFF;
                continue;
            }

            let density = sim.density_at(x, y);
            let excess = (density - base_density).max(0.0);
            let value = (excess * scale).clamp(0.0, 255.0) as u8;

            // Regions near baseline density stay dark; compression around the car shows as brighter "smoke".
            frame[index] = value;
            frame[index + 1] = value;
            frame[index + 2] = value;
            frame[index + 3] = 0xFF;
        }
    }
}