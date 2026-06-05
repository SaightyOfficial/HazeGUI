use haze_gui::{
    Win,
    core::{renderconfig::RenderConfig, size::Size},
    widgets::label::Label,
};

pub struct AppState {}

fn main() {
    let init_state = AppState {}; //initializing appstate
    let mut root = Win::new(init_state, RenderConfig::default()); // Creating window, note that on desktop pcs it is better to use cpu optimized render strategy

    root.title("HazeGUI Hello"); //Setting window title
    root.geometry(Size::new(300, 150)); //Setting window size
    root.resizable(false); //Can window be resized?

    let mut text = Label::new("text".to_string());
    text.text("Hello world!".to_string());

    root.core.mainframe.add_widget(text); //Adding label to main frame

    root.mainloop(|action, _mainframe, _state| {
        //Mainloop is mostly empty here because we dont need to process any actions
        match action {
            _ => {}
        }
    });
}
