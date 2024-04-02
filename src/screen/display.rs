use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::display_info::DisplayInfo;
use super::renderer;

pub struct Display {
    handle: Option<JoinHandle<()>>,
    _sender: Sender<DisplayInfo>,
}

pub fn create_display(window_name: &str) -> Display {
    let (sender, receiver) = channel::<DisplayInfo>();
    let window_name_clone = String::from(window_name);

    let handle = thread::spawn(move || {
        display_loop(window_name_clone.as_str(), receiver);
    });
    Display {
        handle: Some(handle),
        _sender: sender,
    }
}

fn display_loop(window_name: &str, receiver: Receiver<DisplayInfo>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(window_name, 800, 600).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let renderer = renderer::Renderer { receiver: receiver };

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        let _ = renderer.render();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

impl Display {
    pub fn join(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}
