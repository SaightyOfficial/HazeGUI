use haze_gui::{Win, core::{color::Color, size::Size, event::Action}, widgets::{button::Button, label::Label}};

pub struct AppState {
    pub click_count: i32,
}

fn main() {
    let init_state = AppState { click_count: 0 };
    let mut root = Win::new(init_state);
    
    root.title("Simple counter");
    root.geometry(Size::new(400, 400));
    
    let text = Label::new("counter_text".to_string())
        .text(root.state.click_count.to_string())
        .bgcolor(Color::TRANSPARENT);
        
    let buttonadd = Button::new("add".to_string())
        .text("+1")
        .color(Color::BLUE);

    let buttonsub = Button::new("sub".to_string())
        .text("-1")
        .color(Color::BLUE);
    
    root.mainframe.add_widget(text);
    root.mainframe.add_widget(buttonadd);
    root.mainframe.add_widget(buttonsub);

    root.mainloop(|action, mainframe, state| {
        match action {
            Action::ButtonClicked(id) => {
                if id == "add" {
                    state.click_count += 1;
                    
                    if let Some(widget) = mainframe.find_mut("counter_text") {
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                            label.new_text(state.click_count.to_string());
                        }
                    }
                }
                if id == "sub" {
                    state.click_count -= 1;
                    
                    if let Some(widget) = mainframe.find_mut("counter_text") {
                        if let Some(label) = widget.as_any_mut().downcast_mut::<Label>() {
                            label.new_text(state.click_count.to_string());
                        }
                    }
                }
            }
            //Action::Hovered(id) => {
            //    println!("Hovered event: {}", id);
            //}
            _ => {}
        }
    });
}