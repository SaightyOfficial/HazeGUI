use haze_gui::widgets::heavy::scrollframe::ScrollFrame;
/*
NOTE: This file is just for my tests while im working on this lib,
there will be from small to no comments

This is just my sandbox so idk =3
*/
#[allow(unused)]
use haze_gui::{
    Win,
    hsid,
    core::{
        color::Color,
        common::Side,
        event::Action,
        size::Size,
        common::Axis, renderconfig::RenderConfig, widget::Widget},
    widgets::{
        
        button::Button,
        frame::{Frame, FrameStyle},
        label::Label,
        scrollbar::ScrollBar,
    },
};

//creating appstate
pub struct AppState {
    pub click_count: i32,
}

fn main() {
    let init_state = AppState { click_count: 0 };
    let mut root = Win::new(init_state, RenderConfig::default());
    root.title("Ya sumasheshi =3");
    root.geometry(Size::new(400, 400));
    //root.set_fps(60);

    let mut text = Label::new("counter_text".into());
    text.text(root.core.state.click_count.to_string());
    text.bgcolor(Color::TRANSPARENT); //Setting up label with id, text, and background text color

    let mut buttonadd = Button::new("add".into());
    buttonadd.text("+1".into());
    buttonadd.textcolor(Color::WHITE);
    buttonadd.color(Color::DARK_GRAY); //Setting up button with id, text, and background color

    let mut buttonsub = Button::new("sub".into());
    buttonsub.text("-1".into());
    buttonsub.textcolor(Color::WHITE);
    buttonsub.color(Color::DARK_GRAY); //Setting up button with id, text, and background color

    let mut a = ScrollFrame::new("a".into(), Axis::BOTH);
    a.fill(Axis::BOTH);
    //a.padding(10);

    a.add_widget(text); //Adding counter label to main frame
    a.add_widget(buttonadd); //Adding add button to main frame
    a.add_widget(buttonsub); //Adding substract button to main frame

    root.core.mainframe.add_widget(a);

    root.mainloop(|action, mainframe, state| {
        match action {
            Action::ButtonReleased(id) => {
                if *id == hsid!("add") { // checking button id
                    state.click_count += 1; //changing counter data
                    //searching for counter label in mainframe using id
                    if let Some(widget) = mainframe.find_mut(hsid!("counter_text")) {
                        //assuming that what we are found is an label
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                            label.text(state.click_count.to_string()); //changing label text
                        }
                    }
                }
                if *id == hsid!("sub") { // checking button id
                state.click_count -= 1; //changing counter data
                    //searching for counter label in mainframe using id
                    if let Some(widget) = mainframe.find_mut(hsid!("counter_text")) {
                        //assuming that what we are found is an label
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                            label.text(state.click_count.to_string()); //changing label text
                        }
                    }
                }
            }
        _ => {}
    }});
}
