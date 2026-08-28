pub mod core;
pub mod widgets;

use crate::core::{event::{Action, MKey}, render::renderconfig::RenderBackend, size::Size};
use core::kernel::AppCore;
use widgets::frame;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct Win<T> {
    window: Option<Arc<Window>>,
    pub core: AppCore<T>,
    resizable: bool,
    title: String,
    winsize: Size,
    minsize: Option<Size>,
    maxsize: Option<Size>,
    wasfullscr: bool,
    start_num: u128,
    //last_frame_time: time::Instant,
    //needs_redraw: bool,
}

impl<T> Win<T> {
    pub fn new(init_state: T, rconf: RenderBackend) -> Self {
        Self { 
            window: None,
            core: AppCore::new(init_state, rconf),
            resizable: true,
            title: String::from("HazeGUI Window"),
            winsize: Size::new(800, 600),
            minsize: None,
            maxsize: None,
            wasfullscr: false,
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

        if let Some(min) = self.minsize {
            attributes = attributes.with_min_inner_size(winit::dpi::PhysicalSize::new(min.width as u32, min.height as u32));
        }
        if let Some(max) = self.maxsize {
            attributes = attributes.with_max_inner_size(winit::dpi::PhysicalSize::new(max.width as u32, max.height as u32));
        }

        let window = Arc::new(event_loop.create_window(attributes).expect("Failed to initialize window"));

        let phys_size = window.inner_size();
        let actual_size = Size::new(phys_size.width as i32, phys_size.height as i32);
        self.winsize = actual_size;

        self.core.init_window(window.clone(), actual_size);
        self.core.handle_resize(phys_size.width, phys_size.height);

        self.window = Some(window.clone());
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.core.clear_actions();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            
            WindowEvent::RedrawRequested => {
                self.core.draw_to_slice();
            }
            
            WindowEvent::Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    let is_maximized_or_fullscreen = self.window.as_ref().map_or(false, |w| {
                        w.is_maximized() || w.fullscreen().is_some()
                    });

                    let state_changed = self.wasfullscr != is_maximized_or_fullscreen;

                    self.winsize = Size::new(new_size.width as i32, new_size.height as i32);
                    self.core.handle_resize(new_size.width, new_size.height);
                    
                    if state_changed {
                        self.core.handle_resize(new_size.width, new_size.height);
                    }

                    self.wasfullscr = is_maximized_or_fullscreen;
                    
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            
            WindowEvent::MouseInput { state, button, .. } => {
                let key = match button {
                    winit::event::MouseButton::Left => {MKey::Left}
                    winit::event::MouseButton::Middle => {MKey::Middle}
                    winit::event::MouseButton::Right => {MKey::Right}
                    winit::event::MouseButton::Back => {MKey::Back}
                    winit::event::MouseButton::Forward => {MKey::Forward}
                    winit::event::MouseButton::Other(_) => {MKey::None}
                };
                self.core.handle_mouse_click(state == winit::event::ElementState::Pressed, key);
            }
            
            WindowEvent::CursorMoved { position, .. } => {
                self.core.handle_mouse_move(position.x as i32, position.y as i32);
            }
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                let is_pressed = event.state == winit::event::ElementState::Pressed;
                
                let mut ch = '\0';
                if let Some(text) = &event.text {
                    if let Some(c) = text.chars().next() {
                        ch = c;
                    }
                }

                use winit::keyboard::{Key, NamedKey};
                let key = match &event.logical_key {
                    Key::Named(NamedKey::Backspace) => crate::core::event::KKey::Backspace,
                    Key::Named(NamedKey::Enter)     => crate::core::event::KKey::Enter,
                    Key::Named(NamedKey::Space)     => crate::core::event::KKey::Space,
                    Key::Named(NamedKey::ArrowUp)   => crate::core::event::KKey::ArrowUp,
                    Key::Named(NamedKey::ArrowDown) => crate::core::event::KKey::ArrowDown,
                    Key::Named(NamedKey::ArrowLeft) => crate::core::event::KKey::ArrowLeft,
                    Key::Named(NamedKey::ArrowRight)=> crate::core::event::KKey::ArrowRight,
                    Key::Named(NamedKey::Delete)    => crate::core::event::KKey::Delete,
                    Key::Named(NamedKey::Home)      => crate::core::event::KKey::Home,
                    Key::Named(NamedKey::End)       => crate::core::event::KKey::End,
                    Key::Named(NamedKey::PageUp)    => crate::core::event::KKey::PageUp,
                    Key::Named(NamedKey::PageDown)  => crate::core::event::KKey::PageDown,
                    Key::Named(NamedKey::Tab)       => crate::core::event::KKey::Tab,
                    Key::Named(NamedKey::Escape)    => crate::core::event::KKey::Escape,
                    Key::Named(NamedKey::Insert)    => crate::core::event::KKey::Insert,
                    Key::Named(NamedKey::PrintScreen) => crate::core::event::KKey::PrintScr,
                    Key::Named(NamedKey::Shift) => crate::core::event::KKey::Shift,
                    Key::Named(NamedKey::Control) => crate::core::event::KKey::Ctrl,
                    Key::Named(NamedKey::Alt) => crate::core::event::KKey::Alt,
                    Key::Named(NamedKey::Super) | Key::Named(NamedKey::Meta) => crate::core::event::KKey::Super,
                    Key::Named(NamedKey::Fn) => crate::core::event::KKey::Fn,
                    Key::Named(NamedKey::CapsLock)    => crate::core::event::KKey::CapsLock,
                    Key::Named(NamedKey::ContextMenu) => crate::core::event::KKey::ContextMenu,
                    
                    Key::Named(NamedKey::F1)  => crate::core::event::KKey::F1,
                    Key::Named(NamedKey::F2)  => crate::core::event::KKey::F2,
                    Key::Named(NamedKey::F3)  => crate::core::event::KKey::F3,
                    Key::Named(NamedKey::F4)  => crate::core::event::KKey::F4,
                    Key::Named(NamedKey::F5)  => crate::core::event::KKey::F5,
                    Key::Named(NamedKey::F6)  => crate::core::event::KKey::F6,
                    Key::Named(NamedKey::F7)  => crate::core::event::KKey::F7,
                    Key::Named(NamedKey::F8)  => crate::core::event::KKey::F8,
                    Key::Named(NamedKey::F9)  => crate::core::event::KKey::F9,
                    Key::Named(NamedKey::F10) => crate::core::event::KKey::F10,
                    Key::Named(NamedKey::F11) => crate::core::event::KKey::F11,
                    Key::Named(NamedKey::F12) => crate::core::event::KKey::F12,
                    
                    Key::Character(_) => crate::core::event::KKey::None,

                    _ => {
                        //println!("Unknown key: {:?}", event.logical_key);
                        crate::core::event::KKey::None
                    }
                };

                self.core.handle_keyboard_event(is_pressed, key, ch);
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
