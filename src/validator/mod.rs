use std::convert::Into;

use crate::config::setting::GraphicsSetting;
use crate::selector;
use inquire::validator::Validation;
use inquire::{CustomUserError, Select};

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

pub(crate) fn get_select_receiver(
    message: &str,
    key: GraphicsSetting,
) -> Option<Select<&'static str>> {
    match key {
        GraphicsSetting::EnableVSync => {
            return Some(Select::new("是否启用垂直同步?", vec!["true", "false"]));
        }
        GraphicsSetting::RenderScale => {
            return Some(Select::new(message, selector::render_scale_selector()));
        }
        GraphicsSetting::ResolutionQuality => {
            return Some(Select::new(message, selector::generate_selector(1, 5)));
        }
        GraphicsSetting::ShadowQuality => {
            return Some(Select::new(message, selector::shadow_selector()));
        }
        GraphicsSetting::LightQuality => {
            return Some(Select::new(message, selector::generate_selector(1, 5)));
        }
        GraphicsSetting::CharacterQuality => {
            return Some(Select::new(message, selector::generate_selector(2, 4)));
        }
        GraphicsSetting::ReflectionQuality => {
            return Some(Select::new(message, selector::generate_selector(1, 5)));
        }
        GraphicsSetting::BloomQuality => {
            return Some(Select::new(message, selector::generate_selector(0, 5)));
        }
        GraphicsSetting::AAMode => {
            return Some(Select::new(
                message,
                selector::aa_mode_selector().keys().cloned().collect(),
            ));
        }
        _ => None,
    }
}
