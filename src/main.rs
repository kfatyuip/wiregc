use wiregc::conf;
use wiregc::exec;

use gdk_pixbuf::PixbufLoader;
use glib::clone;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("wiregc.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let win: gtk::Window = builder
        .object("main_window")
        .expect("Failed to get main_window");
    win.set_application(Some(app));

    let loader = PixbufLoader::with_type("svg").expect("Failed to create PixbufLoader for SVG");
    loader
        .write(include_bytes!("../resources/wireguard.svg"))
        .expect("Failed to read SVG data");
    loader.close().expect("Failed to close PixbufLoader");
    if let Some(pixbuf) = loader.pixbuf() {
        win.set_icon(Some(&pixbuf));
    }

    let tunnel_list: gtk::ScrolledWindow = builder
        .object("tunnel_list")
        .expect("Failed to get tunnel_list");
    let tunnel_listbox: gtk::ListBox = gtk::ListBox::new();
    tunnel_list.add(&tunnel_listbox);

    let paths: Vec<std::path::PathBuf> =
        conf::get_conf_paths().expect("Failed to get configuration paths");
    let interfaces: Vec<&str> = paths
        .iter()
        .filter_map(|path| path.file_name()?.to_str()?.split(".conf").next())
        .collect();

    for interface in &interfaces {
        let row: gtk::ListBoxRow = gtk::ListBoxRow::new();
        row.add(&gtk::Label::new(Some(&interface)));
        tunnel_listbox.add(&row);
    }
    tunnel_listbox.show_all();

    let button: gtk::Button = builder
        .object("active")
        .expect("Failed to get active button");
    let connect_interface_ptr = Rc::new(RefCell::new(None::<glib::GString>)).clone();
    button.connect_clicked(
        clone!(@strong button, @strong connect_interface_ptr => move |_| {
            let mut selected_interface: Option<glib::GString> = None;
            for childrow in tunnel_listbox.children().iter() {
                let row = childrow.downcast_ref::<gtk::ListBoxRow>().unwrap();
                if row.is_selected() {
                    for childlabel in row.children().iter() {
                        let label = childlabel.downcast_ref::<gtk::Label>().unwrap();
                        let text = label.label();
                        selected_interface = Some(text);
                    }
                }
            }
            if connect_interface_ptr.borrow().clone() != None {
                exec::deactive_wg(&connect_interface_ptr.borrow().clone().unwrap().as_str()).unwrap();
                *connect_interface_ptr.borrow_mut() = None;
            }
            if button.label().unwrap().as_str() == "active" {
                exec::active_wg(&selected_interface.clone().unwrap().as_str()).unwrap();
                button.set_label("deactive");
                *connect_interface_ptr.borrow_mut() = selected_interface;
            } else {
                button.set_label("active");
            }
        }),
    );

    win.show_all();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let application = gtk::Application::new(Some("wireqc.moy.kirean"), Default::default());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();

    Ok(())
}
