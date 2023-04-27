use std::io::{stdin, ErrorKind};
use std::panic;
use std::path::Path;
use std::str::from_utf8;

use native_dialog::{MessageDialog, MessageType};
use serde_json::{Number, Value};
use winreg::enums::{RegType, HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::{RegKey, RegValue};

fn main() {
    panic::set_hook(Box::new(|p_info| {
        eprintln!("{:?}", p_info);
        show_message_dialog("出现错误", MessageType::Error, true);
        println!("按任意键关闭");
        stdin().read_line(&mut String::new()).unwrap();
    }));

    let hkusr = RegKey::predef(HKEY_CURRENT_USER);

    let cn_path = Path::new("Software").join("miHoYo").join("崩坏：星穹铁道");
    //let global_path = Path::new("Software").join("Cognosphere").join("Star Rail");

    let starrail_cn = hkusr.open_subkey_with_flags(cn_path, KEY_ALL_ACCESS);
    //let starrail_en = hkusr.open_subkey(global_path);

    match starrail_cn {
        Ok(entry) => {
            process_graphics_setting(&entry);
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => show_message_dialog(
                "未检测到国服注册表数据, 请尝试修改游戏图形设置并关闭!",
                MessageType::Error,
                true,
            ),
            ErrorKind::PermissionDenied => {
                show_message_dialog("请使用管理员权限运行!", MessageType::Error, true)
            }
            _ => {
                panic!("{:?}", e)
            }
        },
    }

    println!("按任意键关闭");
    stdin().read_line(&mut String::new()).unwrap();
}

fn process_graphics_setting(entry: &RegKey) {
    let r = find_registry_kv(entry);

    if r.is_none() {
        show_message_dialog(
            "未检测到图形配置文件, 请尝试修改游戏图形设置并退出设置.",
            MessageType::Warning,
            true,
        );
        return;
    }

    let (k, v) = r.unwrap();

    let graphics_setting = parse_setting_json(&v);

    if graphics_setting.is_none() {
        return;
    }

    let mut graphics_setting: Value = graphics_setting.unwrap();

    if graphics_setting.is_object() && graphics_setting.get("FPS").is_some() {
        let mut_set = graphics_setting
            .get_mut("FPS")
            .expect("Unable to deserialize game config");

        check_config_fps(mut_set);

        let target_fps = input_fps();

        if !(30..=120).contains(&target_fps) {
            show_message_dialog("请输入合法的帧数 (30-120)!", MessageType::Warning, true);
            return;
        }

        *mut_set = Value::Number(Number::from(target_fps));

        let mut v_json = String::into_bytes(
            serde_json::to_string(&graphics_setting).expect("Unable to deserialize game config"),
        );

        v_json.push(0); // fix setting ui glitch?

        let rv = &RegValue {
            bytes: v_json,
            vtype: RegType::REG_BINARY,
        };

        modify_registry(entry, k, rv);
    } else {
        show_message_dialog(
            "解析图形配置文件失败, 请尝试修改游戏图形设置并关闭!",
            MessageType::Error,
            true,
        )
    }
}

fn input_fps() -> usize {
    println!("请输入欲修改的最高帧数 (最高 120fps)");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("读入帧数失败");
    if let Ok(r) = input.parse() {
        r
    } else {
        println!("输入了不合法的数字");
        0
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
        Ok(r) => Some(r),
        Err(e) => {
            println!("Raw Data: {:?}", rv.bytes);
            println!("Stringify: {:?}", str);
            eprintln!("{:?}", e);
            show_message_dialog("解析视频配置失败!", MessageType::Error, true);
            None
        }
    }
}

fn find_registry_kv(entry: &RegKey) -> Option<(String, RegValue)> {
    entry
        .enum_values()
        .map(|x| x.unwrap())
        .find(|entry| entry.0.starts_with("GraphicsSettings_Model"))
}

fn modify_registry(entry: &RegKey, k: String, rv: &RegValue) {
    match entry.set_raw_value(k, rv) {
        Ok(_) => {
            show_message_dialog("修改成功!", MessageType::Info, false);
        }
        Err(e) => {
            eprintln!("{:?}", e);

            match e.kind() {
                ErrorKind::NotFound => show_message_dialog(
                    "未检测到国服注册表数据, 请尝试修改游戏图形设置并关闭!",
                    MessageType::Error,
                    true,
                ),
                ErrorKind::PermissionDenied => {
                    show_message_dialog("请使用管理员权限运行!", MessageType::Error, true)
                }
                _ => {
                    show_message_dialog("修改配置失败!", MessageType::Error, true);
                }
            }
        }
    }
}

fn check_config_fps(v: &Value) {
    if let Some(fps_num) = v.as_i64() {
        if fps_num > 60 {
            println!("你的游戏帧数已被修改过 (当前为 {:?} fps).", fps_num)
        }
    }
}

fn show_message_dialog(content: &str, msg_type: MessageType, show_alert: bool) {
    let dialog = MessageDialog::new()
        .set_type(msg_type)
        .set_title("StarRail FPS Unlock")
        .set_text(content);

    if show_alert {
        dialog.show_alert().unwrap();
    } else {
        dialog.show_confirm().unwrap();
    }
}
