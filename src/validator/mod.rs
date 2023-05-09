use std::convert::Into;

use inquire::validator::Validation;
use inquire::CustomUserError;

pub(crate) fn fps_validate(input: &str) -> Result<Validation, CustomUserError> {
    match input.parse::<u16>() {
        Ok(fps) => {
            if (30..=120).contains(&fps) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Please input fps in 30-120!".into()))
            }
        }
        Err(_) => Ok(Validation::Invalid("Please input valid fps number!".into())),
    }
}
