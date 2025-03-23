use std::{fmt::Display, str::FromStr, string::ParseError};

#[derive(Debug)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("#") {
            return Err(String::from("RGB Hex in a bad format"));
        }

        let red = u8::from_str_radix(&s[1..3], 16)
            .or_else(|_| Err(String::from("failed to parse red component")))?;
        let green = u8::from_str_radix(&s[3..5], 16)
            .or_else(|_| Err(String::from("failed to parse green component")))?;
        let blue = u8::from_str_radix(&s[5..7], 16)
            .or_else(|_| Err(String::from("failed to parse blue component")))?;

        Ok(Self { red, blue, green })
    }
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.red
    }

    fn g(&self) -> u8 {
        self.green
    }

    fn b(&self) -> u8 {
        self.blue
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
