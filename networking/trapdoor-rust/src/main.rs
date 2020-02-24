extern crate trapdoor;
pub use trapdoor::*;

#[macro_use]
extern crate log;

fn main() {
    if set_global_logger() {
        info!("Set up logging");
    } else {
        error!("Could not set up logging");
    }

    info!("Attempting to load Netherrack {}", get_version());

    start_server();
}
fn set_global_logger() -> bool {
    false
    // let log_setup_result: Result<(), log::SetLoggerError> = log::set_logger(|max_log_level| {
    //     max_log_level.set(log::LogLevelFilter::Trace);
    //     Box::new(core::logger::NetherrackLogger)
    // });
    //
    // if log_setup_result.is_ok() {
    //     return true;
    // } else {
    //     return false;
    // }

}
