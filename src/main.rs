use std::io::{ErrorKind, stdin};

use native_dialog::{MessageDialog, MessageType};
use serde_json::{Number, Value};
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

fn main() {
    let hkusr = RegKey::predef(HKEY_CURRENT_USER);
    let starrail_cn = hkusr.open_subkey("Software\\miHoYo\\崩坏：星穹铁道");
    //let starrail_en = hkusr.open_subkey("Software\\Cognosphere\\Star Rail");

    match starrail_cn {
        Ok(entry) => {
            let (k, v) = entry
                .enum_values()
                .map(|x| x.unwrap())
                .find(|entry| entry.0.starts_with("GraphicsSettings_Model"))
                .expect("未检测到图形配置文件, 请尝试修改游戏图形设置并关闭.");

            let graphics_setting: String = String::from_utf8(v.bytes)
                .expect("解析图形配置文件失败, 请尝试修改游戏图形设置并关闭.");

            let mut graphics_setting: Value = serde_json::from_str(graphics_setting.as_str())
                .expect("Unable to parse game config");

            if graphics_setting.is_object() && graphics_setting.get("FPS").is_some() {
                let mut_set = graphics_setting.get_mut("FPS").expect("Unable to deserialize game config");

                if let Some(fps_num) = mut_set.as_i64() {
                    if fps_num > 60 {
                        show_message_dialog(
                            format!("你的游戏帧数已被修改过 (当前为 {:?} fps)", fps_num).as_str(),
                            MessageType::Error,
                            false);
                        return;
                    }
                }

                *mut_set = Value::Number(Number::from(120));

                entry.set_value(
                    k.as_str(),
                    &serde_json::to_string(graphics_setting.as_str().unwrap())
                        .expect("Unable to deserialize game config"),
                ).expect("Unable to set registry");

                show_message_dialog("修改成功!", MessageType::Info, false)
            } else {
                show_message_dialog("解析图形配置文件失败, 请尝试修改游戏图形设置并关闭!", MessageType::Error, true)
            }
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    show_message_dialog("未检测到国服注册表数据, 请尝试修改游戏图形设置并关闭!", MessageType::Error, true)
                }
                ErrorKind::PermissionDenied => {
                    show_message_dialog("请使用管理员权限运行!", MessageType::Error, true)
                }
                _ => {
                    show_message_dialog("出现错误", MessageType::Error, true);
                    panic!("{:?}", e)
                }
            }
        }
    }

    println!("按任意键关闭");
    stdin().read_line(&mut String::new()).unwrap();
}

fn show_message_dialog(content: &str, msg_type: MessageType, show_alert: bool) {
    let dialog = MessageDialog::new()
        .set_type(msg_type)
        .set_title("StarRailFPSUnlock")
        .set_text(content);

    if show_alert {
        dialog.show_alert().unwrap();
    } else {
        dialog.show_confirm().unwrap();
    }
}