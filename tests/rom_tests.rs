use std::path::Path;
use they::{BootParameters, system::System};

fn setup(p: Option<&str>) -> System {
    let p = match p {
        Some(s) => Path::new(s).to_path_buf(),
        None => Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("rom_tests/blarggs-test-roms/cpu_instrs/cpu_instrs.gb"),
    };
    let bp = BootParameters::new(p.to_str());
    let mut system = System::new(bp);
    system.initialize();
    system
}

#[test]
fn blarggs_cpu_ld_test() {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("rom_tests/blarggs-test-roms/cpu_instrs/individual/06-ld r,r.gb");
    let mut system = setup(p.to_str());
    eprintln!("ram {:#?}", system);
    system.run();
}
