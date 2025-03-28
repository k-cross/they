use they::interface::window;
use they::motherboard::Motherboard;

fn main() {
    let mb = Motherboard::new();
    window::run(mb).ok();
}
