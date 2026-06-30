use crate::graphics::button::Button;
use crate::graphics::canvas::Canvas;
use crate::graphics::control_action::ControlAction;
use crate::graphics::pointer::Pointer;

pub struct ControlPanel {
    generate_button: Button,
    solve_button: Button,
}

impl ControlPanel {
    pub fn new(generate_button: Button, solve_button: Button) -> Self {
        Self {
            generate_button,
            solve_button,
        }
    }

    pub fn action(&self, pointer: &dyn Pointer) -> ControlAction {
        if self.generate_button.is_clicked(pointer) {
            ControlAction::Generate
        } else if self.solve_button.is_clicked(pointer) {
            ControlAction::Solve
        } else {
            ControlAction::None
        }
    }
    pub fn draw(&self, canvas: &mut dyn Canvas) {
        self.generate_button.draw(canvas);
        self.solve_button.draw(canvas);
    }
}
