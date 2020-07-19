// Temperature-related code pinched from
// https://blog.mgattozzi.dev/avoiding-logic-errors/
// by Michael Gattozzi<https://blog.mgattozzi.dev/author/michael/>
use std::fmt;
use std::ops::Add;
use Temperature::*;

#[derive(Debug, PartialEq, Copy, Clone)]
/// An enum representing the different units of Temperature
pub enum Temperature {
    Kelvin(f64),
    Celsius(f64),
    Fahrenheit(f64),
}

impl fmt::Display for Temperature {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Kelvin(k) => write!(fmtr, "{}K", k),
            Celsius(c) => write!(fmtr, "{}°C", c),
            Fahrenheit(f) => write!(fmtr, "{}°F", f),
        }
    }
}

impl Temperature {
    /// Convert whatever `Temperature` unit there is into `Celsius`
    pub fn to_celsius(self) -> Temperature {
        match self {
            Kelvin(k) => Celsius(k - 273.15),
            c @ Celsius(_) => c,
            Fahrenheit(f) => Celsius((f - 32.0) * (5.0 / 9.0)),
        }
    }

    /// Convert whatever `Temperature` unit there is into `Fahrenheit`
    pub fn to_fahrenheit(self) -> Temperature {
        match self {
            Kelvin(k) => Fahrenheit((k * (9.0 / 5.0)) - 459.67),
            Celsius(c) => Fahrenheit((c * (9.0 / 5.0)) + 32.0),
            f @ Fahrenheit(_) => f,
        }
    }

    /// Convert whatever `Temperature` unit there is into `Kelvin`
    pub fn to_kelvin(self) -> Temperature {
        match self {
            k @ Kelvin(_) => k,
            Celsius(c) => Kelvin(c + 273.15),
            Fahrenheit(f) => Kelvin((f + 459.67) * (5.0 / 9.0)),
        }
    }
}

impl Add for Temperature {
    type Output = Temperature;

    /// Add the Temperature units together with automatic conversion.
    /// The RHS will be converted into the unit on the left.
    fn add(self, rhs: Temperature) -> Self::Output {
        match (self, rhs) {
            (Celsius(a), b @ _) => match b.to_celsius() {
                Celsius(b) => Celsius(a + b),
                _ => unreachable!(),
            },
            (Fahrenheit(a), b @ _) => match b.to_fahrenheit() {
                Fahrenheit(b) => Fahrenheit(a + b),
                _ => unreachable!(),
            },
            (Kelvin(a), b @ _) => match b.to_kelvin() {
                Kelvin(b) => Kelvin(a + b),
                _ => unreachable!(),
            },
        }
    }
}

fn main() {
    let x = Kelvin(5778.0).to_fahrenheit();
    println!("Kelvin to Fahrenheit: 5778.0 -> {}", x);
}
