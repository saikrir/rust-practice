mod run_length_encoding {

    pub fn encode(text: &str) -> String {
        let mut final_str = String::new();
        let mut previousChar: Option<char> = None;
        let mut charCnt = 0;

        for ch in text.chars() {
            // if prev cnt is None, increment charCnt = 1, set prevChar = currentChar
            if let Some(prev_ch) = previousChar {
                //you encounterd a char different than previous
                if ch != prev_ch || charCnt == 9 {
                    final_str.push_str(format!("{}{}", charCnt, prev_ch).as_str());
                    charCnt = 0;
                    previousChar = Some(ch);
                }
            }

            previousChar = Some(ch);
            charCnt += 1;
        }
        if previousChar.is_some() {
            final_str.push_str(format!("{}{}", charCnt, previousChar.unwrap()).as_str());
        }

        final_str
    }

    pub fn decode(text: &str) -> String {
        let len = text.len();
        if len % 2 != 0 {
            panic!("invalid length");
        }

        let mut final_str = String::new();
        let mut chars = text.chars();
        while let (Some(count), Some(ch)) = (chars.next(), chars.next()) {
            for _ in 0..count.to_digit(10).unwrap() {
                final_str.push(ch);
            }
        }
        final_str
    }
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
