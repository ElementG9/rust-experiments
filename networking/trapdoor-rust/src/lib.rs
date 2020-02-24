#[macro_use]
extern crate log;
use semver::Version;

pub mod io;

// The version of the Minecraft protocol accepted by Netherrack
pub const MINECRAFT_PROTOCOL_VERSION: u32 = 47;

// The version of Netherrack as determined at compile time
pub const NETHERRACK_VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");

// Gets the current version of Netherrack
pub fn get_version() -> Version {
    Version::parse(NETHERRACK_VERSION_STRING).unwrap()
}

// Starts a new Netherrack server instance
pub fn start_server() {
    info!("Netherrack startup called");

    trace!("Starting network in a new thread");

    std::thread::spawn(move || {
        io::network::start_network();
    });

    debug!("Networking should be set up");

    loop {
        // TODO: In the future, execute a tick. Use Duration's span function to get time taken, sleep the remainder, and go again
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
