use they::BootParameters;
use they::interface::window;
use they::system::System;

fn main() {
    let boot_params = BootParameters::new(None);
    let system = System::new(boot_params);
    window::run(system).ok();
}
