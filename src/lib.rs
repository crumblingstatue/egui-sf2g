//! egui SFML integration helpers
//!
//! Contains various types and functions that helps with integrating egui with SFML.

#![warn(missing_docs)]

mod rendering;

pub use {egui, sf2g};
use {
    egui::{
        Context, CursorIcon, Modifiers, PointerButton, Pos2, RawInput, TextureId, ViewportCommand,
    },
    sf2g::{
        cpp::FBox,
        graphics::{RenderTarget as _, RenderWindow, Texture},
        system::{Clock, Vector2, Vector2i},
        window::{Cursor, CursorType, Event, Key, mouse},
    },
    std::collections::HashMap,
};

fn button_conv(button: mouse::Button) -> Option<PointerButton> {
    let but = match button {
        mouse::Button::Left => PointerButton::Primary,
        mouse::Button::Right => PointerButton::Secondary,
        mouse::Button::Middle => PointerButton::Middle,
        _ => return None,
    };
    Some(but)
}

fn key_conv(code: Key) -> Option<egui::Key> {
    use egui::Key as EKey;
    Some(match code {
        Key::Down => EKey::ArrowDown,
        Key::Left => EKey::ArrowLeft,
        Key::Right => EKey::ArrowRight,
        Key::Up => EKey::ArrowUp,
        Key::Escape => EKey::Escape,
        Key::Tab => EKey::Tab,
        Key::Backspace => EKey::Backspace,
        Key::Enter => EKey::Enter,
        Key::Space => EKey::Space,
        Key::Insert => EKey::Insert,
        Key::Delete => EKey::Delete,
        Key::Home => EKey::Home,
        Key::End => EKey::End,
        Key::PageUp => EKey::PageUp,
        Key::PageDown => EKey::PageDown,
        Key::LBracket => EKey::OpenBracket,
        Key::RBracket => EKey::CloseBracket,
        Key::Num0 => EKey::Num0,
        Key::Num1 => EKey::Num1,
        Key::Num2 => EKey::Num2,
        Key::Num3 => EKey::Num3,
        Key::Num4 => EKey::Num4,
        Key::Num5 => EKey::Num5,
        Key::Num6 => EKey::Num6,
        Key::Num7 => EKey::Num7,
        Key::Num8 => EKey::Num8,
        Key::Num9 => EKey::Num9,
        Key::A => EKey::A,
        Key::B => EKey::B,
        Key::C => EKey::C,
        Key::D => EKey::D,
        Key::E => EKey::E,
        Key::F => EKey::F,
        Key::G => EKey::G,
        Key::H => EKey::H,
        Key::I => EKey::I,
        Key::J => EKey::J,
        Key::K => EKey::K,
        Key::L => EKey::L,
        Key::M => EKey::M,
        Key::N => EKey::N,
        Key::O => EKey::O,
        Key::P => EKey::P,
        Key::Q => EKey::Q,
        Key::R => EKey::R,
        Key::S => EKey::S,
        Key::T => EKey::T,
        Key::U => EKey::U,
        Key::V => EKey::V,
        Key::W => EKey::W,
        Key::X => EKey::X,
        Key::Y => EKey::Y,
        Key::Z => EKey::Z,
        Key::F1 => EKey::F1,
        Key::F2 => EKey::F2,
        Key::F3 => EKey::F3,
        Key::F4 => EKey::F4,
        Key::F5 => EKey::F5,
        Key::F6 => EKey::F6,
        Key::F7 => EKey::F7,
        Key::F8 => EKey::F8,
        Key::F9 => EKey::F9,
        Key::F10 => EKey::F10,
        Key::F11 => EKey::F11,
        Key::F12 => EKey::F12,
        Key::Equal => EKey::Equals,
        Key::Hyphen => EKey::Minus,
        Key::Slash => EKey::Slash,
        Key::Tilde => EKey::Backtick,
        _ => return None,
    })
}

fn modifier(alt: bool, ctrl: bool, shift: bool) -> egui::Modifiers {
    egui::Modifiers {
        alt,
        ctrl,
        shift,
        command: ctrl,
        mac_cmd: false,
    }
}

