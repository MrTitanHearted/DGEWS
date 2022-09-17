use crate::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct WindowBuilder {
    pub(crate) title: String,
    pub(crate) icon: String,
    pub(crate) pos: Point,
    pub(crate) size: Size,
    pub(crate) resizable: bool,
    pub(crate) theme: Theme,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        return Self {
            title: String::from("Direct Game Engine Window"),
            icon: String::default(),
            pos: Point::default(),
            size: Size::new(800, 640),
            resizable: false,
            theme: Theme::default(),
        };
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub fn get_icon(&self) -> String {
        return self.icon.clone();
    }

    pub fn title(&self) -> *const u16 {
        return wchar(&self.title);
    }

    pub fn icon(&self) -> *const u16 {
        return wchar(&self.icon);
    }

    pub fn get_pos(&self) -> Point {
        return self.pos;
    }

    pub fn get_x(&self) -> i32 {
        return self.pos.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.pos.y;
    }

    pub fn get_size(&self) -> Size {
        return self.size;
    }

    pub fn get_width(&self) -> i32 {
        return self.size.width;
    }

    pub fn get_height(&self) -> i32 {
        return self.size.height;
    }

    pub fn get_theme(&self) -> Theme {
        return self.theme;
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        return self;
    }

    pub fn with_icon(mut self, path: &str) -> Self {
        self.icon = path.to_owned();
        return self;
    }

    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Point::new(x, y);
        return self;
    }

    pub fn with_dimensions(mut self, width: i32, height: i32) -> Self {
        self.size = Size::new(width, height);
        return self;
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        return self;
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        return self;
    }

    pub fn is_resizable(&self) -> bool {
        return self.resizable;
    }
}
