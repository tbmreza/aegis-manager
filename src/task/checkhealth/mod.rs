pub mod tips;
pub mod vpn;

pub fn run() {
    vpn::run();
    tips::run();
    println!("i am defined in checkhealth/mod.rs");
}
