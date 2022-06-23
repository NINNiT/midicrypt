use clap::{Arg, Command};
use std::path::Path;

mod cli;
mod crypto;
mod midi;

fn main() {
    // let cli = CliArgs::parse();
    let matches = Command::new("midicrypt")
        .version("0.1.0")
        .author("NINNiT")
        .about("Encrypts and Decrypts Files using MIDI")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file")
                .arg(
                    Arg::with_name("INPUT")
                        .help("The file to encrypt")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("OUTPUT")
                        .help("The file to write the encrypted file to")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("PORT")
                        .short('p')
                        .long("port")
                        .help("The MIDI port to use")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file")
                .arg(
                    Arg::with_name("INPUT")
                        .help("The file to decrypt")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("OUTPUT")
                        .help("The file to write the decrypted file to")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("PORT")
                        .short('p')
                        .long("port")
                        .help("The MIDI port to use")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(Command::new("list-ports").about("Lists available MIDI ports"))
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let input_path = Path::new(sub_matches.value_of("INPUT").unwrap());
            let output_path = Path::new(sub_matches.value_of("OUTPUT").unwrap());
            let port = sub_matches.value_of("PORT").unwrap();

            let mut midi_input = midi::create_midi_input();

            let port = midi::get_input_port_by_name(
                "Impact LX61+:Impact LX61+ MIDI1 28:0",
                &mut midi_input,
            );

            let bytes = midi::read_midi_input_from_port(&port);
            let hash = crypto::hash_bytes_sha256(bytes);
            crypto::encrypt_file(input_path, output_path, hash).unwrap();
        }
        Some(("decrypt", sub_matches)) => {
            let input_path = Path::new(sub_matches.value_of("INPUT").unwrap());
            let output_path = Path::new(sub_matches.value_of("OUTPUT").unwrap());
            let port = sub_matches.value_of("PORT").unwrap();

            let mut midi_input = midi::create_midi_input();

            let port = midi::get_input_port_by_name(
                "Impact LX61+:Impact LX61+ MIDI1 28:0",
                &mut midi_input,
            );

            let bytes = midi::read_midi_input_from_port(&port);
            let hash = crypto::hash_bytes_sha256(bytes);
            crypto::decrypt_file(input_path, output_path, hash).unwrap();
        }
        Some(("list-ports", _)) => {
            let ports = midi::read_available_port_names();

            // print port_name to stdout
            for port in ports {
                println!("{}", port);
            }
        }
        _ => return,
    }
}
