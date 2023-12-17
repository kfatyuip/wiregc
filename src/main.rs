use gdk_pixbuf::PixbufLoader;
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

    win.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("wireqc.moy.kirean"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
    // gtk::main();

    // let app = Application::builder()
    //     .application_id("moy.kieran.wiregc")
    //     .build();

    // app.connect_activate(|app| {
    //     let win = ApplicationWindow::builder()
    //         .application(app)
    //         .default_width(320)
    //         .default_height(200)
    //         .resizable(true)
    //         .title("Hello, World!")
    //         .build();

    //     let button = Button::with_label("Click me!");
    //     button.connect_clicked(clone!(@weak win, @weak button => move |_| {
    //         let size = &win.size();
    //         let ssize = format!("{}, {}", size.0.to_string(), size.1.to_string());
    //         button.set_label(ssize.as_str());
    //     }));
    //     win.add(&button);

    //     win.show_all();
    // });

    // app.run();
}
