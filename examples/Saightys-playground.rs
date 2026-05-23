/*
NOTE: This file is just for my tests while im working on this lib,
there will be from small to no comments

This is just my sandbox so idk =3
*/

use haze_gui::hsid;
#[allow(unused)]
use haze_gui::{
    Win,
    core::{
        color::Color,
        common::{RenderStrategy, Side},
        event::Action,
        size::Size,
    },
    widgets::{
        button::Button,
        frame::{Frame, FrameStyle},
        label::Label,
    },
};

//creating appstate
pub struct AppState {
    pub click_count: i32,
}

fn main() {
    let init_state = AppState { click_count: 0 };
    let mut root = Win::new(init_state, RenderStrategy::CpuOptimized);
    root.title("Simple counter");
    root.geometry(Size::new(400, 400));

    let text = Label::new("counter_text".to_string())
        .text("lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsdlsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsdlsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsdlsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd
    lsd;akkl;asdkdl;sasadlk;sadkl;sdalk;sadkl;sdakl;asdkl;dskl;adskl;asdkl;daskl;asdkl;dskla;lk;asdkl;daskl;sadkl;asdkl;asd
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd".into())
        .font_size(10.0)
        .bgcolor(Color::TRANSPARENT);

    root.mainframe.add_widget(text);

    root.mainloop(|action, mainframe, state| match action {
        Action::ButtonReleased(id) => {
            if *id == hsid!("add") {
                state.click_count += 1;

                if let Some(widget) = mainframe.find_mut(hsid!("counter_text")) {
                    if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                        label.new_text(state.click_count.to_string());
                    }
                }
            }
            if *id == hsid!("sub") {
                state.click_count -= 1;

                if let Some(widget) = mainframe.find_mut(hsid!("counter_text")) {
                    if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                        label.new_text(state.click_count.to_string());
                    }
                }
            }
        }
        _ => {}
    });
}
