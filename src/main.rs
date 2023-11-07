#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, ErrorKind};
use std::panic;
use std::path::Path;
use std::str::from_utf8;

use inquire::{Select, Text};
use log::LevelFilter;
use serde_json::Value;
use strum::EnumProperty;
use winreg::enums::{RegType, HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::{RegKey, RegValue};

use crate::config::setting::GraphicsSetting;
use crate::registry::{find_registry_entry, write_graphics_setting};
use crate::validator::get_select_receiver;

mod config;
mod registry;
mod selector;
mod validator;

const DEFAULT_HELP_MSG: &str = "按 ↑↓ 切换选项, Enter 键选择, Ctrl+C 退出, 输入文本筛选选项.";

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    panic::set_hook(Box::new(|p_info| {
        warn!("出现错误:\n{:?}", p_info);
        suspend()
    }));

    main_menu();

    suspend()
}

fn main_menu() {
    let config_selector = Select::new("请选择要修改的设置", GraphicsSetting::get_selector())
        .with_help_message(DEFAULT_HELP_MSG);

    let setting_entry = find_setting_entry();

    if setting_entry.is_none() {
        warn!("找不到游戏视频设置! 请先启动游戏任意修改一次视频配置.");
        return;
    }

    let config_selector = config_selector.prompt();
    match config_selector {
        Ok(key) => {
            process_graphics_setting(&setting_entry.unwrap(), key);
        }
        Err(e) => {
            warn!("无法解析你的选择: {e}");
            suspend()
        }
    }
}

fn find_setting_entry() -> Option<RegKey> {
    let hkusr = RegKey::predef(HKEY_CURRENT_USER);

    let cn_path = Path::new("Software").join("miHoYo").join("崩坏：星穹铁道");
    let global_path = Path::new("Software").join("Cognosphere").join("Star Rail");

    let starrail_cn = hkusr.open_subkey_with_flags(cn_path, KEY_ALL_ACCESS);
    let starrail_en = hkusr.open_subkey_with_flags(global_path, KEY_ALL_ACCESS);

    match starrail_cn {
        Ok(entry) => return Some(entry),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                warn!("未检测到国服配置, 尝试寻找国际服配置...");
            }
            ErrorKind::PermissionDenied => {
                warn!("请使用管理员权限运行!");
            }
            _ => {
                panic!("{:?}", e)
            }
        },
    }

    match starrail_en {
        Ok(entry) => Some(entry),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    warn!("未检测到国际服配置!");
                }
                ErrorKind::PermissionDenied => {
                    warn!("请使用管理员权限运行!");
                }
                _ => {
                    panic!("{:?}", e)
                }
            }

            None
        }
    }
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

    let key = GraphicsSetting::find_by_display(key).unwrap();
    let message = &format!("请选择{}", key.get_str("display").unwrap());

    if key == GraphicsSetting::Fps {
        match Text::new(select_tips.as_str())
            .with_validator(validator::fps_validate)
            .prompt()
        {
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

        main_menu()
    } else if let Some(receiver) = get_select_receiver(message, key) {
        match receiver.with_help_message(DEFAULT_HELP_MSG).prompt() {
            Ok(input) => {
                write_graphics_setting(entry, k.as_str(), &mut graphics_setting, key, input);
            }

            Err(e) => {
                warn!("输入的值有误! {msg}", msg = e)
            }
        }

        main_menu()
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

fn suspend() {
    info!("按任意键关闭");
    stdin().read_line(&mut String::new()).unwrap();
}
