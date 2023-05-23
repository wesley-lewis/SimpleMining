use sha256::digest;

fn main() {
    let mut input: String ;

    let mut nonce: u32 = 0;
    let mut first_four_char: &str;
    loop {    
        input = "hi".to_string();
        let nonce_string = nonce.to_string();
        let first_four_char: &str;
        input.push_str(&nonce_string[..]);
        let value = digest(&input[..]); 
        first_four_char = &value[0..5];
        
        if first_four_char == "00000" {
            break;
        }

        if nonce % 1000 == 0 {
            println!("{}: {}", nonce, first_four_char);
        }
        nonce = nonce + 1;
    }
    let value = digest(&input[..]);

    println!("Input: {} | Value: {}",&input[..], value);
}
