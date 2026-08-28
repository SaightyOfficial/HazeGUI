use crate::core;
use crate::core::common::SizeEnum;
use crate::core::event::{DrawCommand, KKey, MKey};
use crate::core::render::rendercommands::Renderer;
use crate::core::render::renderconfig::RenderBackend::{self};
use crate::core::{event::Action, pos::Pos, size::Size, common::merge_rects};
use core::color::Color;
use core::shapes::Rect;
use core::widget::Widget;
use std::sync::Arc;
use crate::widgets::frame;
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};

type UserCallback<T> = Box<dyn FnMut(&Action, &mut frame::Frame, &mut T)>;

pub trait GuiWindow: HasWindowHandle + HasDisplayHandle + Send + Sync {}
impl<T: HasWindowHandle + HasDisplayHandle + Send + Sync> GuiWindow for T {}

pub struct AppCore<T> {
    pub render: RenderBackend,
    pub mainframe: frame::Frame,
    pub bufsize: Size,
    pub mouse_pos: Pos,
    pub state: T,
    pub dirty_rect: Option<Option<Rect>>,
    pub user_cb: Option<UserCallback<T>>,
    pub debug_thing: u32,
    pub redraw_actions: Vec<Action>,
    pub actions: Vec<Action>,
    pub draw_command_list: Vec<DrawCommand>,
    pub lastframebuffersize: i32,
}

impl<T> AppCore<T> {
    pub fn new(init_state: T, renderstrat_given: RenderBackend) -> Self {
        let mut mainframe_setter = frame::Frame::new("mainframe".into());
        mainframe_setter.pos(Pos::new(0, 0));
        mainframe_setter.size(Size::new(800, 600));
        mainframe_setter.base.sizestrat.method = SizeEnum::MANUAL;
        mainframe_setter.color(Color::LIGHT_GRAY);
        mainframe_setter.set_relayout_flag(true);
        
        Self {
            render: renderstrat_given,
            mainframe: mainframe_setter,
            bufsize: Size::new(800, 600),
            mouse_pos: Pos::new(0, 0),
            state: init_state,
            dirty_rect: Some(None),
            user_cb: None,
            debug_thing: 0,
            actions: Vec::with_capacity(16),
            redraw_actions: Vec::with_capacity(16),
            draw_command_list: Vec::with_capacity(256),
            lastframebuffersize: 256,
        }
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear();
        self.redraw_actions.clear();
    }

    pub fn init_window(&mut self, window: Arc<dyn GuiWindow>, size: Size) {
        self.render.init_window(window, size);
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.bufsize = Size::new(width as i32, height as i32);
        self.mainframe.base.size = self.bufsize;
        
        self.mainframe.set_relayout_flag(true);
        self.mainframe.update_layout(true);

        if width > 0 && height > 0 {
            self.render.resize(self.bufsize);
        }
        
        self.dirty_rect = Some(None);
    }

    pub fn handle_mouse_click(&mut self, is_pressed: bool, key: MKey) {
        let click_event = if is_pressed {
            core::event::Event::MouseClick { pos: self.mouse_pos, key }
        } else {
            core::event::Event::MouseRelease { pos: self.mouse_pos, key }
        };
        self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut self.actions);
    }

    pub fn handle_mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_pos = Pos::new(x, y);
        let move_event = core::event::Event::MouseMove { pos: self.mouse_pos };
        self.mainframe.handle_event(&move_event, Pos::new(0, 0), &mut self.actions);
    }

    pub fn handle_keyboard_event(&mut self, is_pressed: bool, key: KKey, ch: char) {
        let click_event = if is_pressed {
            core::event::Event::KeyPress { ch, key }
        } else {
            core::event::Event::KeyRelease { ch, key }
        };
        println!("{:?}", &click_event);
        self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut self.actions);
    }

    ///Callbacks, dirty rects and relayout
    ///Returns true if needs redraw
    pub fn post_process_events(&mut self) -> bool {
        if !self.actions.is_empty() {
            if let Some(mut cb) = self.user_cb.take() {
                for action in &self.actions {
                    cb(action, &mut self.mainframe, &mut self.state);
                }
                self.user_cb = Some(cb);
            }
        }

        let needs_layout = self.mainframe.needs_relayout()
            || self.actions.iter().any(|a| matches!(a, Action::UpdateLayoutRequest));

        if needs_layout {
            self.mainframe.update_layout(true);
            self.mainframe.set_relayout_flag(false);
            self.dirty_rect = Some(None);
        }

        self.mainframe.get_dirty_rect(Pos::new(0, 0), &mut self.redraw_actions);

        if !self.redraw_actions.is_empty() {
            for action in &self.redraw_actions {
                if let Action::RedrawRequest(maybe_rect) = action {
                    match (self.dirty_rect, maybe_rect) {
                        (Some(None), _) => {} 
                        (_, None) => self.dirty_rect = Some(None), 
                        (None, Some(rect)) => self.dirty_rect = Some(Some(*rect)), 
                        (Some(Some(current_rect)), Some(new_rect)) => {
                            self.dirty_rect = match merge_rects(current_rect, *new_rect) {
                                Ok(merged) => Some(Some(merged)),
                                Err(_) => Some(None),
                            };
                        }
                    }
                }
            }
        }

        self.dirty_rect.is_some() || needs_layout
    }

    ///Rendering
    pub fn draw_to_slice(&mut self) {
        self.debug_thing = self.debug_thing.wrapping_add(1);

        if let Some(dirty_rect) = self.dirty_rect.take() {
            self.render.begin();

            let clip_rect = if self.render.is_partial_render() {
                dirty_rect.unwrap_or_else(|| {
                    Rect::from_xywh(0, 0, self.bufsize.width, self.bufsize.height).unwrap()
                })
            } else {
                Rect::from_xywh(0, 0, self.bufsize.width, self.bufsize.height).unwrap()
            };

            self.mainframe.draw(&mut self.draw_command_list, Pos::new(0, 0), clip_rect, None);

            self.render.rendercl(&self.draw_command_list);

            let current_len = self.draw_command_list.len();
            self.lastframebuffersize = current_len as i32;

            self.draw_command_list.clear();

            if self.draw_command_list.capacity() > 512 && current_len < self.draw_command_list.capacity() / 4 {
                self.draw_command_list.shrink_to((current_len * 2).max(256));
            }

            //Flushing rendered image to window surface
            self.render.flush();
        }

        self.mainframe.set_dirty_flag(false);
        self.dirty_rect = None;
    }
}