// Connections between the client and Trapdoor.
extern crate chrono;

use self::chrono::duration::Duration;
use std::net::{Shutdown, TcpStream};
use ::io_operations::reader::Reader;

// An enum describing the current connection's state.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ConnectionState {
    HANDSHAKE,
    LOGIN,
    STATUS,
    PLAY,
}

// A struct defining a game connection
#[derive(Debug)]
pub struct GameConnection {
    // The TCP stream bound to this GameConnection
    pub stream: TcpStream,

    // The state of this GameConnection
    pub state: ConnectionState,

    /// A boolean declaring if this GameConnection is connected
    pub connected: bool,
}
impl GameConnection {
    pub fn new(stream: TcpStream) -> GameConnection {
        GameConnection {
            stream: stream,
            state: ConnectionState::HANDSHAKE,
            connected: true,
        }
    }
    pub fn disconnect(&mut self) {
        let result = self.stream.shutdown(Shutdown::Both);
        if !result.is_ok() {
            error!("Error disconnecting: {:?}", result);
        }
        self.connected = false;
        ::std::thread::yield_now();
    }
    pub fn start_listening(&mut self) {
        trace!(
            "Got a new client from {}!",
            self.stream.peer_addr().unwrap()
        );

        let mut timeout_duration = Duration::zero();

        loop {
            // First, let's start off the timeout timer.
            timeout_duration = timeout_duration + Duration::milliseconds(2);
            ::std::thread::sleep(::std::time::Duration::from_millis(2));

            if timeout_duration.num_seconds() >= 1 {
                info!(
                    "Client at {} timed out, dropping connection",
                    self.stream.peer_addr().unwrap()
                );
                self.disconnect();
                break;
            }

            if !self.connected {
                debug!(
                    "Client at {} was forcibly disconnected, maybe for a good reason",
                    self.stream.peer_addr().unwrap()
                );
                break;
            }

            match self.stream.read_unsigned_byte() {
                Err(error) => {
                    if error != "Could not read one byte" {
                        error!("Error: {}", error);
                        self.disconnect();
                    }
                }

                Ok(value) => {
                    timeout_duration = Duration::zero();
                    info!("Value is {}", value);
                }
            }
        }
    }
}
