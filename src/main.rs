use druid::im::{Vector, vector};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, RadioGroup, Scroll, TextBox, WidgetExt,
};
use druid::{AppLauncher, Data, Env, EventCtx, Lens, Widget, WindowDesc};
use druid_widget_nursery::dropdown::DROPDOWN_SHOW;
use druid_widget_nursery::Dropdown;

#[derive(Data, Clone, Lens)]
struct AppData {
    selected: String
}

#[derive(Data, Clone)]
struct VarianDetails {
    id: i32,
    description: String
}

fn get_variants(data: &AppData) -> Vec<(String, String)> {
    vec![
        ("Variant 1 description".to_string(), "Variant1".to_string()),
        ("Variant 2 description".to_string(), "Variant2".to_string()),
        ("Variant 3 description".to_string(), "Variant3".to_string())
    ]
}

fn main_widget(data: &mut AppData) -> impl Widget<AppData> {
    let variants = get_variants(data);
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Dropdown::new(
                Flex::row()
                    .with_child(TextBox::new())
                    .with_flex_spacer(1.)
                    .with_child(Button::new("V").on_click(|ctx: &mut EventCtx, _, _| {
                        ctx.submit_notification(DROPDOWN_SHOW)
                    })),
                move |_, _| {
                    RadioGroup::new(variants.to_owned())
                },
            )
            .align_left()
            .lens(AppData::selected),
        )
        .fix_width(250.)
}

pub fn main() {
    // create the initial app state
    let mut app_data = AppData {
        selected: String::new(),
    };

    let main_window = WindowDesc::new(main_widget(&mut app_data))
        .title("Dropdown")
        .window_size((250., 300.));

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(app_data)
        .expect("Failed to launch application");
}