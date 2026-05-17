use haze_gui::{Win, core::{common::RenderStrategy, size::Size}, widgets::label::Label};

pub struct AppState {}

fn main() {
    let init_state = AppState {}; //initializing appstate
    let mut root = Win::new(init_state, RenderStrategy::CpuOptimized); // Creating window, note that on desktop pcs it is better to use cpu optimized render strategy

    root.title("HazeGUI Sides"); //Setting window title
    root.geometry(Size::new(300, 150)); //Setting window size
    root.resizable(false); //Can window be resized?

    
    let text = Label::new("text".to_string())
        .text("Hello world!".to_string());
    
    root.mainframe.add_widget(text);//Adding label to main frame

    root.mainloop(|action, _mainframe, _state| { //Mainloop is mostly empty here because we dont need to process any actions
        match action {
            _ => {}
        }
    });
}