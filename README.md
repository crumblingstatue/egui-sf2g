# egui-sf2g

[![Crates.io](https://img.shields.io/crates/v/egui-sf2g)](https://crates.io/crates/egui-sf2g)
[![docs.rs](https://img.shields.io/docsrs/egui-sf2g?style=plastic)](https://docs.rs/egui-sf2g)

[sf2g](https://github.com/crumblingstatue/sf2g) integration for [egui](https://github.com/emilk/egui).

This library allows using egui for sf2g projects.
It's a very easy way to add a functional gui to your sf2g game or application!

All you need to do is:
- Create an `SfEgui`
- Feed it SFML events using `add_event`
- Do an egui frame with `do_frame`
- Draw the ui with `draw`

See `examples/hello.rs` for a simple demo.
