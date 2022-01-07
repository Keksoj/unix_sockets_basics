use std::os::unix::net::{UnixListener, UnixStream};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    if std::fs::metadata(socket_path).is_ok() {
        println!("A socket is already present. Deleting...");
        std::fs::remove_file(socket_path)
            .with_context(|| format!("could not delete previous socket at {:?}", socket_path))?;
    }

    let unix_listener =
        UnixListener::bind(socket_path).context("Could not create the unix socket")?;

    let (unix_stream, socket_address) = unix_listener
        .accept()
        .context("Failed at accepting a connection on the unix listener")?;

    Ok(())
}
