extern crate yew;
extern crate iomentum;

use iomentum::MountError;

fn main() -> Result<(), MountError> {
    yew::initialize();
    iomentum::mount_app()?;
    yew::run_loop();
    Ok(())
}