/// Converts an SFML event to an egui event and adds it to the `RawInput`.
fn handle_event(
    raw_input: &mut egui::RawInput,
    event: &sf2g::window::Event,
    clipboard: &mut arboard::Clipboard,
) {
    match *event {
        Event::KeyPressed {
            code,
            alt,
            ctrl,
            shift,
            system: _,
            scan: _,
        } => {
            if ctrl {
                match code {
                    Key::V => match clipboard.get_text() {
                        Ok(text) => raw_input.events.push(egui::Event::Text(text)),
                        Err(e) => {
                            eprintln!("[egui-sf2g] Paste failed: {e}");
                        }
                    },
                    Key::C => raw_input.events.push(egui::Event::Copy),
                    Key::X => raw_input.events.push(egui::Event::Cut),
                    _ => {}
                }
            }
            if let Some(key) = key_conv(code) {
                raw_input.events.push(egui::Event::Key {
                    key,
                    modifiers: modifier(alt, ctrl, shift),
                    pressed: true,
                    repeat: false,
                    physical_key: None,
                });
            }
        }
        Event::KeyReleased {
            code,
            alt,
            ctrl,
            shift,
            system: _,
            scan: _,
        } => {
            if let Some(key) = key_conv(code) {
                raw_input.events.push(egui::Event::Key {
                    key,
                    modifiers: modifier(alt, ctrl, shift),
                    pressed: false,
                    repeat: false,
                    physical_key: None,
                });
            }
        }
        Event::MouseMoved { x, y } => {
            raw_input
                .events
                .push(egui::Event::PointerMoved(Pos2::new(x as f32, y as f32)));
        }
        Event::MouseButtonPressed { x, y, button } => {
            if let Some(button) = button_conv(button) {
                raw_input.events.push(egui::Event::PointerButton {
                    pos: Pos2::new(x as f32, y as f32),
                    button,
                    pressed: true,
                    modifiers: Modifiers::default(),
                });
            }
        }
        Event::MouseButtonReleased { x, y, button } => {
            if let Some(button) = button_conv(button) {
                raw_input.events.push(egui::Event::PointerButton {
                    pos: Pos2::new(x as f32, y as f32),
                    button,
                    pressed: false,
                    modifiers: Modifiers::default(),
                });
            }
        }
        Event::TextEntered { unicode } => {
            if !unicode.is_control() {
                raw_input
                    .events
                    .push(egui::Event::Text(unicode.to_string()));
            }
        }
        Event::MouseWheelScrolled { delta, .. } => {
            if sf2g::window::Key::LControl.is_pressed() {
                raw_input
                    .events
                    .push(egui::Event::Zoom(if delta > 0.0 { 1.1 } else { 0.9 }));
            } else {
                raw_input.events.push(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Line,
                    delta: egui::vec2(0.0, delta),
                    modifiers: egui::Modifiers::default(),
                });
            }
        }
        Event::Resized { width, height } => {
            raw_input.screen_rect = Some(raw_input_screen_rect(width, height));
        }
        _ => {}
    }
}

/// Creates a `RawInput` that fits the window.
fn make_raw_input(window: &RenderWindow) -> RawInput {
    let Vector2 { x: w, y: h } = window.size();
    RawInput {
        screen_rect: Some(raw_input_screen_rect(w, h)),
        max_texture_side: Some(Texture::maximum_size() as usize),
        ..Default::default()
    }
}

fn raw_input_screen_rect(w: u32, h: u32) -> egui::Rect {
    egui::Rect {
        min: Pos2::new(0., 0.),
        max: Pos2::new(w as f32, h as f32),
    }
}

/// A source for egui user textures.
///
/// You can create a struct that contains all the necessary information to get a user texture from
/// an id, and implement this trait for it.
pub trait UserTexSource {
    /// Get the texture that corresponds to `id`.
    ///
    /// Returns (width, height, texture).
    fn get_texture(&mut self, id: u64) -> (f32, f32, &Texture);
}

/// A dummy texture source in case you don't care about providing user textures
struct DummyTexSource {
    tex: FBox<Texture>,
}

impl Default for DummyTexSource {
    fn default() -> Self {
        Self {
            tex: Texture::new().unwrap(),
        }
    }
}

impl UserTexSource for DummyTexSource {
    fn get_texture(&mut self, _id: u64) -> (f32, f32, &Texture) {
        (0., 0., &self.tex)
    }
}

type TextureMap = HashMap<TextureId, FBox<Texture>>;

/// `Egui` integration for SFML.
pub struct SfEgui {
    clock: FBox<Clock>,
    ctx: Context,
    raw_input: RawInput,
    textures: TextureMap,
    last_window_pos: Vector2i,
    cursors: Cursors,
    clipboard: arboard::Clipboard,
}

