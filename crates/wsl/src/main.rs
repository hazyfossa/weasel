use std::{env, os::unix::net::UnixStream};

use eyre::{Context, OptionExt, Result};

/// See: https://wsl.dev/technical-documentation/interop/
struct WSLInterop(UnixStream);

impl WSLInterop {
    fn open_current() -> Result<Self> {
        let path = env::var_os("WSL_INTEROP")
            .ok_or_eyre("No interop server can be found from environment")?;

        // TODO: fallback

        let socket = UnixStream::connect(path).context("Cannot connect to socket")?;

        Ok(Self(socket))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(())
}
