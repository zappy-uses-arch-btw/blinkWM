use x11rb::protocol::xproto::Window;

#[derive(Debug, Clone)]
pub struct ManagedWindow {
    pub id: Window,
    pub frame: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub floating: bool,
    pub workspace: usize,
}

impl ManagedWindow {
    pub fn new(id: Window, frame: Window, x: i16, y: i16, width: u16, height: u16) -> Self {
        Self {
            id,
            frame,
            x,
            y,
            width,
            height,
            floating: false,
            workspace: 1,
        }
    }
}
