pub mod core;
pub mod widgets;

use crate::core::{event::Action, renderconfig::RenderConfig, size::Size};
use core::kernel::AppCore;
use widgets::frame;

use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct Win<T> {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    pub core: AppCore<T>,
    resizable: bool,
    title: String,
    winsize: Size,
    minsize: Option<Size>,
    maxsize: Option<Size>,
    start_num: u128,
    //last_frame_time: time::Instant,
    //needs_redraw: bool,
}

impl<T> Win<T> {
    pub fn new(init_state: T, rconf: RenderConfig) -> Self {
        Self { 
            window: None,
            surface: None,
            core: AppCore::new(init_state, rconf),
            resizable: true,
            title: String::from("HazeGUI Window"),
            winsize: Size::new(800, 600),
            minsize: None,
            maxsize: None,
            start_num: 0,
            //last_frame_time: time::Instant::now(),
            //needs_redraw: false,
        }
    }

    pub fn title(&mut self, newtitle: &str) { self.title = newtitle.to_string(); }
    pub fn min_size(&mut self, size: Size) { self.minsize = Some(size); }
    pub fn max_size(&mut self, size: Size) { self.maxsize = Some(size); }
    pub fn resizable(&mut self, state: bool) { self.resizable = state; }
    //pub fn set_fps(&mut self, new_fps: u32) { self.core.renderconf.fps = new_fps; }

    pub fn geometry(&mut self, newsize: Size) {
        self.winsize = newsize;
        self.core.mainframe.base.size = self.winsize;
        if let Some(window) = &self.window {
            let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(newsize.width, newsize.height));
        }
    }

    pub fn mainloop<F>(&mut self, cb: F)
    where
        F: FnMut(&Action, &mut frame::Frame, &mut T) + 'static,
    {
        self.core.user_cb = Some(Box::new(cb));
        let event_loop = EventLoop::new().expect("Failed to create mainloop");
        let _ = event_loop.run_app(self);
    }
}

impl<T> ApplicationHandler for Win<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes()
            .with_title(&self.title)
            .with_resizable(self.resizable)
            .with_inner_size(winit::dpi::PhysicalSize::new(self.winsize.width as u32, self.winsize.height as u32));

        if let Some(min) = self.minsize { attributes = attributes.with_min_inner_size(winit::dpi::PhysicalSize::new(min.width as u32, min.height as u32)); }
        if let Some(max) = self.maxsize { attributes = attributes.with_max_inner_size(winit::dpi::PhysicalSize::new(max.width as u32, max.height as u32)); }

        let window = Arc::new(event_loop.create_window(attributes).expect("Failed to initialize window"));
        let context = Context::new(window.clone()).expect("Failed to initialize window context");
        let surface = Surface::new(&context, window.clone()).expect("Failed to initialize window surface");

        self.core.handle_resize(self.winsize.width as u32, self.winsize.height as u32);

        self.window = Some(window);
        self.surface = Some(surface);

        if let Some(window) = &self.window { window.request_redraw(); }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.core.clear_actions();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface {
                    if let Ok(mut buffer) = surface.buffer_mut() {
                        self.core.draw_to_slice(&mut buffer);
                        let _ = buffer.present();
                    }
                }
                //self.needs_redraw = false;
            }
            
            WindowEvent::Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    self.winsize = Size::new(new_size.width as i32, new_size.height as i32);
                    self.core.handle_resize(new_size.width, new_size.height);

                    if let Some(surface) = &mut self.surface {
                        if let (Some(w), Some(h)) = (NonZeroU32::new(new_size.width), NonZeroU32::new(new_size.height)) {
                            let _ = surface.resize(w, h);
                        }
                    }
                }
            }
            
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left {
                    self.core.handle_mouse_click(state == winit::event::ElementState::Pressed);
                }
            }
            
            WindowEvent::CursorMoved { position, .. } => {
                self.core.handle_mouse_move(position.x as i32, position.y as i32);
            }
            _ => (),
        }

        if self.core.post_process_events() {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        //if self.core.post_process_events() {
        //    self.needs_redraw = true;
        //}
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        //let now = std::time::Instant::now();

        if self.start_num < 3 {
            if let Some(window) = &self.window {
                self.core.dirty_rect = Some(None);
                window.request_redraw();
                self.start_num += 1;
                //self.last_frame_time = now;
                //self.needs_redraw = false;
            }
            return;
        }

        //let frame_duration = std::time::Duration::from_secs_f32(1.0 / self.core.renderconf.fps as f32);

        //if self.needs_redraw && now.duration_since(self.last_frame_time) >= frame_duration {
        //    if let Some(window) = &self.window {
        //        window.request_redraw();
        //        self.last_frame_time = now;
        //        self.needs_redraw = false;
        //    }
        //}

        //let next_frame_time = self.last_frame_time + frame_duration;
        
        //event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame_time));
    }
}
