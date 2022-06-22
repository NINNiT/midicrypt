use midir::{Ignore, MidiInput};

pub fn read_available_ports() -> Vec<String> {
    // create new MIDI input
    let mut midi_input = MidiInput::new("midicrypt_input").unwrap();
    // ignore nothing (could be time, sensitivity,...)
    midi_input.ignore(Ignore::None);

    let in_ports = midi_input.ports();

    in_ports
        .iter()
        .map(|port| midi_input.port_name(port).unwrap())
        .collect()
}
