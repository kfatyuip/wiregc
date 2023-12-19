use wiregc::conf;
use wiregc::exec;

use gdk_pixbuf::PixbufLoader;
use glib::GString;
use glib::clone;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("wiregc.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let win: gtk::Window = builder.object("main_window").unwrap();
    win.set_application(Some(app));
    let loader = PixbufLoader::with_type("svg").unwrap();
    loader
        .write(include_bytes!("../resouces/wireguard.svg"))
        .unwrap();
    loader.close().unwrap();
    win.set_icon(Some(&loader.pixbuf()).unwrap().as_ref());
    let tunnel_list: gtk::ScrolledWindow = builder.object("tunnel_list").unwrap();
    let tunnel_listbox: gtk::ListBox = gtk::ListBox::new();
    tunnel_list.add(&tunnel_listbox);
    let paths: Vec<std::path::PathBuf> = conf::get_conf_paths().unwrap();
    let interfaces: Vec<&str> = paths
        .iter()
        .map(|path| {
            let conf: &str = path.file_name().unwrap().to_str().unwrap();
            conf.split(".conf").next().unwrap()
        })
        .collect();

    for interface in &interfaces {
        let row: gtk::ListBoxRow = gtk::ListBoxRow::new();
        row.add(&gtk::Label::new(Some(&interface)));
        tunnel_listbox.add(&row);
    }
    tunnel_listbox.show_all();
    let button: gtk::Button = builder.object("active").unwrap();
    button.connect_clicked(clone!(@strong button=> move |_| {
    let mut selected_interface:GString = GString::new();
        for childrow in tunnel_listbox.children().iter()
        {
            let row:gtk::ListBoxRow = childrow.clone().downcast::<gtk::ListBoxRow>().unwrap();
            if row.is_selected() {
                for childlabel in row.children().iter() {
                    let label = childlabel.clone().downcast::<gtk::Label>();
                    let text = label.unwrap().label();
                    selected_interface = text;
                }
            }
        }
        if button.label().unwrap().as_str() == "active" {
            button.set_label("disable");
            exec::active_wg(&selected_interface.as_str()).unwrap();
        } else {
            button.set_label("active");
            exec::deactive_wg(&selected_interface.as_str()).unwrap();
        }
    }));

    win.show_all();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application = gtk::Application::new(Some("wireqc.moy.kirean"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();

    Ok(())
}
