use crate::errors::{ConfusingBaseError, Error};
use regex::Regex;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct BaseChoice {
    pub bin: bool,
    pub dec: bool,
    pub hex: bool,
    pub oct: bool,
}
impl BaseChoice {
    pub fn to_vec(&self) -> Vec<bool> {
        Vec::from([self.bin, self.dec, self.hex, self.oct])
    }

    pub fn to_radix(&self) -> Result<u32, Error> {
        self.validate()?;
        Ok(if self.bin {
            2
        } else if self.dec {
            10
        } else if self.hex {
            16
        } else if self.oct {
            8
        } else {
            DEFAULT_BASE
        })
    }
    pub fn validate(&self) -> Result<(), ConfusingBaseError> {
        return if self.to_vec().iter().filter(|&x| *x).count() > 1 {
            Err(ConfusingBaseError::new(format!(
                "only one choice is allowed, but got multiple: {}",
                self
            )))
        } else {
            Ok(())
        };
    }
    pub fn chr(&self, val: u32) -> Result<char, Error> {
        match char::from_u32(val) {
            Some(cs) => Ok(cs),
            None => Err(Error::DataConversionError(format!("could not convert {} to char", val)))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Base {
    Bin,
    Dec,
    Hex,
    Oct,
}

impl Base {
    pub fn to_choice(&self) -> BaseChoice {
        match self {
            Self::Bin => BaseChoice {
                bin: true,
                dec: false,
                hex: false,
                oct: false,
            },
            Self::Dec => BaseChoice {
                bin: false,
                dec: true,
                hex: false,
                oct: false,
            },
            Self::Hex => BaseChoice {
                bin: false,
                dec: false,
                hex: true,
                oct: false,
            },
            Self::Oct => BaseChoice {
                bin: false,
                dec: false,
                hex: false,
                oct: true,
            },
        }
    }
}
#[cfg(test)]
mod base_tests {
    use crate::base::Base;
    use crate::errors::Error;

    #[test]
    fn test_chr() -> Result<(), Error> {
        assert_eq!(Base::Dec.to_choice().chr(71)?, 'G');
        Ok(())
    }
}

impl std::fmt::Display for BaseChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut choices: Vec<&str> = Vec::new();
        if self.bin {
            choices.push("bin");
        }
        if self.dec {
            choices.push("dec");
        }
        if self.hex {
            choices.push("hex");
        }
        if self.oct {
            choices.push("oct");
        }
        write!(f, "{}", choices.join(", "))
    }
}
pub const DEFAULT_BASE: u32 = 10;


#[cfg(test)]
mod base_choice_tests {
    use crate::base::BaseChoice;
    use crate::errors::Error;
    use k9::assert_matches_regex;

    #[test]
    fn test_to_vec() {
        assert_eq!(
            BaseChoice {
                bin: true,
                dec: false,
                hex: false,
                oct: false,
            }
            .to_vec(),
            vec![true, false, false, false],
        );
    }

    #[test]
    fn test_to_radix_success_2() -> Result<(), Error> {
        assert_eq!(
            BaseChoice {
                bin: true,
                dec: false,
                hex: false,
                oct: false,
            }
            .to_radix()?,
            2,
        );
        Ok(())
    }
    #[test]
    fn test_to_radix_success_10() -> Result<(), Error> {
        assert_eq!(
            BaseChoice {
                bin: false,
                dec: true,
                hex: false,
                oct: false,
            }
            .to_radix()?,
            10,
        );
        Ok(())
    }
    #[test]
    fn test_to_radix_success_16() -> Result<(), Error> {
        assert_eq!(
            BaseChoice {
                bin: false,
                dec: false,
                hex: true,
                oct: false,
            }
            .to_radix()?,
            16,
        );
        Ok(())
    }
    #[test]
    fn test_to_radix_success_8() -> Result<(), Error> {
        assert_eq!(
            BaseChoice {
                bin: false,
                dec: false,
                hex: false,
                oct: true,
            }
            .to_radix()?,
            8,
        );
        Ok(())
    }

    #[test]
    fn test_to_radix_fail_multiple_choices_bd() {
        match (BaseChoice {
            bin: true,
            dec: true,
            hex: false,
            oct: false,
        })
        .to_radix()
        {
            Ok(_) => (),
            Err(e) => {
                assert_matches_regex!(e.to_string().as_str(), "got multiple: bin, dec");
            }
        }
    }
    #[test]
    fn test_to_radix_fail_multiple_choices_ho() {
        match (BaseChoice {
            bin: false,
            dec: false,
            hex: true,
            oct: true,
        })
        .to_radix()
        {
            Ok(_) => (),
            Err(e) => {
                assert_matches_regex!(e.to_string().as_str(), "got multiple: hex, oct");
            }
        }
    }
}

pub fn parse_base_from_prefix(data: String) -> Base {
    let br = Regex::new(r"^(0[box])?").unwrap();
    let target = data.to_lowercase();
    match br.captures(&target) {
        Some(caps) => match caps.get(1).map_or("", |m| m.as_str()) {
            "0b" => Base::Bin,
            "0o" => Base::Oct,
            "0x" => Base::Hex,
            _ => Base::Dec,
        },
        None => Base::Dec,
    }
}

pub fn parse_u32_from_string(data: String) -> Result<u32, Error> {
    let br = Regex::new(r"^(0[box])?([0-9a-zA-Z]+)$").unwrap();
    let base = parse_base_from_prefix(data.to_lowercase());
    let bd = data.to_lowercase();
    let caps = match br.captures(&bd) {
        None => {
            return Err(Error::RegexCaptureError(format!(
                "regex {:?} failed to capture groups in {:?}",
                br, bd
            )))
        }
        Some(ss) => ss,
    };
    let prenum = caps.get(2).map_or("0", |m| m.as_str());
    let radix = base.to_choice().to_radix()?;
    eprintln!("prenum:{:?} radix: {:?}", prenum, radix);
    let num = u32::from_str_radix(prenum, radix).unwrap();
    Ok(num)
}

#[cfg(test)]
mod parsing_tests {
    use crate::base::{parse_base_from_prefix, parse_u32_from_string, Base};
    use crate::errors::Error;

    #[test]
    fn test_parse_base_from_prefix() {
        assert_eq!(parse_base_from_prefix("0b100".to_string()), Base::Bin);
        assert_eq!(parse_base_from_prefix("0o71".to_string()), Base::Oct);
        assert_eq!(parse_base_from_prefix("0x3f".to_string()), Base::Hex);
        assert_eq!(parse_base_from_prefix("0".to_string()), Base::Dec);
    }
    #[test]
    fn test_parse_u32_from_string_hex() -> Result<(), Error> {
        assert_eq!(parse_u32_from_string("0b100".to_string())?, 4);
        assert_eq!(parse_u32_from_string("0o71".to_string())?, 57);
        assert_eq!(parse_u32_from_string("0x3f".to_string())?, 63);
        assert_eq!(parse_u32_from_string("55".to_string())?, 55);
        Ok(())
    }
}
