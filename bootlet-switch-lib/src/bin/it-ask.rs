use bootlet_switch_lib::get_switch_state;

fn main() {
    let value = get_switch_state().unwrap();

    println!("Switch state: {}", value);
}
