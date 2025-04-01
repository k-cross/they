use they::BootParameters;
use they::interface::window;
use they::system::System;

fn main() {
    let boot_params = BootParameters::new(None);
    let mut system = System::new(boot_params);
    system.initialize();
    system.run();
    window::run(system).ok();
}
