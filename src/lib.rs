#[derive(Debug)]
pub struct APDUCommand {
    pub class: u8,
    pub instruction: u8,
    pub parameter1: u8,
    pub parameter2: u8,
    pub data: Vec<u8>,
    pub expected_response_bytes: u8,
}

impl From<APDUCommand> for Vec<u8> {
    fn from(apdu: APDUCommand) -> Self {
        let mut v = vec![apdu.class, apdu.instruction, apdu.parameter1, apdu.parameter2];
        let mut data = apdu.data;
        v.append(&mut data);
        v.push(apdu.expected_response_bytes);
        v
    }
}
