use haze_gui::{
    Win, core::{color::Color, event::Action, render::cpurender::CPURenderConfig, render::renderconfig::RenderBackend, size::Size, widget::Widget}, hsid, widgets::{button::Button, label::Label}
};

//creating appstate
pub struct AppState {
    pub click_count: i32,
}

fn main() {
    let init_state = AppState { click_count: 0 }; //initializing appstate
    let mut root = Win::new(init_state, RenderBackend::CPU(CPURenderConfig::default())); // Creating window

    root.title("Simple counter"); //Setting window title
    root.geometry(Size::new(400, 400)); //Setting window size
    //root.resizable(false); //Can window be resized?

    let mut text = Label::new("counter_text".into());
    text.set_text(root.core.state.click_count.to_string());
    //text.bgcolor(Color::TRANSPARENT); //Setting up label with id, text, and background text color

    let mut buttonadd = Button::new("add".into());
    buttonadd.text("+1".into());
    buttonadd.textcolor(Color::WHITE);
    buttonadd.color(Color::DARK_GRAY); //Setting up button with id, text, and background color

    let mut buttonsub = Button::new("sub".into());
    buttonsub.text("-1".into());
    buttonsub.textcolor(Color::WHITE);
    buttonsub.color(Color::DARK_GRAY); //Setting up button with id, text, and background color

    root.core.mainframe.add_widget(text); //Adding counter label to main frame
    root.core.mainframe.add_widget(buttonadd); //Adding add button to main frame
    root.core.mainframe.add_widget(buttonsub); //Adding substract button to main frame

    root.mainloop(|action, mainframe, state| {
        //Mainloop where you procces events
        match action {
            //proccesing action that buttons send when they are released
            Action::ButtonReleased(id) => {
                if *id == hsid!("add") { // checking button id
                    state.click_count += 1; //changing counter data
                    //searching for counter label in mainframe using id
                    if let Some(widget) = mainframe.find_mut(hsid!("counter_text")) {
                        //assuming that what we are found is an label
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                            label.set_text(state.click_count.to_string()); //changing label text
                        }
                    }
                }
                if *id == hsid!("sub") { // checking button id
                    state.click_count -= 1; //changing counter data
                    //searching for counter label in mainframe using id
                    if let Some(widget) = mainframe.find_mut(hsid!("counter_text")) {
                        //assuming that what we are found is an label
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                            label.set_text(state.click_count.to_string()); //changing label text
                        }
                    }
                }
            }
            _ => {} // ignoring other signals such as Action::None and Action::Hovered
        }
    });
}
