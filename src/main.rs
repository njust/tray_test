use tray_item::TrayItem;
use gtk;
use gio::ResourceLookupFlags;

fn main() {
    gtk::init().unwrap();

    let res_bytes = include_bytes!("../resources.gresource");
    let data = glib::Bytes::from(&res_bytes[..]);
    let resources = gio::Resource::from_data(&data).unwrap();
    gio::resources_register(&resources);

    let svg = gio::resources_lookup_data("/org/gtk/test/assets/test.svg", ResourceLookupFlags::all()).expect("Failed to load svg");
    println!("Svg size: {}", svg.len());
    let png = gio::resources_lookup_data("/org/gtk/test/assets/test.png", ResourceLookupFlags::all()).expect("Failed to load png");
    println!("Png size: {}", png.len());

    let mut tray = TrayItem::new("Tray Example", "/org/gtk/test/assets/test.svg").unwrap();
    // let mut tray = TrayItem::new("Tray Example", "/org/gtk/test/assets/test.png").unwrap();
    // let mut tray = TrayItem::new("Tray Example", "accessories-calculator").unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).unwrap();

    gtk::main();

}