struct Cursors {
    arrow: FBox<Cursor>,
    horizontal: FBox<Cursor>,
    vertical: FBox<Cursor>,
    hand: FBox<Cursor>,
    cross: FBox<Cursor>,
    text: FBox<Cursor>,
}

impl Default for Cursors {
    fn default() -> Self {
        Self {
            arrow: Cursor::from_system(CursorType::Arrow).unwrap(),
            horizontal: Cursor::from_system(CursorType::SizeHorizontal).unwrap(),
            vertical: Cursor::from_system(CursorType::SizeVertical).unwrap(),
            hand: Cursor::from_system(CursorType::Hand).unwrap(),
            cross: Cursor::from_system(CursorType::Cross).unwrap(),
            text: Cursor::from_system(CursorType::Text).unwrap(),
        }
    }
}

/// Data required to draw the egui ui
pub struct DrawInput {
    shapes: Vec<egui::epaint::ClippedShape>,
    pixels_per_point: f32,
}

impl SfEgui {
    /// Create a new `SfEgui`.
    ///
    /// The size of the egui ui will be the same as `window`'s size.
    pub fn new(window: &RenderWindow) -> Self {
        Self {
            clock: sf2g::system::Clock::start().unwrap(),
            raw_input: make_raw_input(window),
            ctx: Context::default(),
            textures: TextureMap::default(),
            last_window_pos: Vector2i::default(),
            cursors: Cursors::default(),
            clipboard: arboard::Clipboard::new().unwrap(),
        }
    }
    /// Convert an SFML event into an egui event and add it for later use by egui.
    ///
    /// Call this in an event polling loop for each event.
    pub fn add_event(&mut self, event: &Event) {
        handle_event(&mut self.raw_input, event, &mut self.clipboard);
    }
    /// Does a [`egui::Context::run`] to run your egui ui.
    ///
    /// This supports egui uis that depend on multiple passes.
    ///
    /// See [`egui::Context::request_discard`].
    ///
    /// The `f` parameter is a user supplied ui function that does the desired ui
    pub fn run(
        &mut self,
        rw: &mut RenderWindow,
        mut f: impl FnMut(&mut RenderWindow, &Context),
    ) -> Result<DrawInput, PassError> {
        self.prepare_raw_input();
        let out = self.ctx.run(self.raw_input.take(), |ctx| f(rw, ctx));
        self.handle_output(
            rw,
            out.platform_output,
            out.textures_delta,
            out.viewport_output,
        )?;
        Ok(DrawInput {
            shapes: out.shapes,
            pixels_per_point: out.pixels_per_point,
        })
    }

    /// Begins a (single) egui pass.
    ///
    /// This does not support egui uis that depend on multiple passes.
    /// Use [`Self::run`] for that.
    ///
    /// If you call this, it should be paired with [`Self::end_pass`].
    pub fn begin_pass(&mut self) {
        self.prepare_raw_input();
        self.ctx.begin_pass(self.raw_input.take());
    }

    /// Ends an egui pass. Call [`Self::begin_pass`] first.
    pub fn end_pass(&mut self, rw: &mut RenderWindow) -> Result<DrawInput, PassError> {
        let out = self.ctx.end_pass();
        self.handle_output(
            rw,
            out.platform_output,
            out.textures_delta,
            out.viewport_output,
        )?;
        Ok(DrawInput {
            shapes: out.shapes,
            pixels_per_point: out.pixels_per_point,
        })
    }

