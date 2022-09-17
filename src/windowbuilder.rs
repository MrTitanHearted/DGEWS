use crate::prelude::*;

/// The WindowBuilder provides required information about the window before creating it in the Manager.
/// 
/// # Example
/// 
/// ```
/// let window_builder = WindowBuilder::new();
/// assert_eq!(&window_builder.get_title(), "Direct Game Engine Window");
/// assert_eq!(&window_builder.get_icon(), "");
/// assert_eq!(window_builder.get_theme(), Theme::default());
/// ```
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
    /// Creates a new instance of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_size(), Size::new(800, 640));
    /// ```
    pub fn new() -> Self {
        return Self::default();
    }

    /// Returns the title of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(&window_builder.get_title(), "Direct Game Engine Window");
    /// ```
    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    /// Return the icon of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(&window_builder.get_icon(), "");
    /// ```
    pub fn get_icon(&self) -> String {
        return self.icon.clone();
    }

    /// Return a pointer to the title of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// func(window_builder.title());
    /// 
    /// // Where func takes a pointer to the title of the WindowBuilder
    /// ```
    pub fn title(&self) -> *const u16 {
        return wchar(&self.title);
    }

    /// Return a pointer to the icon of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// func(window_builder.icon());
    /// 
    /// // Where func takes a pointer to the icon of the WindowBuilder
    /// ```
    pub fn icon(&self) -> *const u16 {
        return wchar(&self.icon);
    }

    /// Returns the position of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_position(), Point::default());
    /// ```
    pub fn get_pos(&self) -> Point {
        return self.pos;
    }

    /// Returns the x position of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_x(), 0);
    /// ```
    pub fn get_x(&self) -> i32 {
        return self.pos.x;
    }

    /// Returns the y position of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_y(), 0);
    /// ```
    pub fn get_y(&self) -> i32 {
        return self.pos.y;
    }
    /// Return the size of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_size(), Size::new(800, 640));
    /// ```
    pub fn get_size(&self) -> Size {
        return self.size;
    }

    /// Returns the width of the WindowBUilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_width(), 800);
    /// ```
    pub fn get_width(&self) -> i32 {
        return self.size.width;
    }

    /// Returns the height of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_height(), 640);
    /// ```
    pub fn get_height(&self) -> i32 {
        return self.size.height;
    }

    /// Returns the theme of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.get_theme(), Theme::default());
    /// ```
    pub fn get_theme(&self) -> Theme {
        return self.theme;
    }

    /// Returns a WindowBuilder with a given title
    /// 
    /// # Example       
    /// 
    /// ```       
    /// let window_builder = WindowBuilder::new().with_title("My Window");
    /// assert_eq!(&window_builder.get_title(), "My Window");
    /// ```
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        return self;
    }

    /// Returns a WindowBuilder with a given icon
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new().with_icon("assets/icon.png");    
    /// assert_eq!(&window_builder.get_icon(), "assets/icon.png");
    /// ```
    pub fn with_icon(mut self, path: &str) -> Self {
        self.icon = path.to_owned();
        return self;
    }

    /// Returns a WindowBuilder with a given position
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new().with_pos(Point::new(60, 50));
    /// assert_eq!(window_builder.get_pos(), Point::new(60, 50));
    /// ```
    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Point::new(x, y);
        return self;
    }

    /// Returns a WindowBuilder with a given size
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new().with_size(Size::new(1024, 768));
    /// assert_eq!(window_builder.get_size(), Size::new(1024, 768));
    /// ```
    pub fn with_dimensions(mut self, width: i32, height: i32) -> Self {
        self.size = Size::new(width, height);
        return self;
    }

    /// Returns a WindowBuilder with a give resizablity
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new().with_resizable(true);
    /// assert_eq!(window_builder.is_resizable(), true);
    /// ```
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        return self;
    }

    /// Returns a WindowBuilder with a given theme
    /// 
    /// # Example
    /// 
    /// ```
    /// let window_builder = WindowBuilder::new().with_theme(Theme::Dark);
    /// assert_eq!(window_builder.get_theme(), Theme::Dark);
    /// ```
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        return self;
    }

    /// Returns the resizablity of the WindowBuilder
    /// 
    /// # Example
    /// 
    /// ```
    /// let mut window_builder = WindowBuilder::new();
    /// assert_eq!(window_builder.is_resizable(), false);
    /// 
    /// window_builder = window_builder.with_resizable(true);
    /// assert_eq!(window_builder.is_resizable(), true);
    /// ```
    pub fn is_resizable(&self) -> bool {
        return self.resizable;
    }
}
