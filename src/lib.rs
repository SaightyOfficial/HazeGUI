pub mod core;
pub mod widgets;

use crate::core::{common::RenderStrategy, event::Action, pos::Pos, size::Size};
use core::color::Color;
use core::common::merge_rects;
use core::widget::Widget;
use widgets::frame;

use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use tiny_skia::{Pixmap, PixmapMut, Rect};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

type UserCallback<T> = Box<dyn FnMut(&Action, &mut frame::Frame, &mut T)>; //Making shortcut to not write that shi again

///Window struct that stores everything that HazeGUI window needs
pub struct Win<T> {
    start_num: u128, //That is needed to fix black screen bug when using CpuOptimized strategy
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    backbuffer: Option<Pixmap>,
    renderstrat: RenderStrategy,
    title: String,
    pub mainframe: frame::Frame,
    winsize: Size,
    minsize: Option<Size>,
    maxsize: Option<Size>,
    resizable: bool,
    mouse_pos: Pos,
    pub state: T,
    dirty_rect: Option<Option<Rect>>,
    user_cb: Option<UserCallback<T>>,
    debug_thing: u32, //for debug purposes such as redraw number and etc
    redraw_actions: Vec<Action>,
    actions: Vec<Action>,
}

impl<T> Win<T> {
    /// Function that creates a new window
    /// 
    /// Takes struct with data what window will easily able to operate and [`RenderStrategy`] by which rendering and memory will be optimized in some way
    pub fn new(init_state: T, renderstrat_given: RenderStrategy) -> Self {
        let mut mainframe_setter = frame::Frame::new("mainframe".into())
            .pos(Pos::new(0, 0))
            .size(Size::new(800, 600))
            .color(Color::LIGHT_GRAY);
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
            actions: Vec::with_capacity(16),
            redraw_actions: Vec::with_capacity(16),
        }
    }

    ///Setting up window title
    pub fn title(&mut self, newtitle: &str) {
        self.title = newtitle.to_string();
    }

    ///Setting up window size
    pub fn geometry(&mut self, newsize: Size) {
        self.winsize = Size::new(newsize.width as i32, newsize.height as i32);
        self.mainframe.base.size = self.winsize;

        if let Some(window) = &self.window {
            let _ = window
                .request_inner_size(winit::dpi::PhysicalSize::new(newsize.width, newsize.height));
        }
    }

    ///Minimal window size
    pub fn min_size(&mut self, size: Size) {
        self.minsize = Some(size);
    }

    ///Maximal window size
    pub fn max_size(&mut self, size: Size) {
        self.maxsize = Some(size);
    }

    ///Can window be resized?
    pub fn resizable(&mut self, state: bool) {
        self.resizable = state;
    }

    ///Window mainloop
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
    //Function that is called at first launch
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes()
            .with_title(&self.title)
            .with_resizable(self.resizable)
            .with_inner_size(winit::dpi::PhysicalSize::new(
                self.winsize.width as u32,
                self.winsize.height as u32,
            )); //Setting up window attributes

        //Checking for minimal size
        if let Some(min) = self.minsize {
            attributes = attributes.with_min_inner_size(winit::dpi::PhysicalSize::new(
                min.width as u32,
                min.height as u32,
            ));
        }

        //Checking for maximal size
        if let Some(max) = self.maxsize {
            attributes = attributes.with_max_inner_size(winit::dpi::PhysicalSize::new(
                max.width as u32,
                max.height as u32,
            ));
        }

        //Window creation with setted attributes
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        //Creation of window context, surface and backbuffer if using CpuOptimized strategy
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        if self.renderstrat == RenderStrategy::CpuOptimized {
            self.backbuffer = Pixmap::new(self.winsize.width as u32, self.winsize.height as u32);
        }

        //Moving window and surface to Win struct
        self.window = Some(window);
        self.surface = Some(surface);

        //Forced mainframe layout update
        self.mainframe.update_layout(true);

        //Full window redraw
        self.dirty_rect = Some(None); //Requesting full redraw by saying that there is an dirty rect but its a whole window
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    //Fucntion that is called to handle events such as mouse clicks/moves and key presses
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        //Clearing actions lists
        self.actions.clear();
        self.redraw_actions.clear();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(), //If close request -> exit
            WindowEvent::RedrawRequested => {
                self.debug_thing += 1; //Changing debug redraw counter
                println!("Redrawing {}", self.debug_thing); //This is for debug
                //Checking for window surface (There are lots of unsafe unwraps I will wix that later, sorry guys)
                if let Some(surface) = &mut self.surface {
                    let mut buffer = surface.buffer_mut().unwrap(); //Initializing mutable framebuffer
                    match self.renderstrat {
                        RenderStrategy::CpuOptimized => {
                            //Checking for backbuffer and dirty rect
                            if let (Some(backbuffer), Some(dirty_type)) =
                                (&mut self.backbuffer, self.dirty_rect.take()) {
                                
                                let clip_rect = match dirty_type {
                                    Some(rect) => rect, //If there is dirty rect, use it
                                    None => Rect::from_xywh(
                                        0.0,
                                        0.0,
                                        self.winsize.width as f32,
                                        self.winsize.height as f32,
                                    ).unwrap(), //If there is no dirty rect then redraw whole window
                                };

                                println!("{:?}", clip_rect); //Debug again

                                self.mainframe.draw(
                                    &mut backbuffer.as_mut(),
                                    Pos::new(0, 0),
                                    clip_rect,
                                    None,
                                );//Drawing widget tree

                                buffer.copy_from_slice(bytemuck::cast_slice(backbuffer.data()));//Putting data fram backbuffer to window
                            }
                        }
                        RenderStrategy::RamOptimized => {
                            //If ram usage(which is already small) should be optimized, then there should be no backbuffer just redraw whole window
                            self.dirty_rect = None;
                            self.backbuffer = None;
                            let full_rect = Rect::from_xywh(
                                0.0,
                                0.0,
                                self.winsize.width as f32,
                                self.winsize.height as f32,
                            ).unwrap();//Window rect

                            let mut pixmap = PixmapMut::from_bytes(
                                bytemuck::cast_slice_mut(&mut buffer),
                                self.winsize.width as u32,
                                self.winsize.height as u32,
                            ).unwrap();//Window pixmap

                            self.mainframe.draw(&mut pixmap, Pos::new(0, 0), full_rect, None);
                        }
                    }
                    buffer.present().unwrap();//Idk i forgot just dont touch that
                }
                self.mainframe.set_dirty_flag(false); //Setting all dirtiness to zero cuz we already did renreding
                self.dirty_rect = None; //Setting dirty rect to none
            }
            WindowEvent::Resized(new_size) => {
                self.winsize = Size::new(new_size.width as i32, new_size.height as i32);//Window mainframe size
                self.mainframe.base.size = self.winsize;//Changing mainframe size
                self.mainframe.update_layout(true);//Forced layput update

                if new_size.width > 0 && new_size.height > 0 {//If not zero
                    self.backbuffer = None;//Clearing backbuffer

                    //Creating backbuffer if we are optimizing cpu
                    if self.renderstrat == RenderStrategy::CpuOptimized {
                        self.backbuffer = Pixmap::new(new_size.width, new_size.height);
                    }

                    //Checking for window surface
                    if let Some(surface) = &mut self.surface {
                        //Resizing it
                        surface
                            .resize(
                                NonZeroU32::new(new_size.width).unwrap(),
                                NonZeroU32::new(new_size.height).unwrap(),
                            ).unwrap();
                    }
                }

                self.dirty_rect = Some(None);//Pls redraw whole window pls pls

                if let Some(window) = &self.window {
                    window.request_redraw();//Requesting redraw
                }
            }
            //Mouse input handler
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left //Checking button
                    && state == winit::event::ElementState::Pressed //Checking if pressed
                {
                    let click_event = crate::core::event::Event::MouseClick {
                        pos: self.mouse_pos,
                    };//Generating event

                    //Handling event
                    self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut self.actions);
                } else if button == winit::event::MouseButton::Left //Checking button
                    && state == winit::event::ElementState::Released //Checking if released
                {
                    let click_event = crate::core::event::Event::MouseRelease {
                        pos: self.mouse_pos,
                    };//Generating event(again)

                    //Handling event
                    self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut self.actions);
                }
            }
            //Mouse move handler
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = Pos::new(position.x as i32, position.y as i32); //Setting mouse pos
                let move_event = crate::core::event::Event::MouseMove {
                    pos: self.mouse_pos,
                };//Generating event

                //You probably know that we are goint to handle events
                self.mainframe.handle_event(&move_event, Pos::new(0, 0), &mut self.actions);
            }
            _ => (),
        }

        if !self.actions.is_empty() {
            if let Some(mut cb) = self.user_cb.take() {
                //If actions are not empty we are putting them to our state
                for action in &self.actions {
                    cb(action, &mut self.mainframe, &mut self.state);
                }
                self.user_cb = Some(cb);
            }
        }

        //Getting all dirty rects
        self.mainframe.get_dirty_rect(Pos::new(0, 0), &mut self.redraw_actions);

        //If redraw actions are not empty then
        if !self.redraw_actions.is_empty() {
            for action in &self.redraw_actions {
                if let Action::RedrawRequest(maybe_rect) = action {
                    match (self.dirty_rect, maybe_rect) {
                        (Some(None), _) => {}//If full window then ignore anything else
                        (_, None) => self.dirty_rect = Some(None), //If we want to request full window then do full window
                        (None, Some(rect)) => self.dirty_rect = Some(Some(*rect)), //If dirty rect is nothing and we have something then just put that rect
                        (Some(Some(current_rect)), Some(new_rect)) => { //If we have 2 dirte rects or more then just merge them into a bigger one
                            self.dirty_rect = match merge_rects(current_rect, *new_rect) {
                                Ok(merged) => Some(Some(merged)),
                                Err(_) => Some(None),
                            };//Yay slides!
                        }
                    }
                }//Still going!
            }
        }
        //That was good =D

        //Checking if widget tree needs relayout
        let needs_layout = self.mainframe.needs_relayout()
            || self.actions
                .iter()
                .any(|a| matches!(a, Action::UpdateLayoutRequest));

        //If it does then relayout
        if needs_layout {
            println!("Relayout");//Debug
            self.mainframe.update_layout(true);//Forced relayout
            self.mainframe.set_relayout_flag(false);//Setting relayout flag to false
        }

        //If we have firty rects or we need relayout then requesting redraw
        if self.dirty_rect.is_some() || needs_layout {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.start_num < 3 { //Black screen fix
            if let Some(window) = &self.window {
                self.dirty_rect = Some(None);
                window.request_redraw();
                self.start_num += 1;
            }
        }
    }
}

//Idk something
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
