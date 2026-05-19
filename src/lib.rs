pub mod core;
pub mod widgets;

use core::color::Color;
use core::widget::Widget;
use core::common::merge_rects;
use crate::core::{common::RenderStrategy, event::Action, pos::Pos, size::Size};
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
    start_num: i32,
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    backbuffer: Option<tiny_skia::Pixmap>,
    renderstrat: RenderStrategy,
    title: String,
    pub mainframe: frame::Frame,
    winsize: Size,
    minsize: Option<Size>,
    maxsize: Option<Size>,
    resizable: bool,
    mouse_pos: Pos,
    pub state: T,
    dirty_rect: Option<Option<tiny_skia::Rect>>,
    user_cb: Option<UserCallback<T>>,
    debug_thing: u32,
}

impl<T> Win<T> {
    pub fn new(init_state: T, renderstrat_given: RenderStrategy) -> Self {
        let mut mainframe_setter = frame::Frame::new("mainframe".to_string()).pos(Pos::new(0, 0)).size(Size::new(800, 600)).color(Color::LIGHT_GRAY);
        mainframe_setter.set_relayout_flag(true);
        Self { 
            start_num: 0,
            title: String::from("HazeGUI window"), 
            window: None, 
            surface: None,
            backbuffer: None,
            renderstrat: renderstrat_given,
            mainframe: mainframe_setter,
            winsize: Size::new(800, 600),
            minsize: None,
            maxsize: None,
            resizable: true,
            mouse_pos: Pos::new(0, 0),
            state: init_state,
            dirty_rect: Some(None),
            user_cb: None,
            debug_thing: 0,
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
        if self.renderstrat == RenderStrategy::CpuOptimized{
            self.backbuffer = tiny_skia::Pixmap::new(self.winsize.width as u32, self.winsize.height as u32);
        }

        self.window = Some(window);
        self.surface = Some(surface);

        self.mainframe.update_layout(true);

        self.dirty_rect = Some(None);
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let mut actions = Vec::new();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.debug_thing += 1;
                println!("Redrawing {}", self.debug_thing);
                if let Some(surface) = &mut self.surface {
                    let mut buffer = surface.buffer_mut().unwrap();
                    match self.renderstrat {
                        RenderStrategy::CpuOptimized => {
                            if let (Some(backbuffer), Some(dirty_type)) = (&mut self.backbuffer, self.dirty_rect.take()) {
                                let clip_rect = match dirty_type {
                                    Some(rect) => rect,
                                    None => tiny_skia::Rect::from_xywh(0.0, 0.0, self.winsize.width as f32, self.winsize.height as f32).unwrap()
                                };

                                println!("{:?}", clip_rect);

                                self.mainframe.draw(&mut backbuffer.as_mut(), Pos::new(0, 0), clip_rect, None);

                                buffer.copy_from_slice(bytemuck::cast_slice(backbuffer.data()));
                            }
                        }
                        RenderStrategy::RamOptimized => {
                            self.dirty_rect = None;
                            self.backbuffer = None;
                            let full_rect = tiny_skia::Rect::from_xywh(0.0, 0.0, self.winsize.width as f32, self.winsize.height as f32).unwrap();

                            let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                                bytemuck::cast_slice_mut(&mut buffer),
                                self.winsize.width as u32,
                                self.winsize.height as u32
                            ).unwrap();

                            self.mainframe.draw(&mut pixmap, Pos::new(0, 0), full_rect, None);
                        }
                    }
                    buffer.present().unwrap();
                }
                self.mainframe.set_dirty_flag(false);
                self.dirty_rect = None;
            }
            WindowEvent::Resized(new_size) => {
                self.winsize = Size::new(new_size.width as i32, new_size.height as i32);
                self.mainframe.base.size = self.winsize;
                self.mainframe.update_layout(true);

                if new_size.width > 0 && new_size.height > 0 {
                    self.backbuffer = None;

                    if self.renderstrat == RenderStrategy::CpuOptimized { self.backbuffer = tiny_skia::Pixmap::new(new_size.width, new_size.height); }
                    
                    if let Some(surface) = &mut self.surface {
                        surface.resize(
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap()
                        ).unwrap();
                    }
                }
                
                self.dirty_rect = Some(None);
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left && state == winit::event::ElementState::Pressed {
                    let click_event = crate::core::event::Event::MouseClick { pos: self.mouse_pos };
                    
                    self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut actions);
                } else if button == winit::event::MouseButton::Left && state == winit::event::ElementState::Released {
                    let click_event = crate::core::event::Event::MouseRelease { pos: self.mouse_pos };
                    
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
            if let Some(mut cb) = self.user_cb.take() {
                let current_actions = actions.clone(); 
                for action in &current_actions {
                    cb(action, &mut self.mainframe, &mut self.state);
                }
                self.user_cb = Some(cb);
            }
        }

        let mut redraw_actions = Vec::new();
        self.mainframe.get_dirty_rect(Pos::new(0, 0), &mut redraw_actions);

        if !redraw_actions.is_empty() {
            for action in &redraw_actions {
                if let Action::RedrawRequest(maybe_rect) = action {
                    match (self.dirty_rect, maybe_rect) {
                        (Some(None), _) => {},
                        (_, None) => self.dirty_rect = Some(None),
                        (None, Some(rect)) => self.dirty_rect = Some(Some(*rect)),
                        (Some(Some(current_rect)), Some(new_rect)) => {
                            self.dirty_rect = Some(Some(merge_rects(current_rect, *new_rect)));
                        }
                    }
                }
            }
        }

        let needs_layout = self.mainframe.needs_relayout() || actions.iter().any(|a| matches!(a, Action::UpdateLayoutRequest));

        if needs_layout {
            println!("Relayout");
            self.mainframe.update_layout(true);
            self.mainframe.set_relayout_flag(false);
        }

        if self.dirty_rect.is_some() || needs_layout {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.start_num < 5 {
            if let Some(window) = &self.window {
                self.dirty_rect = Some(None);
                window.request_redraw();
                self.start_num += 1;
            }
        }
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
