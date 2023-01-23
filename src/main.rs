fn hamming_encode(data: &[u8]) -> Vec<u8> {
    let mut encoded = vec![0; data.len() * 8];
    let mut encoded_idx = 0;

    for i in 0..data.len() {
        let mut value = data[i];
        for _ in 0..8 {
            encoded[encoded_idx] = (value & 1) as u8;
            encoded_idx += 1;
            value >>= 1;
        }
    }

    encoded
}



fn hamming_decode(data: &[u8]) -> Vec<u8> {
    let mut decoded = vec![0; (data.len() + 1) / 2];
    let mut decoded_idx = 0;
    let mut value = 0;
    let mut value_idx = 0;

    for i in 0..data.len() {
        value |= (data[i] as u8) << value_idx;
        value_idx += 1;
        if value_idx == 8 {
            decoded[decoded_idx] = value;
            decoded_idx += 1;
            value = 0;
            value_idx = 0;
        }
    }

    decoded
}


fn main() {
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10];
    let encoded = hamming_encode(&data);
    let decoded = hamming_decode(&encoded);

    println!("data: {:?}", data);
    println!("encoded: {:?}", encoded);
    println!("decoded: {:?}", decoded);
}

