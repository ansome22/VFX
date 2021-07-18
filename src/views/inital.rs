use kas::event::{Manager, VoidMsg, VoidResponse};
use kas::macros::{make_widget, VoidMsg};
use kas::widget::{EditField, RowSplitter, TextButton, Window};

#[derive(Clone, Debug, VoidMsg)]
enum Message {
    Decr,
    Incr,
}

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let buttons = make_widget! {
        #[layout(row)]
        #[handler(msg = Message)]
        struct {
            #[widget] _ = TextButton::new_msg("−", Message::Decr),
            #[widget] _ = TextButton::new_msg("+", Message::Incr),
        }
    };
    let mut panes = RowSplitter::<EditField>::default();
    let _ = panes.resize_with(2, |n| {
        EditField::new(format!("Pane {}", n)).multi_line(true)
    });

    let window = Window::new(
        "Slitter panes",
        make_widget! {
            // TODO: use vertical splitter
            #[layout(column)]
            #[handler(msg = VoidMsg)]
            struct {
                #[widget(handler = handle_button)] buttons -> Message = buttons,
                #[widget] panes: RowSplitter<EditField> = panes,
                counter: usize = 0,
            }
            impl {
                fn handle_button(&mut self, mgr: &mut Manager, msg: Message)
                    -> VoidResponse
                {
                    match msg {
                        Message::Decr => {
                            *mgr |= self.panes.pop().1;
                        }
                        Message::Incr => {
                            let n = self.panes.len() + 1;
                            *mgr |= self.panes.push(EditField::new(format!("Pane {}", n)).multi_line(true));
                        }
                    };
                    VoidResponse::None
                }
            }
        },
    );

    let theme = kas_theme::ShadedTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}
