//! 
//! Vizuara 主题系统
//! 
//! 提供一致的视觉主题功能，支持：
//! - 预定义主题 (预设的视觉风格)
//! - 主题定制 (颜色、字体、样式自定义)
//! - 主题切换 (运行时动态主题变更)
//! - 主题持久化 (主题配置保存和加载)
//! - 主题继承 (基于现有主题的扩展)
//!

pub mod theme;
pub mod palette;
pub mod presets;
pub mod manager;
pub mod builder;

pub use theme::{Theme, ComponentTheme};
pub use palette::ColorPalette;
pub use presets::ThemePresets;
pub use manager::ThemeManager;
pub use builder::{ThemeBuilder, ComponentThemeBuilder, PaletteBuilder};

use vizuara_core::Color;
use serde::{Serialize, Deserialize};

/// 主题系统的错误类型
#[derive(Debug, Clone)]
pub enum ThemeError {
    /// 主题文件解析错误
    ParseError(String),
    /// 主题文件IO错误
    IoError(String),
    /// 未找到指定主题
    ThemeNotFound(String),
    /// 无效的颜色格式
    InvalidColor(String),
    /// 主题配置无效
    InvalidTheme(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::ParseError(msg) => write!(f, "主题解析错误: {}", msg),
            ThemeError::IoError(msg) => write!(f, "主题文件错误: {}", msg),
            ThemeError::ThemeNotFound(name) => write!(f, "未找到主题: {}", name),
            ThemeError::InvalidColor(color) => write!(f, "无效的颜色格式: {}", color),
            ThemeError::InvalidTheme(msg) => write!(f, "无效的主题配置: {}", msg),
        }
    }
}

impl std::error::Error for ThemeError {}

/// 主题系统的结果类型
pub type ThemeResult<T> = Result<T, ThemeError>;

/// 组件类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// 散点图
    ScatterPlot,
    /// 折线图
    LinePlot,
    /// 柱状图
    BarPlot,
    /// 直方图
    Histogram,
    /// 热力图
    Heatmap,
    /// 箱线图
    BoxPlot,
    /// 3D散点图
    Scatter3D,
    /// 3D表面图
    Surface3D,
    /// 3D网格
    Mesh3D,
    /// 坐标轴
    Axis,
    /// 网格线
    Grid,
    /// 图例
    Legend,
    /// 标题
    Title,
    /// 标签
    Label,
    /// 背景
    Background,
    /// 边框
    Border,
    /// 动画
    Animation,
    /// 交互提示
    Tooltip,
}

/// 主题属性类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeProperty {
    /// 主要颜色
    PrimaryColor,
    /// 次要颜色
    SecondaryColor,
    /// 强调颜色
    AccentColor,
    /// 背景颜色
    BackgroundColor,
    /// 表面颜色
    SurfaceColor,
    /// 文本颜色
    TextColor,
    /// 边框颜色
    BorderColor,
    /// 网格颜色
    GridColor,
    /// 字体大小
    FontSize,
    /// 字体粗细
    FontWeight,
    /// 线条宽度
    LineWidth,
    /// 点大小
    PointSize,
    /// 圆角半径
    BorderRadius,
    /// 透明度
    Opacity,
    /// 阴影
    Shadow,
    /// 动画持续时间
    AnimationDuration,
    /// 动画缓动
    AnimationEasing,
}

/// 主题值类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeValue {
    /// 颜色值
    Color(Color),
    /// 数值
    Number(f32),
    /// 字符串
    String(String),
    /// 布尔值
    Boolean(bool),
}

impl ThemeValue {
    /// 获取颜色值
    pub fn as_color(&self) -> Option<Color> {
        match self {
            ThemeValue::Color(color) => Some(*color),
            _ => None,
        }
    }

    /// 获取数值
    pub fn as_number(&self) -> Option<f32> {
        match self {
            ThemeValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// 获取字符串
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ThemeValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// 获取布尔值
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ThemeValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_value_conversion() {
        let color_value = ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(color_value.as_color(), Some(Color::rgb(1.0, 0.0, 0.0)));
        assert_eq!(color_value.as_number(), None);

        let number_value = ThemeValue::Number(42.0);
        assert_eq!(number_value.as_number(), Some(42.0));
        assert_eq!(number_value.as_color(), None);

        let string_value = ThemeValue::String("test".to_string());
        assert_eq!(string_value.as_string(), Some("test"));
        assert_eq!(string_value.as_boolean(), None);

        let bool_value = ThemeValue::Boolean(true);
        assert_eq!(bool_value.as_boolean(), Some(true));
        assert_eq!(bool_value.as_string(), None);
    }

    #[test]
    fn test_component_type_equality() {
        assert_eq!(ComponentType::ScatterPlot, ComponentType::ScatterPlot);
        assert_ne!(ComponentType::ScatterPlot, ComponentType::LinePlot);
    }

    #[test]
    fn test_theme_property_hash() {
        let mut map = HashMap::new();
        map.insert(ThemeProperty::PrimaryColor, ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)));
        map.insert(ThemeProperty::FontSize, ThemeValue::Number(14.0));
        
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(&ThemeProperty::PrimaryColor));
        assert!(map.contains_key(&ThemeProperty::FontSize));
    }
}
