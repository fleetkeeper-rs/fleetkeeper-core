fn main() {
    let hex = std::env::args().nth(1).expect("give a 64 character hex string");
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("not valid hex"))
        .collect();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes);
    println!("{}", stellar_strkey::Contract(array).to_string().as_str().to_string());
}
