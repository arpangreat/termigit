use cursive::{Cursive, CursiveExt};
use cursive::views::{Button, TextView, FixedLayout};
use cursive::Rect;

fn main() {
    let mut app = Cursive::new();

    app.load_toml(include_str!("tui.toml")).unwrap();

    // app.add_layer(TextView::new("Hello This is Termigit!!"));

    // app.add_global_callback('q', |s| s.quit());

    // let quit_button = Button::new("Quit", |s| s.quit());

    // app.add_layer(quit_button);





let layout = FixedLayout::new()
    .child(Rect::from_size((0, 0), (22, 1)), TextView::new("Hello This is Termigit"))
    .child(
        Rect::from_size((3, 15), (16, 1)),
        Button::new("Quit", |s| s.quit()),
    );

    app.add_layer(layout);



    app.run();
}
