use crate::AngleMode;

pub enum Function {
    Sin,  // Sine
    Cos,  // Cosine
    Tan,  // Tangent
    Asin, // Inverse sine
    Acos, // Inverse cosine
    Atan, // Inverse tangent
    Deg,  // Convert from radians to degrees
    Rad,  // Convert from degrees to radians
}

impl Function {
    pub fn parse(s: &str) -> Result<Self, ParseFunctionError> {
        match s {
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            "asin" => Ok(Self::Asin),
            "acos" => Ok(Self::Acos),
            "atan" => Ok(Self::Atan),
            "deg" => Ok(Self::Deg),
            "rad" => Ok(Self::Rad),
            _ => Err(ParseFunctionError(s.to_string())),
        }
    }

    pub fn call(&self, val: f64, angle_mode: AngleMode) -> f64 {
        match self {
            Self::Sin => match angle_mode {
                AngleMode::Degrees => val.to_radians().sin(),
                AngleMode::Radians => val.sin(),
            },
            Self::Cos => match angle_mode {
                AngleMode::Degrees => val.to_radians().cos(),
                AngleMode::Radians => val.cos(),
            },
            Self::Tan => match angle_mode {
                AngleMode::Degrees => val.to_radians().tan(),
                AngleMode::Radians => val.tan(),
            },
            Self::Asin => match angle_mode {
                AngleMode::Degrees => val.asin().to_degrees(),
                AngleMode::Radians => val.asin(),
            },
            Self::Acos => match angle_mode {
                AngleMode::Degrees => val.acos().to_degrees(),
                AngleMode::Radians => val.acos(),
            },
            Self::Atan => match angle_mode {
                AngleMode::Degrees => val.atan().to_degrees(),
                AngleMode::Radians => val.atan(),
            },
            Self::Deg => val.to_degrees(),
            Self::Rad => val.to_radians(),
        }
    }
}

pub struct ParseFunctionError(String);
