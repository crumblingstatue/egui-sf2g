use {
    egui_demo_lib::{DemoWindows, easy_mark},
    egui_sf2g::SfEgui,
    sf2g::{
        graphics::{Color, Rect, RenderTarget, RenderWindow, View},
        window::{Event, Style, VideoMode},
    },
};

fn main() {
    let vm = VideoMode::desktop_mode();
    let mut rw = RenderWindow::new(vm, "Egui test", Style::NONE, &Default::default()).unwrap();
    rw.set_position((0, 0).into());
    rw.set_vertical_sync_enabled(true);
    let mut sfegui = SfEgui::new(&rw);
    let mut demo = DemoWindows::default();
    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            sfegui.add_event(&ev);
            match ev {
                Event::Closed => {
                    rw.close();
                }
                Event::Resized { width, height } => {
                    rw.set_view(
                        &View::from_rect(Rect::new(0., 0., width as f32, height as f32)).unwrap(),
                    );
                }
                _ => {}
            }
        }
        let di = sfegui
            .run(&mut rw, |_rw, ctx| {
                demo.ui(ctx);
                egui::Window::new("EasyMark").show(ctx, |ui| {
                    easy_mark::easy_mark(ui, include_str!("../README.md"));
                });
            })
            .unwrap();
        rw.clear(Color::BLACK);
        sfegui.draw(di, &mut rw, None);
        rw.display();
    }
}
