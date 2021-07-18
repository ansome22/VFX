#[derive(Clone, Debug, VoidMsg)]
enum Item {
    Button,
    Check(bool),
    Combo(i32),
    Radio(u32),
    Edit(String),
    Slider(i32),
    Scroll(i32),
}

let widgets = make_widget! {
	#[layout(grid)]
	#[handler(msg = Item)]
	struct {
		#[widget(row=0, col=0)] _ = Label::new("Label"),
		#[widget(row=0, col=1)] _ = Label::new("Hello world"),
		#[widget(row=1, col=0)] _ = Label::new("EditBox"),
		#[widget(row=1, col=1)] _ = EditBox::new("edit me").with_guard(Guard),
		#[widget(row=2, col=0)] _ = Label::new("TextButton"),
		#[widget(row=2, col=1)] _ = TextButton::new_msg("&Press me", Item::Button),
		#[widget(row=3, col=0)] _ = Label::new("CheckBox"),
		#[widget(row=3, col=1)] _ = CheckBox::new("&Check me")
			.with_state(true)
			.on_toggle(|_, check| Some(Item::Check(check))),
		#[widget(row=4, col=0)] _ = Label::new("RadioBox"),
		#[widget(row=4, col=1)] _ = RadioBox::new("radio box &1", radio)
			.on_select(|_| Some(Item::Radio(1))),
		#[widget(row=5, col=0)] _ = Label::new("RadioBox"),
		#[widget(row=5, col=1)] _ = RadioBox::new("radio box &2", radio)
			.with_state(true)
			.on_select(|_| Some(Item::Radio(2))),
		#[widget(row=6, col=0)] _ = Label::new("ComboBox"),
		#[widget(row=6, col=1)] _ =
			ComboBox::new(&["&One", "T&wo", "Th&ree"], 0)
			.on_select(|_, index| Some(Item::Combo((index + 1).cast()))),
		#[widget(row=7, col=0)] _ = Label::new("Slider"),
		#[widget(row=7, col=1, handler = handle_slider)] s =
			Slider::<i32, Right>::new(-2, 2, 1).with_value(0),
		#[widget(row=8, col=0)] _ = Label::new("ScrollBar"),
		#[widget(row=8, col=1, handler = handle_scroll)] sc: ScrollBar<Right> =
			ScrollBar::new().with_limits(100, 20),
		#[widget(row=9, col=1)] pg: ProgressBar<Right> = ProgressBar::new(),
		#[widget(row=9, col=0)] _ = Label::new("ProgressBar"),
		#[widget(row=10, col=0)] _ = Label::new("SVG"),
		#[widget(row=10, col=1, align=centre)] _ =
			Svg::from_path_and_factors("res/rustacean-flat-happy.svg", 0.1, 0.3),
		#[widget(row=11, col=0)] _ = Label::new("Child window"),
		#[widget(row=11, col=1)] _ = popup_edit_box,
	}