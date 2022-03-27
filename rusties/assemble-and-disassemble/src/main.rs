use std::convert::TryInto;

fn assemble(messages: &mut Vec<Vec<u8>>) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(messages.iter().map(Vec::len).sum());

    let num_messages = messages.len();

    buffer.push(num_messages as u8);

    for msg in messages {
        let msg_len = msg.len() as u16;
        let msg_len_bytes = msg_len.to_be_bytes();

        buffer.push(msg_len_bytes[0]);
        buffer.push(msg_len_bytes[1]);
        buffer.append(msg);
    }

    buffer
}

fn disassemble(buffer: Vec<u8>) -> Vec<Vec<u8>> {
    let mut messages = vec![];

    let num_messages = buffer[0];
    let mut current_index = 1usize; // 1 because we skip first byte which is "num_messages"

    for _ in 0..num_messages {
        let msg_length = u16::from_be_bytes(
            buffer[current_index..=current_index + 1]
                .try_into()
                .unwrap(),
        ) as usize;

        current_index += 2; // skip 2 bytes

        let msg = buffer[current_index..current_index + msg_length].to_vec();

        current_index += msg_length; // skip message length

        messages.push(msg);
    }

    messages
}

fn main() {
    let mut messages: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3],
    ];

    let buffer = assemble(&mut messages);

    println!("{:?}", buffer);

    let disassembled = disassemble(buffer);

    println!("{:?}", disassembled);
}
