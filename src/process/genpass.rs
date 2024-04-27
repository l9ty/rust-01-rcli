use rand::seq::SliceRandom;

const UPPERCASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!#$%&*";

pub fn process_genpass(
    length: u8,
    number: bool,
    symbol: bool,
    uppercase: bool,
    lowercase: bool,
) -> anyhow::Result<String> {
    let mut password = Vec::with_capacity(length as usize);
    let chars_len: usize = UPPERCASE.len() + LOWERCASE.len() + NUMBERS.len() + SYMBOLS.len();
    let mut chars = Vec::with_capacity(chars_len);
    let mut rng = rand::thread_rng();

    if number {
        chars.extend_from_slice(NUMBERS);
        let char = NUMBERS.choose(&mut rng).unwrap();
        password.push(*char);
    }

    if symbol {
        chars.extend_from_slice(SYMBOLS);
        let char = SYMBOLS.choose(&mut rng).unwrap();
        password.push(*char);
    }

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        let char = UPPERCASE.choose(&mut rng).unwrap();
        password.push(*char);
    }

    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        let char = LOWERCASE.choose(&mut rng).unwrap();
        password.push(*char);
    }

    for _ in 0..(length - password.len() as u8) {
        let char = chars.choose(&mut rng).unwrap();
        password.push(*char);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    anyhow::Ok(password)
}
