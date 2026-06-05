use haze_gui::{
    Win,
    core::{
        color::Color, common::Side, renderconfig::RenderConfig, size::Size
    },
    widgets::{
        frame::{Frame, FrameStyle},
        label::Label,
    },
};

pub struct AppState {}

fn main() {
    let init_state = AppState {}; //initializing appstate
    let mut root = Win::new(init_state, RenderConfig::default()); // Creating window, note that on desktop pcs it is better to use cpu optimized render strategy

    root.title("HazeGUI Sides"); //Setting window title
    root.geometry(Size::new(300, 300)); //Setting window size

    let mut textmiddle = Label::new("textmiddle".into());
    textmiddle.text("Middle text".into());
    textmiddle.side(Side::MIDDLE); //Creating new text that will be in the middle of the frame(Side::MIDDLE is default)

    let mut textleft = Label::new("textleft".into());
    textleft.text("Left text".into());
    textleft.side(Side::LEFT); //Creating new text that will be on the left

    let mut textright = Label::new("textright".into());
    textright.text("Right text".into());
    textright.side(Side::RIGHT); //Creating new text that will be on the right

    let mut textrightagain = Label::new("textrightagain".into());
    textrightagain.text("again..".into());
    textrightagain.side(Side::RIGHT); //Creating new text that will be on the right, again...

    let mut frame = Frame::new("frame".into());
    frame.style(FrameStyle::SUNKEN);
    frame.color(Color::TEAL);

    //Adding them in random order to show that composing engine is working well
    frame.add_widget(textmiddle); //Adding label to frame
    frame.add_widget(textright); //Adding label to frame
    frame.add_widget(textrightagain); //Adding label to frame
    frame.add_widget(textleft); //Adding label to frame

    root.core.mainframe.add_widget(frame); //Adding frame to mainframe

    root.mainloop(|action, _mainframe, _state| {
        //Mainloop is mostly empty here because we dont need to process any actions
        match action {
            _ => {}
        }
    });
}
