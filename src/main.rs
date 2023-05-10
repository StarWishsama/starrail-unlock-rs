#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, ErrorKind};
use std::panic;
use std::path::Path;
use std::str::from_utf8;

use inquire::{Select, Text};
use log::LevelFilter;
use serde_json::{Number, Value};
use strum::EnumProperty;
use winreg::enums::{RegType, HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::{RegKey, RegValue};

use crate::config::setting::GraphicsSetting;

mod config;
mod selector;
mod validator;

const DEFAULT_HELP_MSG: &str = "按 ↑↓ 切换选项, 按 Enter 键选择, 输入文本筛选选项.";

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    panic::set_hook(Box::new(|p_info| {
        warn!("出现错误:\n{:?}", p_info);
        suspend()
    }));

    let config_selector = Select::new("请选择要修改的设置", GraphicsSetting::get_selector())
        .with_help_message(DEFAULT_HELP_MSG);

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
                    warn!("无法识别你的选项: {e}");
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

    let select_tips = format!("请输入 {} 欲修改的值:", key);

    let mut select_receiver: Option<Select<&str>> = None;
    let mut input_receiver = Text::new(select_tips.as_str());

    let key = GraphicsSetting::find_by_display(key).unwrap();
    let message = &format!("请选择{}", key.get_str("display").unwrap());

    match key {
        GraphicsSetting::Fps => {
            input_receiver = input_receiver.with_validator(validator::fps_validate)
        }
        GraphicsSetting::EnableVSync => {
            select_receiver = Some(Select::new("是否启用垂直同步?", vec!["true", "false"]));
        }
        GraphicsSetting::RenderScale => {
            select_receiver = Some(Select::new(message, selector::render_scale_selector()));
        }
        GraphicsSetting::ResolutionQuality => {
            select_receiver = Some(Select::new(message, selector::generate_selector(1, 5)));
        }
        GraphicsSetting::ShadowQuality => {
            select_receiver = Some(Select::new(message, selector::shadow_selector()));
        }
        GraphicsSetting::LightQuality => {
            select_receiver = Some(Select::new(message, selector::generate_selector(1, 5)));
        }
        GraphicsSetting::CharacterQuality => {
            select_receiver = Some(Select::new(message, selector::generate_selector(2, 4)));
        }
        GraphicsSetting::ReflectionQuality => {
            select_receiver = Some(Select::new(message, selector::generate_selector(1, 5)));
        }
        GraphicsSetting::BloomQuality => {
            select_receiver = Some(Select::new(message, selector::generate_selector(0, 5)));
        }
        GraphicsSetting::AAMode => {
            select_receiver = Some(Select::new(
                message,
                selector::aa_mode_selector().keys().cloned().collect(),
            ));
        }
    }

    if let Some(sr) = select_receiver {
        match sr.with_help_message(DEFAULT_HELP_MSG).prompt() {
            Ok(input) => {
                write_graphics_setting(entry, k.as_str(), &mut graphics_setting, key, input);
            }

            Err(e) => {
                warn!("输入的值有误! {msg}", msg = e)
            }
        }
    } else {
        match input_receiver.prompt() {
            Ok(input) => {
                write_graphics_setting(
                    entry,
                    k.as_str(),
                    &mut graphics_setting,
                    key,
                    input.as_str(),
                );
            }
            Err(e) => {
                warn!("输入的值有误! {msg}", msg = e)
            }
        }
    }
}

fn write_graphics_setting(
    reg_entry: &RegKey,
    reg_key: &str,
    entry: &mut Value,
    key: GraphicsSetting,
    value: &str,
) {
    let entry = entry
        .get_mut(key.to_string())
        .expect("Unable to deserialize game config");

    match key {
        GraphicsSetting::EnableVSync => *entry = Value::Bool(value.parse().unwrap()),
        GraphicsSetting::RenderScale => *entry = Value::Number(value.parse().unwrap()),
        GraphicsSetting::AAMode => {
            let v = selector::aa_mode_selector();
            *entry = Value::Number(Number::from(v[value]))
        }
        _ => {
            *entry = Value::Number(Number::from(
                selector::get_num_by_option_name(value).unwrap(),
            ))
        }
    }

    let mut raw_json = String::into_bytes(
        serde_json::to_string(&value).expect("Unable to deserialize game config"),
    );

    raw_json.push(0);

    let rv = &RegValue {
        bytes: raw_json,
        vtype: RegType::REG_BINARY,
    };

    modify_registry(reg_entry, reg_key, rv);
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

fn modify_registry(entry: &RegKey, k: &str, rv: &RegValue) {
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

fn suspend() {
    info!("按任意键关闭");
    stdin().read_line(&mut String::new()).unwrap();
}
