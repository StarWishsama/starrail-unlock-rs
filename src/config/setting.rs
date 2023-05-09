use strum::{Display, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

#[derive(Debug, Display, EnumString, IntoStaticStr, EnumIter)]
pub enum GraphicsSetting {
    /// 帧率 (30~120)
    #[strum(serialize = "FPS")]
    Fps,
    /// 垂直同步 true = 开启, false = 关闭
    EnableVSync,
    RenderScale,
    ResolutionQuality,
    ShadowQuality,
    LightQuality,
    /// 角色质量 2 ~ 4 <=> 低 ~ 高
    CharacterQuality,
    /// 反射质量 1 ~ 5 <=> 非常低 => 非常高
    ReflectionQuality,
    /// 泛光效果 0 = 关闭, 1 ~ 5 非常低 => 非常高
    BloomQuality,
    /// 反锯齿 0 = off, 1 = TAA, 2 = FXAA
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

    pub fn get_vec() -> Vec<&'static str> {
        let mut v = Vec::new();
        for gs in GraphicsSetting::iter() {
            v.push(gs.as_static_str())
        }
        v
    }
}
