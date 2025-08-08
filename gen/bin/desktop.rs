
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    macroquad_survival::start_app();
}
