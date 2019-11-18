use rustyboy::cpu::Cpu;
use rustyboy::mbc;

fn main() {
    let mbc = match mbc::get_rom("./rom/firmware.bin") {
        Ok(m) => m,
        Err(error) => panic!("oups {}", error),
    };
    let mut r = Cpu::new(mbc);
    loop {
        r.cycle();
    }
}
