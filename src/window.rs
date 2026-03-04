pub struct Window {
    window: minifb::Window,
    framebuffer: Framebuffer
}

pub struct Framebuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
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
            window,
            framebuffer: Framebuffer::new(width, height)
        }
    }

    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    // check
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    // updates
    pub fn display(&mut self) {
        self.window.update_with_buffer(
            &self.framebuffer.data,
            self.framebuffer.width,
            self.framebuffer.height
        ).expect("Failed to display");

        let (width, height) = self.window.get_size();
        if width != self.framebuffer.width() || height != self.framebuffer.height() {
            self.framebuffer = Framebuffer::new(width, height);
        }
    }
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer { 
            data:vec![0; width * height], 
            width, 
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.data[x + y * self.width] = value;
    }
}