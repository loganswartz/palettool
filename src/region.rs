use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct Region {
    pub label: Option<String>,
    pub start: Point,
    pub end: Point,
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, range) = match s.split_once("=") {
            None => (None, s),
            Some((label, range)) => (Some(label.to_string()), range),
        };

        let (first, second) = range.split_once("-").ok_or("Not a valid range.")?;
        let start = Point::from_str(first)?;
        let end = Point::from_str(second)?;

        Ok(Region {
            label,
            start,
            end,
        })
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range = format!("{}-{}", self.start, self.end);

        let output = match &self.label {
            None => range,
            Some(label) => format!("{}={}", label, range),
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(",").ok_or("Not a valid range.")?;

        let Ok(x) = first.trim().parse() else {
            return Err("The x component couldn't be parsed.".to_string());
        };
        let Ok(y) = second.trim().parse() else {
            return Err("The y component couldn't be parsed.".to_string());
        };

        Ok(Point {
            x,
            y,
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
