pub mod core;
pub mod widgets;

use core::color::Color;
use core::widget::Widget;
use crate::core::{pos::Pos, size::Size, event::Action};

use tiny_skia::Rect;
use widgets::frame;

use winit::{
    event::WindowEvent,
    event_loop::{EventLoop,ActiveEventLoop},
    window::{Window, WindowId},
    application::ApplicationHandler,
};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;

type UserCallback<T> = Box<dyn FnMut(&Action, &mut frame::Frame, &mut T)>;

pub struct Win<T> {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    title: String,
    pub mainframe: frame::Frame,
    winsize: Size,
    minsize: Option<Size>,
    maxsize: Option<Size>,
    resizable: bool,
    mouse_pos: Pos,
    pub state: T,
    user_cb: Option<UserCallback<T>>,
}

impl<T> Win<T> {
    pub fn new(init_state: T) -> Self {
        Self { 
            title: String::from("HazeGUI window"), 
            window: None, 
            surface: None,
            mainframe: frame::Frame::new("mainframe".to_string()).pos(Pos::new(0, 0)).size(Size::new(800, 600)).color(Color::LIGHT_GRAY),
            winsize: Size::new(800, 600),
            minsize: None,
            maxsize: None,
            resizable: true,
            mouse_pos: Pos::new(0, 0),
            state: init_state,
            user_cb: None,
        }
    }

    pub fn title(&mut self, newtitle: &str) {
        self.title = newtitle.to_string();
    }

    pub fn geometry(&mut self, newsize: Size) {
        self.winsize = Size::new(newsize.width as i32,newsize.height as i32);
        self.mainframe.base.size = self.winsize;
           
        if let Some(window) = &self.window {
            let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(newsize.width, newsize.height));
        }

    }

    pub fn min_size(&mut self, size: Size) {
        self.minsize = Some(size);
    }

    pub fn max_size(&mut self, size: Size) {
        self.maxsize = Some(size);
    }

    pub fn resizable(&mut self, state: bool) {
        self.resizable = state;
    }

    pub fn mainloop<F>(&mut self, cb: F)
    where
        F: FnMut(&Action, &mut frame::Frame, &mut T) + 'static,
    {
        self.user_cb = Some(Box::new(cb));
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
            attributes = attributes.with_min_inner_size(winit::dpi::PhysicalSize::new(
                min.width as u32, 
                min.height as u32
            ));
        }

        if let Some(max) = self.maxsize {
            attributes = attributes.with_max_inner_size(winit::dpi::PhysicalSize::new(
                max.width as u32, 
                max.height as u32
            ));
        }

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let mut actions = Vec::new();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
                    let size = window.inner_size();
                    
                    if let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                        surface.resize(w, h).unwrap();
                        
                        let mut buffer = surface.buffer_mut().unwrap();

                        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                            bytemuck::cast_slice_mut(&mut buffer),
                            size.width,
                            size.height,
                        ).unwrap();

                        let window_rect = Rect::from_xywh(
                            0.0, 
                            0.0, 
                            self.mainframe.base.size.width as f32, 
                            self.mainframe.base.size.height as f32
                        ).unwrap();

                        self.mainframe.draw(&mut pixmap, Pos::new(0, 0), window_rect);

                        buffer.present().unwrap();
                    }
                }
            }
            WindowEvent::Resized(new_size) => {
                self.winsize = Size::new(new_size.width as i32, new_size.height as i32);
                self.mainframe.base.size = self.winsize;

                self.mainframe.update_layout();
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left && state == winit::event::ElementState::Pressed {
                    let click_event = crate::core::event::Event::MouseClick { pos: self.mouse_pos };
                    
                    self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut actions);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Pos::new(position.x as i32, position.y as i32);
                let move_event = crate::core::event::Event::MouseMove { pos: self.mouse_pos };
                
                self.mainframe.handle_event(&move_event, Pos::new(0, 0), &mut actions);
            }
            _ => (),
        }

        if !actions.is_empty() {
            let mut onlynone = true;
            if let Some(mut cb) = self.user_cb.take() {
                for action in &actions {
                    if *action != Action::None {
                        onlynone = false;
                    }
                    cb(action, &mut self.mainframe, &mut self.state);
                }
                self.user_cb = Some(cb);
            }

            let needs_layout = actions.iter().any(|a| matches!(a, Action::UpdateLayoutRequest));

            if needs_layout {
                self.mainframe.update_layout();
            }

            self.mainframe.update_layout();
            if let Some(window) = &self.window {
                if !onlynone {
                    window.request_redraw();
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        //if let Some(window) = &self.window {
        //    window.request_redraw();
        //}
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
