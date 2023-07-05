use crate::errors::{ConfusingBaseError, Error};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct BaseChoice {
    pub bin: bool,
    pub dec: bool,
    pub hex: bool,
    pub oct: bool,
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
}

#[cfg(test)]
mod base_choice_tests {
    use super::BaseChoice;
    use crate::errors::Error;
    use k9::assert_matches_regex;

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
