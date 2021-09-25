use fltk::{*, button::Button, enums::*, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut win = Window::default().with_size(500, 500);
    let mut but = Button::default().with_size(100, 50).with_label("Test").center_of_parent();
    but.set_callback(move |b| {
        let mut bb = b.clone();
        let x =std::thread::spawn(move || {
            while bb.x() < 400 {
                bb.set_pos(bb.x() + 1, bb.y());
                app::sleep(0.03);
                bb.parent().unwrap().redraw();
            }
        });
    });
    win.end();
    win.show();
    app.run().unwrap();
}
