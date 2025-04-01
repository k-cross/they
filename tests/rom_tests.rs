use they::{BootParameters, system::System};

fn setup() -> System {
    let bp = BootParameters::new(None);
    let mut system = System::new(bp);
    system.initialize();
    system
}

#[test]
fn blarggs_cpu_instr_test() {
    eprintln!("Starting Blarggs CPU Test");
    let system = setup();
    eprintln!("ram {:#?}", system);
    //system.run();
}
