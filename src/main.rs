#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, ErrorKind};
use std::panic;
use std::path::Path;
use std::str::{from_utf8, FromStr};

use inquire::{Select, Text};
use log::LevelFilter;
use serde_json::Value;
use winreg::enums::{RegType, HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::{RegKey, RegValue};

use crate::config::setting::GraphicsSetting;
use crate::validator::RangeValidate;

mod config;
mod selector;
mod validator;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    panic::set_hook(Box::new(|p_info| {
        warn!("出现错误:\n{:?}", p_info);
        suspend()
    }));

    let config_selector = Select::new("请选择要修改的设置", selector::get_graphics_settings())
        .with_help_message("按 ↑↓ 切换选项, 按 Enter 键选择, 输入文本筛选选项.");

    let hkusr = RegKey::predef(HKEY_CURRENT_USER);

    let cn_path = Path::new("Software").join("miHoYo").join("崩坏：星穹铁道");
    let global_path = Path::new("Software").join("Cognosphere").join("Star Rail");

    let starrail_cn = hkusr.open_subkey_with_flags(cn_path, KEY_ALL_ACCESS);
    let starrail_en = hkusr.open_subkey_with_flags(global_path, KEY_ALL_ACCESS);

    match starrail_cn {
        Ok(entry) => {
            let config_selector = config_selector.prompt();
            match config_selector {
                Ok(key) => {
                    process_graphics_setting(&entry, key);
                }
                Err(e) => {
                    warn!("无法检测你的选项: {e}");
                }
            }

            suspend();

            return;
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                warn!("未检测到国服注册表数据, 请尝试修改游戏图形设置一次并关闭!");
            }
            ErrorKind::PermissionDenied => {
                warn!("请使用管理员权限运行!");
                return;
            }
            _ => {
                panic!("{:?}", e)
            }
        },
    }

    match starrail_en {
        Ok(entry) => {
            let config_selector = config_selector.prompt();
            match config_selector {
                Ok(key) => {
                    process_graphics_setting(&entry, key);
                }
                Err(e) => {
                    warn!("Unable to parse your select: {e}");
                    suspend()
                }
            }
            return;
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                warn!("未检测到国际服注册表数据, 请尝试修改游戏图形设置并关闭!")
            }
            ErrorKind::PermissionDenied => warn!("请使用管理员权限运行!"),
            _ => panic!("{:?}", e),
        },
    }

    suspend()
}

fn suspend() {
    info!("按任意键关闭");
    stdin().read_line(&mut String::new()).unwrap();
}

fn process_graphics_setting(entry: &RegKey, key: &str) {
    let r = find_registry_entry(entry);

    if r.is_none() {
        warn!("未检测到图形配置文件, 请尝试修改游戏图形设置并退出设置.");
        return;
    }

    let (k, v) = r.unwrap();

    let graphics_setting = parse_setting_json(&v);

    if graphics_setting.is_none() {
        return;
    }

    let mut graphics_setting: Value = graphics_setting.unwrap();

    debug!(
        "Current graphics setting: \n{:?}",
        serde_json::to_string(&graphics_setting)
    );

    let select_tips = format!("请输入 {} 要修改的值:", key);

    let mut input_receiver = Text::new(select_tips.as_str());

    let key = GraphicsSetting::from_str(key).unwrap();
    let key_str = key.as_static_str();

    match key {
        GraphicsSetting::Fps => {
            input_receiver = input_receiver.with_validator(validator::fps_validate)
        }
        GraphicsSetting::EnableVSync => {
            input_receiver = input_receiver.with_validator(validator::bool_validate)
        }
        GraphicsSetting::RenderScale => {
            input_receiver = input_receiver.with_validator(validator::render_scale_validate)
        }
        //"resolution_quality" => input_receiver = input_receiver.with_validator(validator::get_custom_range_validator()),
        //"shadow_quality" => input_receiver = input_receiver.with_validator(validator::get_custom_range_validator()),
        //"light_quality" => input_receiver = input_receiver.with_validator(validator::get_custom_range_validator()),
        GraphicsSetting::CharacterQuality => {
            struct Validator;
            impl RangeValidate<2, 4> for Validator {}
            input_receiver =
                input_receiver.with_validator(|input: &str| Validator::validate(key_str, input))
        }
        GraphicsSetting::ReflectionQuality => {
            struct Validator;
            impl RangeValidate<1, 5> for Validator {}
            input_receiver =
                input_receiver.with_validator(|input: &str| Validator::validate(key_str, input))
        }
        GraphicsSetting::BloomQuality => {
            struct Validator;
            impl RangeValidate<0, 5> for Validator {}
            input_receiver =
                input_receiver.with_validator(|input: &str| Validator::validate(key_str, input))
        }
        GraphicsSetting::AAMode => {
            struct Validator;
            impl RangeValidate<0, 2> for Validator {}
            input_receiver =
                input_receiver.with_validator(|input: &str| Validator::validate(key_str, input))
        }
        _ => {}
    }

    match input_receiver.prompt() {
        Ok(value) => {
            if graphics_setting.is_object() && graphics_setting.get(key_str).is_some() {
                let entry = graphics_setting
                    .get_mut(key.to_string())
                    .expect("Unable to deserialize game config");

                match key {
                    GraphicsSetting::EnableVSync => *entry = Value::Bool(value.parse().unwrap()),
                    GraphicsSetting::RenderScale => *entry = Value::Number(value.parse().unwrap()),
                    _ => *entry = Value::String(value.parse().unwrap()),
                }
            } else {
                warn!("Unable to deserialize game config")
            }

            let mut raw_json = String::into_bytes(
                serde_json::to_string(&graphics_setting)
                    .expect("Unable to deserialize game config"),
            );

            raw_json.push(0);

            let rv = &RegValue {
                bytes: raw_json,
                vtype: RegType::REG_BINARY,
            };

            modify_registry(entry, k, rv);
        }
        Err(e) => {
            warn!("输入的值有误! {msg}", msg = e)
        }
    }
}

fn parse_setting_json(rv: &RegValue) -> Option<Value> {
    if rv.vtype != RegType::REG_BINARY {
        return None;
    }

    let str = from_utf8(rv.bytes.as_slice())
        .unwrap()
        .trim_matches(char::from(0));

    let result: Result<Value, _> = serde_json::from_str(str);

    match result {
        Ok(v) => Some(v),
        Err(e) => {
            warn!("解析视频配置失败!\n{:?}", e);
            None
        }
    }
}

fn find_registry_entry(entry: &RegKey) -> Option<(String, RegValue)> {
    entry
        .enum_values()
        .map(|x| x.unwrap())
        .find(|entry| entry.0.starts_with("GraphicsSettings_Model"))
}

fn modify_registry(entry: &RegKey, k: String, rv: &RegValue) {
    match entry.set_raw_value(k, rv) {
        Ok(_) => {
            info!("修改成功! 如未生效请尝试重启游戏");
        }
        Err(e) => {
            warn!("{:?}", e);

            match e.kind() {
                ErrorKind::NotFound => {
                    warn!("未检测到国服注册表数据, 请尝试修改游戏图形设置并关闭!")
                }
                ErrorKind::PermissionDenied => warn!("请使用管理员权限运行!"),
                _ => warn!("修改配置失败!"),
            }
        }
    }
}
