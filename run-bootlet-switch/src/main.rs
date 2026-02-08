use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
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
