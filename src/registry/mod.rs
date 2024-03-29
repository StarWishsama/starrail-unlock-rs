use crate::config::setting::GraphicsSetting;
use crate::selector;
use serde_json::{Number, Value};
use std::io::ErrorKind;
use winreg::enums::RegType;
use winreg::{RegKey, RegValue};

pub(crate) fn write_graphics_setting(
    reg_entry: &RegKey,
    reg_key: &str,
    entry: &mut Value,
    key: GraphicsSetting,
    input: &str,
) {
    let key_entry = entry
        .get_mut(key.to_string())
        .expect("Unable to deserialize game config");

    match key {
        GraphicsSetting::EnableVSync => *key_entry = Value::Bool(input.parse().unwrap()),
        GraphicsSetting::RenderScale => *key_entry = Value::Number(input.parse().unwrap()),
        GraphicsSetting::Fps => *key_entry = Value::Number(input.parse().unwrap()),
        GraphicsSetting::AAMode => {
            *key_entry = Value::Number(Number::from(selector::aa_mode_selector()[input]))
        }
        _ => {
            *key_entry = Value::Number(Number::from(
                selector::get_num_by_option_name(input).unwrap(),
            ))
        }
    }

    let mut raw_json = String::into_bytes(
        serde_json::to_string(&entry).expect("Unable to deserialize game config"),
    );

    raw_json.push(0);

    let rv = &RegValue {
        bytes: raw_json,
        vtype: RegType::REG_BINARY,
    };

    modify_registry(reg_entry, reg_key, rv);
}

pub(crate) fn find_registry_entry(entry: &RegKey) -> Option<(String, RegValue)> {
    entry
        .enum_values()
        .map(|x| x.unwrap())
        .find(|entry| entry.0.starts_with("GraphicsSettings_Model"))
}

pub(crate) fn modify_registry(entry: &RegKey, k: &str, rv: &RegValue) {
    match entry.set_raw_value(k, rv) {
        Ok(_) => {
            info!("修改成功! 如未生效请尝试重启游戏");
        }
        Err(e) => {
            warn!("{:?}", e);

            match e.kind() {
                ErrorKind::NotFound => {
                    warn!("未检测到当前客户端注册表数据, 请尝试修改游戏图形设置并关闭!")
                }
                ErrorKind::PermissionDenied => warn!("请使用管理员权限运行!"),
                _ => warn!("修改配置失败!"),
            }
        }
    }
}
