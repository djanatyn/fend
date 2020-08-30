use crate::num::{FormattingStyle, Number};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Num(Number),
    Func(String),
    Format(FormattingStyle),
    Dp,
}

impl Value {
    pub fn expect_num(&self) -> Result<Number, String> {
        match self {
            Self::Num(bigrat) => Ok(bigrat.clone()),
            _ => Err("Expected a number".to_string()),
        }
    }

    pub fn apply(&self, other: &Self, allow_multiplication: bool, force_multiplication: bool) -> Result<Self, String> {
        Ok(Self::Num(match self {
            Self::Num(n) => {
                if let Self::Dp = other {
                    let num = self.expect_num()?;
                    return Ok(Self::Format(FormattingStyle::ApproxFloat(num.try_as_usize()?)));
                }
                if allow_multiplication {
                    n.clone() * other.expect_num()?
                } else {
                    return Err(format!("{} is not a function", self));
                }
            }
            Self::Func(name) => {
                if force_multiplication {
                    return Err(format!("Cannot apply function '{}' in this context", self));
                }
                if name == "sqrt" {
                    other.expect_num()?.root_n(&2.into())?
                } else if name == "cbrt" {
                    other.expect_num()?.root_n(&3.into())?
                } else if name == "approximately" {
                    other.expect_num()?.make_approximate()
                } else if name == "abs" {
                    other.expect_num()?.abs()?
                } else {
                    return Err(format!("Unknown function '{}'", name));
                }
            }
            _ => {
                return Err(format!("'{}' is not a function or a number", self));
            }
        }))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Num(n) => write!(f, "{}", n)?,
            Self::Func(name) => write!(f, "{}", name)?,
            Self::Format(fmt) => write!(f, "{}", fmt)?,
            Self::Dp => write!(f, "dp")?,
        }
        Ok(())
    }
}
