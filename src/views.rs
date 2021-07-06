let theme = kas_theme::ShadedTheme::new();
let mut toolkit = kas_wgpu::Toolkit::new(theme)?;

let window = Window::new(
	"Widget Gallery",
	make_widget! {
		#[layout(column)]
		#[handler(msg = VoidMsg)]
		struct {
			#[widget(handler = menu)] _ = menubar,
			#[widget(halign = centre)] _ = Frame::new(head),
			#[widget(handler = activations)] gallery:
				for<W: Widget<Msg = Item>> ScrollBarRegion<W> =
					ScrollBarRegion::new(widgets),
		}
		impl {
			fn menu(&mut self, mgr: &mut Manager, msg: Menu) -> VoidResponse {
				match msg {
					Menu::Theme(name) => {
						println!("Theme: {:?}", name);
						#[cfg(not(feature = "stack_dst"))]
						println!("Warning: switching themes requires feature 'stack_dst'");

						mgr.adjust_theme(|theme| theme.set_theme(name));
					}
					Menu::Colour(name) => {
						println!("Colour scheme: {:?}", name);
						mgr.adjust_theme(|theme| theme.set_scheme(&name));
					}
					Menu::Disabled(state) => {
						*mgr |= self.gallery.set_disabled(state);
					}
					Menu::Quit => {
						*mgr |= TkAction::EXIT;
					}
				}
				Response::None
			}
			fn activations(&mut self, _: &mut Manager, item: Item) -> VoidResponse {
				match item {
					Item::Button => println!("Clicked!"),
					Item::Check(b) => println!("CheckBox: {}", b),
					Item::Combo(c) => println!("ComboBox: {}", c),
					Item::Radio(id) => println!("RadioBox: {}", id),
					Item::Edit(s) => println!("Edited: {}", s),
					Item::Slider(p) => println!("Slider: {}", p),
					Item::Scroll(p) => println!("ScrollBar: {}", p),
				};
				Response::None
			}
		}
	},
);


toolkit.add(window)?;
toolkit.run()