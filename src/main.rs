use pcsc::{Context, Scope, Error, ShareMode, Protocols, MAX_BUFFER_SIZE};

fn main() {
    let context = Context::establish(Scope::User).expect("Failed to establish context");

    let mut readers_buffer = [0; 2048];
    let mut readers = context.list_readers(&mut readers_buffer).expect("Failed to list readers");

    let reader = readers.next().expect("No readers are connected");

    println!("Using reader: {:?}", reader);

    let card = match context.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => card,
        Err(Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };

    // ASCII: (nul)ñ(eot)(nul)(nl)á(nul)(nul)(nul)b(etx)(soh)(np)(ack)(soh)
    // not that it means something...
    let apdu_command = [0x00, 0xA4, 0x04, 0x00, 0x0A, 0xA0, 0x00, 0x00, 0x00, 0x62, 0x03, 0x01, 0x0C, 0x06, 0x01];
    println!("APDU command: {:?}", apdu_command);

    let mut apdu_response_buffer = [0; MAX_BUFFER_SIZE];
    let apdu_response = card.transmit(&apdu_command, &mut apdu_response_buffer).expect("Failed to transmit APDU command to card");

    // [106, 30]
    // ASCII: j(rs)
    // also, don't know yet if this means something in ASCII
    println!("APDU response: {:?}", apdu_response);
}
