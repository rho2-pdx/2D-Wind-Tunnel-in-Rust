mod sim;
use sim::Simulation;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
const SIM_WIDTH: u32 = 128;
const SIM_HEIGHT: u32 = 128;
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
    let mut min_density = f64::MAX;
    let mut max_density = f64::MIN;
    
    for y in 0..height {
        for x in 0..width {
            let d = sim.density_at(x, y);
            if d < min_density { min_density = d; }
            if d > max_density { max_density = d; }
        }
    }
    
    let range = (max_density - min_density).max(1e-6); // prevent division by zero
    
    for y in 0..height {
        for x in 0..width {
            let density = sim.density_at(x, y);
            let t = ((density - min_density) / range).clamp(0.0, 1.0);
            let value = (t * 256.0) as u8;
            
            let index = (y * width + x) * 4;
            frame[index] = value;
            frame[index + 1] = value;
            frame[index + 2] = value;   
            frame[index + 3] = value;
        }
    }
}