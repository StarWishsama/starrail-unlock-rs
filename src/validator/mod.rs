use std::convert::Into;
use std::ops::{RangeBounds, RangeInclusive};

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

pub(crate) fn bool_validate(input: &str) -> Result<Validation, CustomUserError> {
    match input.to_lowercase().as_str() {
        "true" => Ok(Validation::Valid),
        "false" => Ok(Validation::Valid),
        _ => Ok(Validation::Invalid("Please input true or false".into())),
    }
}

pub(crate) fn render_scale_validate(input: &str) -> Result<Validation, CustomUserError> {
    if let Ok(scale) = input.parse::<f32>() {
        if (0.6..=2.0).contains(&scale) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(
                "Please input valid render scale! (0.6~2.0)".into(),
            ))
        }
    } else {
        Ok(Validation::Invalid(
            "Please input valid render scale! (0.6~2.0)".into(),
        ))
    }
}

pub trait RangeValidate<const START: u16, const END: u16> {
    fn validate(key: &str, input: &str) -> Result<Validation, CustomUserError> {
        if let Ok(scale) = input.parse::<u16>() {
            let range = START..=END;
            if range.contains(&scale) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    format!("Please input valid {}! {}", key, get_range_text(&range)).into(),
                ))
            }
        } else {
            Ok(Validation::Invalid(
                format!("Please input valid {}!", key).into(),
            ))
        }
    }
}

fn get_range_text<R: RangeBounds<u16>>(range: &R) -> String {
    format!("{:?}-{:?}", range.start_bound(), range.end_bound())
}
