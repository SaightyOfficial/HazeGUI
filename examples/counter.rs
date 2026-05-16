use haze_gui::{Win, core::{color::Color, event::Action, size::Size}, widgets::{button::Button, frame::{Frame, FrameStyle}, label::Label}};

//creating appstate
pub struct AppState {
    pub click_count: i32,
}

fn main() {
    let init_state = AppState { click_count: 0 }; //initializing appstate
    let mut root = Win::new(init_state); // Creating window
    
    root.title("Simple counter");                    //Setting window title
    root.geometry(Size::new(400, 400)); //Setting window size

    let mut frame = Frame::new("frame".to_string()).color(Color::TEAL).style(FrameStyle::GROOVE);
    
    let text = Label::new("counter_text".to_string())
        .text(root.state.click_count.to_string())
        .bgcolor(Color::TRANSPARENT);//Setting up label with id, text, and background text color
        
    let buttonadd = Button::new("add".to_string())
        .text("+1")
        .textcolor(Color::WHITE)
        .color(Color::DARK_GRAY);//Setting up button with id, text, and background color

    let buttonsub = Button::new("sub".to_string())
        .text("-1")
        .textcolor(Color::WHITE)
        .color(Color::DARK_GRAY);//Setting up button with id, text, and background color
    
    frame.add_widget(text);     //Adding counter label to main frame
    frame.add_widget(buttonadd);//Adding add button to main frame
    frame.add_widget(buttonsub);//Adding substract button to main frame

    root.mainframe.add_widget(frame);

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