use ksni;
use ksni::Icon;
use std::io::Cursor;

#[derive(Debug)]
struct MyTray {
    selected_option: usize,
    checked: bool,
    ti: Vec<u8>,
}

impl ksni::Tray for MyTray {
    fn title(&self) -> String {
        "MyTray".into()
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        vec![Icon {
           width: 64,
           height: 64,
            data: self.ti.clone()
        }]
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

fn main() {
    let cursor = Cursor::new(include_bytes!("../assets/test.png"));
    let decoder = png::Decoder::new(cursor);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let service = ksni::TrayService::new(MyTray {
        selected_option: 0,
        checked: false,
        ti: buf
    });

    service.spawn();
    loop {
        std::thread::park();
    }
}