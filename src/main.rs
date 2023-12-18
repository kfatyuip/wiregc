use wiregc::exec;

use gdk_pixbuf::PixbufLoader;
use glib::clone;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("builder_basics.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let win: gtk::Window = builder.object("window1").unwrap();
    win.set_application(Some(app));
    let loader = PixbufLoader::with_type("svg").unwrap();
    loader
        .write(include_bytes!("../resouces/icon.svg"))
        .unwrap();
    loader.close().unwrap();
    win.set_icon(Some(&loader.pixbuf()).unwrap().as_ref());
    let button: gtk::Button = builder.object("button1").unwrap();
    button.connect_clicked(clone!(@strong button => move |_| {
        if button.label().unwrap().as_str() == "active" {
            button.set_label("disable");
            exec::active_wg("wg0").unwrap();
        } else {
            button.set_label("active");
            exec::deactive_wg("wg0").unwrap();
        }
    }));

    win.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("wireqc.moy.kirean"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
