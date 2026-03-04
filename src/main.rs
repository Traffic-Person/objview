mod window;
use window::Window;

fn main() {
    let mut window = Window::new("hm.", 512, 512);

    while window.is_open() {
        window.update();
    }
}
