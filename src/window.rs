pub struct Window {
    window: minifb::Window
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(
            name,
            width,
            height,
            options
        ).expect("Window no worky");
    
        Window {
            window
        }
    }

    // check
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    // updates
    pub fn update(&mut self) {
        self.window.update();
    }
}