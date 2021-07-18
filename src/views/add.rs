use kas::widget::MessageBox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build widgets.
    // Message is a Window with an "Ok" button and notification status.
    // Each Window::new method creates objects then solves constraints.
    let text = kas::text::format::Markdown::new("Hello *world*!")?;
    let window = MessageBox::new("Message", text);

    let theme = kas_theme::FlatTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}
