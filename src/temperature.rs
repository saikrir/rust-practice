pub enum Scale {
    Celsius,
    Fahrenheit,
}

pub struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    pub fn new(degrees: f32, scale: Scale) -> Self {
        Self { degrees, scale }
    }

    pub fn to_celcius(&self) -> f32 {
        match &self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32.0) * (5.0 / 9.0),
        }
    }

    pub fn to_fahrenheit(&self) -> f32 {
        match &self.scale {
            Scale::Fahrenheit => self.degrees,
            Scale::Celsius => ((9.0 / 5.0) * self.degrees) + 32.0,
        }
    }
}

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0, Scale::Celsius);
    assert!((cold.to_fahrenheit() - 33.8) < 0.01);
    assert!((cold.to_fahrenheit() - 33.8) >= 0.0);
}
