use haze_gui::{Win, core::{color::Color, common::Side, size::Size}, widgets::{frame::{Frame, FrameStyle}, label::Label}};

pub struct AppState {}

fn main() {
    let init_state = AppState {}; //initializing appstate
    let mut root = Win::new(init_state); // Creating window
    
    root.title("HazeGUI Sides"); //Setting window title
    root.geometry(Size::new(300, 300)); //Setting window size
    
    let textmiddle = Label::new("textmiddle".into())
        .text("Middle text".into())
        .side(Side::MIDDLE);//Creating new text that will be in the middle of the frame(Side::MIDDLE is default)

    let textleft = Label::new("textleft".into())
        .text("Left text".into())
        .side(Side::LEFT);//Creating new text that will be on the left

    let texright = Label::new("textright".into())
        .text("Right text".into())
        .side(Side::RIGHT);//Creating new text that will be on the right
    let texrightagain = Label::new("textrightagain".into())
        .text("again..".into())
        .side(Side::RIGHT);//Creating new text that will be on the right, again...

    let mut frame = Frame::new("frame".into())
        .style(FrameStyle::SUNKEN)
        .color(Color::TEAL);
    
    //Adding them in random order to show that composing engine is working well
    frame.add_widget(textmiddle);//Adding label to frame
    frame.add_widget(texright);//Adding label to frame
    frame.add_widget(texrightagain);//Adding label to frame
    frame.add_widget(textleft);//Adding label to frame

    root.mainframe.add_widget(frame);//Adding frame to mainframe

    root.mainloop(|action, _mainframe, _state| { //Mainloop is mostly empty here because we dont need to process any actions
        match action {
            _ => {}
        }
    });
}