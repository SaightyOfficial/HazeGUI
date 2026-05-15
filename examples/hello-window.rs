use haze_gui::{Win, core::size::Size, widgets::label::Label};

pub struct AppState {}

fn main() {
    let init_state = AppState {};
    let mut root = Win::new(init_state);
    
    root.title("HazeGUI Hello window");
    root.geometry(Size::new(300, 150));
    root.resizable(false);

    
    let text = Label::new("text".to_string())
        .text("Hello world!".to_string());
    
    root.mainframe.add_widget(text);//Adding label to main frame

    root.mainloop(|action, _mainframe, _state| { //Mainloop is mostly empty here because we dont need to process ane actions
        match action {
            _ => {}
        }
    });
}