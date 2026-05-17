use haze_gui::{Win, core::{color::Color, common::{RenderStrategy, Side}, event::Action, size::Size}, widgets::{button::Button, frame::{Frame, FrameStyle}, label::Label}};


/* 
NOTE: This file is just for my tests while im working on this lib,
there will be from small to no comments

This is just my sandbox so idk =3
*/

//creating appstate
pub struct AppState {
    pub click_count: i32,
}

fn main() {
    let init_state = AppState { click_count: 0 }; //initializing appstate
    let mut root = Win::new(init_state, RenderStrategy::CpuOptimized); // Creating window, note that on desktop pcs it is better to use cpu optimized render strategy

    root.title("Simple counter"); //Setting window title
    root.geometry(Size::new(400, 400)); //Setting window size
    
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
    lkhaskdjhsdklhgsdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhsgdfkjhgsdfkjhgsdfkjhgsd".into())
        .font_size(10.0)
        .bgcolor(Color::TRANSPARENT);//Setting up label with id, text, and background text color
    
    root.mainframe.add_widget(text);     //Adding counter label to main frame

    root.mainloop(|action, mainframe, state| { //Mainloop where you procces events
        match action {
            Action::ButtonReleased(id) => { //proccesing action that buttons send when they are clicked
                if id == "add" { // checking button id
                    state.click_count += 1; //changing counter data
                    
                    if let Some(widget) = mainframe.find_mut("counter_text") { //searching for counter label in mainframe using id
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() { //assuming that what we are found is an label
                            label.new_text(state.click_count.to_string()); //changing label text
                            //mainframe.style = FrameStyle::GROOVE;
                        }
                    }
                }
                if id == "sub" {  // checking button id
                    state.click_count -= 1; //changing counter data
                    
                    if let Some(widget) = mainframe.find_mut("counter_text") { //searching for counter label in mainframe using id
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() { //assuming that what we are found is an label
                            label.new_text(state.click_count.to_string()); //changing label text
                            //mainframe.style = FrameStyle::FLAT;
                        }
                    }
                }
            }
            _ => {}// ignoring other signals such as Acrion::None
        }
    });
}