    fn handle_output(
        &mut self,
        rw: &mut RenderWindow,
        platform_output: egui::PlatformOutput,
        textures_delta: egui::TexturesDelta,
        viewport_output: egui::ViewportIdMap<egui::ViewportOutput>,
    ) -> Result<(), PassError> {
        for (id, delta) in &textures_delta.set {
            let tex = self
                .textures
                .entry(*id)
                .or_insert_with(|| Texture::new().unwrap());
            rendering::update_tex_from_delta(tex, delta)?;
        }
        for id in &textures_delta.free {
            self.textures.remove(id);
        }
        let new_cursor = match platform_output.cursor_icon {
            CursorIcon::Default => Some(&self.cursors.arrow),
            CursorIcon::None => None,
            CursorIcon::PointingHand | CursorIcon::Grab | CursorIcon::Grabbing => {
                Some(&self.cursors.hand)
            }
            CursorIcon::Crosshair => Some(&self.cursors.cross),
            CursorIcon::Text => Some(&self.cursors.text),
            CursorIcon::ResizeHorizontal | CursorIcon::ResizeColumn => {
                Some(&self.cursors.horizontal)
            }
            CursorIcon::ResizeVertical => Some(&self.cursors.vertical),
            _ => Some(&self.cursors.arrow),
        };
        match new_cursor {
            Some(cur) => {
                rw.set_mouse_cursor_visible(true);
                unsafe {
                    rw.set_mouse_cursor(cur);
                }
            }
            None => rw.set_mouse_cursor_visible(false),
        }
        for cmd in platform_output.commands {
            match cmd {
                egui::OutputCommand::CopyText(txt) => {
                    if let Err(e) = self.clipboard.set_text(txt) {
                        eprintln!("[egui-sf2g] Failed to set clipboard text: {e}");
                    }
                }
                egui::OutputCommand::CopyImage(_img) => {
                    eprintln!("egui-sf2g: Unimplemented image copy");
                }
                egui::OutputCommand::OpenUrl(_url) => {
                    eprintln!("egui-sf2g: Unimplemented url open");
                }
            }
        }
        // TODO: Multi-viewport support
        for (_, out) in viewport_output {
            for cmd in out.commands {
                match cmd {
                    ViewportCommand::Close => rw.close(),
                    ViewportCommand::Title(s) => rw.set_title(&s),
                    ViewportCommand::Visible(visible) => {
                        if !visible {
                            self.last_window_pos = rw.position();
                        }
                        rw.set_visible(visible);
                        if visible {
                            rw.set_position(self.last_window_pos);
                        }
                    }
                    ViewportCommand::Focus => {
                        // This trick forces focus where `request_focus` would
                        // only flash the tray icon.
                        let rw_pos = rw.position();
                        rw.set_visible(false);
                        rw.set_visible(true);
                        rw.set_position(rw_pos);
                    }
                    _ => eprintln!("egui_sf2g: Unhandled ViewportCommand: {cmd:?}"),
                }
            }
        }
        Ok(())
    }

    fn prepare_raw_input(&mut self) {
        self.raw_input.time = Some(self.clock.elapsed_time().as_seconds() as f64);
        // Update modifiers every frame, otherwise querying them (input.modifiers.*) doesn't seem
        // up-to-date
        self.raw_input.modifiers.alt = Key::LAlt.is_pressed() || Key::RAlt.is_pressed();
        self.raw_input.modifiers.ctrl = Key::LControl.is_pressed() || Key::RControl.is_pressed();
        self.raw_input.modifiers.shift = Key::LShift.is_pressed() || Key::RShift.is_pressed();
    }
    /// Draw the ui to a `RenderWindow`.
    ///
    /// Takes an optional [`UserTexSource`] to act as a user texture source.
    pub fn draw(
        &mut self,
        input: DrawInput,
        window: &mut RenderWindow,
        user_tex_src: Option<&mut dyn UserTexSource>,
    ) {
        rendering::draw(
            window,
            &self.ctx,
            input.shapes,
            user_tex_src.unwrap_or(&mut DummyTexSource::default()),
            &self.textures,
            input.pixels_per_point,
        )
    }
    /// Returns a handle to the egui context
    pub fn context(&self) -> &Context {
        &self.ctx
    }
}

#[derive(Debug)]
/// Error when failing to create a texture
pub struct TextureCreateError {
    /// The width of the requested texture
    pub width: usize,
    /// The height of the requested texture
    pub height: usize,
}

impl std::fmt::Display for TextureCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = (self.width, self.height);
        f.write_fmt(format_args!(
            "Failed to create texture of size {width}x{height}"
        ))
    }
}

/// Error that can happen during an egui pass
#[non_exhaustive]
#[derive(Debug)]
pub enum PassError {
    /// Failed to create a texture
    TextureCreateError(TextureCreateError),
}

impl From<TextureCreateError> for PassError {
    fn from(src: TextureCreateError) -> Self {
        Self::TextureCreateError(src)
    }
}

impl std::fmt::Display for PassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PassError::TextureCreateError(e) => {
                f.write_fmt(format_args!("Texture create error: {e}"))
            }
        }
    }
}

impl std::error::Error for PassError {}
