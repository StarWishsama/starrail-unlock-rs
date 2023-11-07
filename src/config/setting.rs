use strum::{Display, EnumIter, EnumProperty, EnumString, IntoEnumIterator, IntoStaticStr};

#[derive(
    Debug, Display, EnumString, EnumProperty, IntoStaticStr, EnumIter, PartialEq, Clone, Copy,
)]
pub enum GraphicsSetting {
    /// 帧率 (30~120)
    #[strum(serialize = "FPS", props(display = "帧率"))]
    Fps,
    /// 垂直同步 true = 开启, false = 关闭
    #[strum(props(display = "垂直同步"))]
    EnableVSync,
    #[strum(props(display = "渲染精度"))]
    RenderScale,
    #[strum(props(display = "场景细节"))]
    ResolutionQuality,
    #[strum(props(display = "阴影质量"))]
    ShadowQuality,
    #[strum(props(display = "光照质量"))]
    LightQuality,
    #[strum(props(display = "角色质量"))]
    CharacterQuality,
    #[strum(props(display = "反射质量"))]
    ReflectionQuality,
    #[strum(props(display = "泛光效果"))]
    BloomQuality,
    /// 反锯齿 0 = off, 1 = TAA, 2 = FXAA
    #[strum(props(display = "抗锯齿"))]
    AAMode,
}

impl GraphicsSetting {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            GraphicsSetting::Fps => "FPS",
            GraphicsSetting::EnableVSync => "EnableVSync",
            GraphicsSetting::RenderScale => "RenderScale",
            GraphicsSetting::ResolutionQuality => "ResolutionQuality",
            GraphicsSetting::ShadowQuality => "ShadowQuality",
            GraphicsSetting::LightQuality => "LightQuality",
            GraphicsSetting::CharacterQuality => "CharacterQuality",
            GraphicsSetting::ReflectionQuality => "ReflectionQuality",
            GraphicsSetting::BloomQuality => "BloomQuality",
            GraphicsSetting::AAMode => "AAMode",
        }
    }

    pub fn get_selector() -> Vec<&'static str> {
        let mut v = Vec::new();
        for gs in GraphicsSetting::iter() {
            v.push(gs.get_str("display").unwrap())
        }
        v
    }

    pub fn find_by_display(display: &str) -> Option<GraphicsSetting> {
        for gs in GraphicsSetting::iter() {
            if let Some(d) = gs.get_str("display") {
                if d == display {
                    return Some(gs);
                }
            }
        }

        None
    }
}
