use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref GENERAL_QUALITY_NAME: HashMap<u16, &'static str> = {
        let mut map = HashMap::new();

        map.insert(0, "关");
        map.insert(1, "非常低");
        map.insert(2, "低");
        map.insert(3, "中");
        map.insert(4, "高");
        map.insert(5, "非常高");

        map
    };
}

pub fn render_scale_selector() -> Vec<&'static str> {
    vec!["0.6", "0.8", "1.0", "1.2", "1.4", "1.6", "1.8", "2.0"]
}

pub fn aa_mode_selector() -> HashMap<&'static str, u16> {
    let mut map = HashMap::new();

    map.insert("关闭", 0);
    map.insert("TAA", 1);
    map.insert("FXAA", 2);

    map
}

pub fn shadow_selector() -> Vec<&'static str> {
    let mut v = generate_selector(0, 4);

    v.remove(1);

    v
}

pub fn generate_selector(start: u16, end: u16) -> Vec<&'static str> {
    let mut v = Vec::new();
    for n in start..=end {
        v.push(get_option_name_by_num(&n));
    }
    v
}

pub fn get_num_by_option_name(str: &str) -> Option<u16> {
    for (num, name) in GENERAL_QUALITY_NAME.iter() {
        if *name == str {
            return Some(*num);
        }
    }

    None
}

pub fn get_option_name_by_num(num: &u16) -> &'static str {
    GENERAL_QUALITY_NAME[num]
}
