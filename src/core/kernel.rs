use crate::core;
use crate::core::renderconfig::RenderConfig;
use crate::core::{event::Action, pos::Pos, size::Size, common::merge_rects};
use core::color::Color;
use core::widget::Widget;
use crate::widgets::frame;

use tiny_skia::{Pixmap, Rect};

type UserCallback<T> = Box<dyn FnMut(&Action, &mut frame::Frame, &mut T)>;

pub struct AppCore<T> {
    pub backbuffer: Option<Pixmap>,
    pub renderconf: RenderConfig,
    pub mainframe: frame::Frame,
    pub bufsize: Size,
    pub mouse_pos: Pos,
    pub state: T,
    pub dirty_rect: Option<Option<Rect>>,
    pub user_cb: Option<UserCallback<T>>,
    pub debug_thing: u32,
    pub redraw_actions: Vec<Action>,
    pub actions: Vec<Action>,
}

impl<T> AppCore<T> {
    pub fn new(init_state: T, renderstrat_given: RenderConfig) -> Self {
        let mut mainframe_setter = frame::Frame::new("mainframe".into());
        mainframe_setter.pos(Pos::new(0, 0));
        mainframe_setter.size(Size::new(800, 600));
        mainframe_setter.base.sizestrat.method = core::common::SizeEnum::MANUAL;
        mainframe_setter.color(Color::LIGHT_GRAY);
        mainframe_setter.set_relayout_flag(true);
        
        Self {
            backbuffer: None,
            renderconf: renderstrat_given,
            mainframe: mainframe_setter,
            bufsize: Size::new(800, 600),
            mouse_pos: Pos::new(0, 0),
            state: init_state,
            dirty_rect: Some(None),
            user_cb: None,
            debug_thing: 0,
            actions: Vec::with_capacity(16),
            redraw_actions: Vec::with_capacity(16),
        }
    }

    pub fn clear_actions(&mut self) {
        self.actions.clear();
        self.redraw_actions.clear();
    }

    pub fn handle_resize(&mut self, width: u32, height: u32) {
        self.bufsize = Size::new(width as i32, height as i32);
        self.mainframe.base.size = self.bufsize;
        self.mainframe.update_layout(true);

        if width > 0 && height > 0 {
            self.backbuffer = Some(Pixmap::new(width, height).expect("Failed to resize backbuffer"));
        }
        self.dirty_rect = Some(None);
    }

    pub fn handle_mouse_click(&mut self, is_pressed: bool) {
        let click_event = if is_pressed {
            core::event::Event::MouseClick { pos: self.mouse_pos }
        } else {
            core::event::Event::MouseRelease { pos: self.mouse_pos }
        };
        self.mainframe.handle_event(&click_event, Pos::new(0, 0), &mut self.actions);
    }

    pub fn handle_mouse_move(&mut self, x: i32, y: i32) {
        self.mouse_pos = Pos::new(x, y);
        let move_event = core::event::Event::MouseMove { pos: self.mouse_pos };
        self.mainframe.handle_event(&move_event, Pos::new(0, 0), &mut self.actions);
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
    pub fn draw_to_slice(&mut self, window_buffer: &mut [u32]) {
        self.debug_thing += 1;
        println!("Redrawing {}", self.debug_thing);

        if let (Some(backbuffer), Some(dirty_rect)) = (&mut self.backbuffer, self.dirty_rect.take()) {
            let clip_rect = match dirty_rect {
                Some(rect) => rect,
                None => Rect::from_xywh(0.0, 0.0, self.bufsize.width as f32, self.bufsize.height as f32).unwrap(),
            };

            //println!("{:?}", dirty_rect);

            self.mainframe.draw(&mut backbuffer.as_mut(), Pos::new(0, 0), clip_rect, None);

            let raw_window_bytes: &mut [u8] = bytemuck::cast_slice_mut(window_buffer);
            raw_window_bytes.copy_from_slice(bytemuck::cast_slice(backbuffer.data()));
        }

        self.mainframe.set_dirty_flag(false);
        self.dirty_rect = None;
    }
}