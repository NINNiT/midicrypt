use std::io::stdin;

use midir::{Ignore, MidiInput, MidiInputPort};

pub fn read_available_port_names() -> Vec<String> {
    let midi_input = create_midi_input();
    let in_ports = midi_input.ports();

    in_ports
        .iter()
        .map(|port| midi_input.port_name(port).unwrap())
        .collect()
}

pub fn create_midi_input() -> MidiInput {
    let mut midi_input = MidiInput::new("midicrypt_input").unwrap();
    midi_input.ignore(Ignore::All);

    return midi_input;
}

pub fn get_input_port_by_name(name: &str) -> MidiInputPort {
    let midi_input = MidiInput::new("midicrypt_input").unwrap();
    let in_ports = midi_input.ports();

    for port in in_ports {
        if midi_input.port_name(&port).unwrap() == name {
            return port;
        }
    }

    panic!("No input port with name {} found", name);
}

pub fn read_midi_input_from_port(in_port: &MidiInputPort) -> Vec<u8> {
    let midi_input = create_midi_input();
    let mut cli_input = String::new();
    let messages = Vec::new();

    println!("Input Melody (Press Enter when finished)");

    let _conn_in = midi_input
        .connect(
            in_port,
            "midir-read-input",
            move |_stamp, message, log| {
                print!("♪");
                let mut removed = message.to_vec();
                removed.pop();
                log.extend(removed);
            },
            messages,
        )
        .unwrap();

    cli_input.clear();
    stdin().read_line(&mut cli_input).unwrap();

    let (_, messages) = _conn_in.close();
    println!("Closing connection");

    return messages;
}
