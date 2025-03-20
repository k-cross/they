use they::motherboard::Motherboard;

fn main() {
    let mb = Motherboard::new();
    println!("{:#?}", mb.cpu.registers);
}
