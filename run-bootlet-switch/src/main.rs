use std::{
    env::args,
    process::{Command, Stdio},
};

fn main() -> anyhow::Result<()> {
    if args().any(|arg| arg == "--wait") {
        bootlet_switch_lib::wait_port(5, 100)?;
    }

    let state = bootlet_switch_lib::get_switch_state()?;

    let target = if state {
        "bootletSwitch1"
    } else {
        "bootletSwitch0"
    };

    println!("State is {}. Booting to {}", state, target);

    Command::new("it")
        .arg(target)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}
