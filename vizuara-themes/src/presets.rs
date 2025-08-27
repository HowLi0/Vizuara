use vizuara_core::Color;
use crate::{Theme, ComponentTheme, ColorPalette, ComponentType, ThemeProperty, ThemeValue};

/// 预设主题集合
/// 
/// 提供常用的预定义主题，开箱即用
pub struct ThemePresets;

impl ThemePresets {
    /// 默认主题（浅色）
    pub fn default() -> Theme {
        let palette = ColorPalette::new("Default", "Default light theme")
            .with_primary(Color::rgb(0.2, 0.6, 0.8))
            .with_secondary(Color::rgb(0.8, 0.4, 0.2))
            .with_accent(Color::rgb(0.9, 0.2, 0.5))
            .with_background(Color::rgb(1.0, 1.0, 1.0))
            .with_text(Color::rgb(0.2, 0.2, 0.2));

        Self::create_theme_from_palette("default", "默认主题", palette)
    }

    /// 深色主题
    pub fn dark() -> Theme {
        let palette = ColorPalette::new("Dark", "Dark theme")
            .with_primary(Color::rgb(0.4, 0.7, 0.9))
            .with_secondary(Color::rgb(0.9, 0.5, 0.3))
            .with_accent(Color::rgb(1.0, 0.3, 0.6))
            .with_background(Color::rgb(0.12, 0.12, 0.12))
            .with_text(Color::rgb(0.9, 0.9, 0.9));

        Self::create_theme_from_palette("dark", "深色主题", palette)
    }

    /// 科学主题（蓝色调）
    pub fn scientific() -> Theme {
        let palette = ColorPalette::new("Scientific", "Scientific blue theme")
            .with_primary(Color::rgb(0.15, 0.4, 0.7))
            .with_secondary(Color::rgb(0.2, 0.6, 0.8))
            .with_accent(Color::rgb(0.0, 0.3, 0.6))
            .with_background(Color::rgb(0.98, 0.99, 1.0))
            .with_text(Color::rgb(0.1, 0.1, 0.2))
            .with_series(vec![
                Color::rgb(0.15, 0.4, 0.7),   // 深蓝
                Color::rgb(0.2, 0.6, 0.8),    // 中蓝
                Color::rgb(0.4, 0.7, 0.9),    // 浅蓝
                Color::rgb(0.1, 0.5, 0.6),    // 蓝绿
                Color::rgb(0.0, 0.3, 0.5),    // 深蓝绿
                Color::rgb(0.6, 0.8, 0.9),    // 天蓝
            ]);

        Self::create_theme_from_palette("scientific", "科学主题", palette)
    }

    /// 商业主题（专业色调）
    pub fn business() -> Theme {
        let palette = ColorPalette::new("Business", "Professional business theme")
            .with_primary(Color::rgb(0.2, 0.3, 0.4))
            .with_secondary(Color::rgb(0.6, 0.7, 0.8))
            .with_accent(Color::rgb(0.8, 0.3, 0.1))
            .with_background(Color::rgb(0.99, 0.99, 0.99))
            .with_text(Color::rgb(0.1, 0.1, 0.1))
            .with_series(vec![
                Color::rgb(0.2, 0.3, 0.4),    // 深灰蓝
                Color::rgb(0.8, 0.3, 0.1),    // 橙红
                Color::rgb(0.3, 0.5, 0.2),    // 深绿
                Color::rgb(0.5, 0.2, 0.4),    // 紫色
                Color::rgb(0.7, 0.6, 0.1),    // 金黄
                Color::rgb(0.4, 0.6, 0.7),    // 钢蓝
            ]);

        Self::create_theme_from_palette("business", "商业主题", palette)
    }

    /// 医疗主题（绿色调）
    pub fn medical() -> Theme {
        let palette = ColorPalette::new("Medical", "Medical green theme")
            .with_primary(Color::rgb(0.2, 0.6, 0.4))
            .with_secondary(Color::rgb(0.3, 0.7, 0.5))
            .with_accent(Color::rgb(0.1, 0.5, 0.3))
            .with_background(Color::rgb(0.98, 1.0, 0.99))
            .with_text(Color::rgb(0.1, 0.2, 0.1))
            .with_series(vec![
                Color::rgb(0.2, 0.6, 0.4),    // 医疗绿
                Color::rgb(0.1, 0.5, 0.3),    // 深绿
                Color::rgb(0.3, 0.7, 0.5),    // 浅绿
                Color::rgb(0.4, 0.8, 0.6),    // 薄荷绿
                Color::rgb(0.0, 0.4, 0.2),    // 森林绿
                Color::rgb(0.5, 0.9, 0.7),    // 春绿
            ]);

        Self::create_theme_from_palette("medical", "医疗主题", palette)
    }

    /// 教育主题（温暖色调）
    pub fn education() -> Theme {
        let palette = ColorPalette::new("Education", "Warm education theme")
            .with_primary(Color::rgb(0.8, 0.5, 0.2))
            .with_secondary(Color::rgb(0.9, 0.6, 0.3))
            .with_accent(Color::rgb(0.7, 0.4, 0.1))
            .with_background(Color::rgb(1.0, 0.99, 0.97))
            .with_text(Color::rgb(0.2, 0.1, 0.0))
            .with_series(vec![
                Color::rgb(0.8, 0.5, 0.2),    // 橙色
                Color::rgb(0.7, 0.3, 0.4),    // 暖红
                Color::rgb(0.6, 0.4, 0.7),    // 紫色
                Color::rgb(0.3, 0.6, 0.5),    // 青绿
                Color::rgb(0.9, 0.7, 0.3),    // 金黄
                Color::rgb(0.5, 0.5, 0.8),    // 淡紫
            ]);

        Self::create_theme_from_palette("education", "教育主题", palette)
    }

    /// 高对比度主题（可访问性）
    pub fn high_contrast() -> Theme {
        let palette = ColorPalette::new("HighContrast", "High contrast accessibility theme")
            .with_primary(Color::rgb(0.0, 0.0, 0.0))
            .with_secondary(Color::rgb(1.0, 1.0, 1.0))
            .with_accent(Color::rgb(1.0, 0.0, 0.0))
            .with_background(Color::rgb(1.0, 1.0, 1.0))
            .with_text(Color::rgb(0.0, 0.0, 0.0))
            .with_series(vec![
                Color::rgb(0.0, 0.0, 0.0),    // 黑色
                Color::rgb(1.0, 0.0, 0.0),    // 红色
                Color::rgb(0.0, 0.0, 1.0),    // 蓝色
                Color::rgb(0.0, 0.8, 0.0),    // 绿色
                Color::rgb(1.0, 0.0, 1.0),    // 品红
                Color::rgb(1.0, 1.0, 0.0),    // 黄色
            ]);

        Self::create_theme_from_palette("high_contrast", "高对比度主题", palette)
    }

    /// 色盲友好主题
    pub fn colorblind_friendly() -> Theme {
        let palette = ColorPalette::new("ColorblindFriendly", "Colorblind-friendly theme")
            .with_primary(Color::rgb(0.0, 0.4, 0.8))      // 蓝色
            .with_secondary(Color::rgb(0.9, 0.6, 0.0))    // 橙色
            .with_accent(Color::rgb(0.0, 0.6, 0.5))       // 青色
            .with_background(Color::rgb(1.0, 1.0, 1.0))
            .with_text(Color::rgb(0.2, 0.2, 0.2))
            .with_series(vec![
                Color::rgb(0.0, 0.4, 0.8),    // 蓝色
                Color::rgb(0.9, 0.6, 0.0),    // 橙色
                Color::rgb(0.0, 0.6, 0.5),    // 青色
                Color::rgb(0.8, 0.4, 0.7),    // 淡紫
                Color::rgb(0.2, 0.6, 0.2),    // 绿色
                Color::rgb(0.8, 0.8, 0.0),    // 黄色
            ]);

        Self::create_theme_from_palette("colorblind_friendly", "色盲友好主题", palette)
    }

    /// 打印友好主题（黑白）
    pub fn print_friendly() -> Theme {
        let palette = ColorPalette::new("PrintFriendly", "Print-friendly grayscale theme")
            .with_primary(Color::rgb(0.2, 0.2, 0.2))
            .with_secondary(Color::rgb(0.5, 0.5, 0.5))
            .with_accent(Color::rgb(0.0, 0.0, 0.0))
            .with_background(Color::rgb(1.0, 1.0, 1.0))
            .with_text(Color::rgb(0.0, 0.0, 0.0))
            .with_series(vec![
                Color::rgb(0.1, 0.1, 0.1),    // 深灰
                Color::rgb(0.3, 0.3, 0.3),    // 中深灰
                Color::rgb(0.5, 0.5, 0.5),    // 中灰
                Color::rgb(0.7, 0.7, 0.7),    // 浅灰
                Color::rgb(0.9, 0.9, 0.9),    // 很浅灰
                Color::rgb(0.0, 0.0, 0.0),    // 黑色
            ]);

        Self::create_theme_from_palette("print_friendly", "打印友好主题", palette)
    }

    /// 从调色板创建主题
    fn create_theme_from_palette(name: &str, description: &str, palette: ColorPalette) -> Theme {
        let mut theme = Theme::new(name, description)
            .with_version("1.0.0")
            .with_author("Vizuara Theme System");

        // 设置全局属性
        theme.set_global(ThemeProperty::PrimaryColor, ThemeValue::Color(palette.primary));
        theme.set_global(ThemeProperty::SecondaryColor, ThemeValue::Color(palette.secondary));
        theme.set_global(ThemeProperty::AccentColor, ThemeValue::Color(palette.accent));
        theme.set_global(ThemeProperty::BackgroundColor, ThemeValue::Color(palette.background));
        theme.set_global(ThemeProperty::TextColor, ThemeValue::Color(palette.text));
        theme.set_global(ThemeProperty::BorderColor, ThemeValue::Color(palette.text));
        theme.set_global(ThemeProperty::FontSize, ThemeValue::Number(12.0));
        theme.set_global(ThemeProperty::LineWidth, ThemeValue::Number(2.0));

        // 散点图主题
        let scatter_theme = ComponentTheme::new("ScatterPlot")
            .with_primary_color(palette.get_series_color(0))
            .with_point_size(4.0)
            .with_line_width(1.5);
        theme.add_component(ComponentType::ScatterPlot, scatter_theme);

        // 折线图主题
        let line_theme = ComponentTheme::new("LinePlot")
            .with_primary_color(palette.get_series_color(1))
            .with_line_width(2.5);
        theme.add_component(ComponentType::LinePlot, line_theme);

        // 柱状图主题
        let bar_theme = ComponentTheme::new("BarPlot")
            .with_primary_color(palette.get_series_color(2))
            .with_border_color(palette.text)
            .with_line_width(1.0);
        theme.add_component(ComponentType::BarPlot, bar_theme);

        // 直方图主题
        let histogram_theme = ComponentTheme::new("Histogram")
            .with_primary_color(palette.get_series_color(3))
            .with_border_color(palette.text)
            .with_line_width(1.0);
        theme.add_component(ComponentType::Histogram, histogram_theme);

        // 热力图主题
        let heatmap_theme = ComponentTheme::new("Heatmap")
            .with_primary_color(palette.primary);
        theme.add_component(ComponentType::Heatmap, heatmap_theme);

        // 箱线图主题
        let boxplot_theme = ComponentTheme::new("BoxPlot")
            .with_primary_color(palette.get_series_color(4))
            .with_border_color(palette.text)
            .with_line_width(2.0);
        theme.add_component(ComponentType::BoxPlot, boxplot_theme);

        // 3D散点图主题
        let scatter3d_theme = ComponentTheme::new("Scatter3D")
            .with_primary_color(palette.get_series_color(0))
            .with_point_size(5.0);
        theme.add_component(ComponentType::Scatter3D, scatter3d_theme);

        // 3D表面图主题
        let surface3d_theme = ComponentTheme::new("Surface3D")
            .with_primary_color(palette.primary);
        theme.add_component(ComponentType::Surface3D, surface3d_theme);

        // 坐标轴主题
        let axis_theme = ComponentTheme::new("Axis")
            .with_primary_color(palette.text)
            .with_line_width(1.5);
        theme.add_component(ComponentType::Axis, axis_theme);

        // 网格主题
        let grid_theme = ComponentTheme::new("Grid")
            .with_primary_color(Color::new(
                palette.text.r, 
                palette.text.g, 
                palette.text.b, 
                0.3
            ))
            .with_line_width(0.5);
        theme.add_component(ComponentType::Grid, grid_theme);

        // 图例主题
        let legend_theme = ComponentTheme::new("Legend")
            .with_primary_color(palette.text)
            .with_border_color(palette.text)
            .with_line_width(1.0);
        theme.add_component(ComponentType::Legend, legend_theme);

        // 标题主题
        let title_theme = ComponentTheme::new("Title")
            .with_primary_color(palette.text);
        theme.add_component(ComponentType::Title, title_theme);

        // 背景主题
        let background_theme = ComponentTheme::new("Background")
            .with_primary_color(palette.background);
        theme.add_component(ComponentType::Background, background_theme);

        // 存储调色板信息到自定义属性
        theme.set_custom("palette_name", ThemeValue::String(palette.name.clone()));
        for (i, color) in palette.series.iter().enumerate() {
            theme.set_custom(
                format!("series_color_{}", i), 
                ThemeValue::Color(*color)
            );
        }

        theme
    }

    /// 获取所有预设主题名称
    pub fn list_preset_names() -> Vec<&'static str> {
        vec![
            "default",
            "dark",
            "scientific",
            "business",
            "medical",
            "education",
            "high_contrast",
            "colorblind_friendly",
            "print_friendly",
        ]
    }

    /// 根据名称获取预设主题
    pub fn get_preset(name: &str) -> Option<Theme> {
        match name {
            "default" => Some(Self::default()),
            "dark" => Some(Self::dark()),
            "scientific" => Some(Self::scientific()),
            "business" => Some(Self::business()),
            "medical" => Some(Self::medical()),
            "education" => Some(Self::education()),
            "high_contrast" => Some(Self::high_contrast()),
            "colorblind_friendly" => Some(Self::colorblind_friendly()),
            "print_friendly" => Some(Self::print_friendly()),
            _ => None,
        }
    }

    /// 获取主题的简短描述
    pub fn get_theme_description(name: &str) -> Option<&'static str> {
        match name {
            "default" => Some("经典的浅色主题，适合一般用途"),
            "dark" => Some("深色主题，减少眼睛疲劳"),
            "scientific" => Some("科学研究风格，蓝色调为主"),
            "business" => Some("商业专业风格，稳重的配色"),
            "medical" => Some("医疗健康风格，绿色调为主"),
            "education" => Some("教育培训风格，温暖的配色"),
            "high_contrast" => Some("高对比度主题，增强可访问性"),
            "colorblind_friendly" => Some("色盲友好主题，使用可区分的颜色"),
            "print_friendly" => Some("打印友好主题，灰度配色"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = ThemePresets::default();
        assert_eq!(theme.name, "default");
        assert!(theme.validate().is_ok());
        
        // 验证全局属性
        assert!(theme.get_global(&ThemeProperty::PrimaryColor).is_some());
        assert!(theme.get_global(&ThemeProperty::BackgroundColor).is_some());
        assert!(theme.get_global(&ThemeProperty::TextColor).is_some());
    }

    #[test]
    fn test_dark_theme() {
        let theme = ThemePresets::dark();
        assert_eq!(theme.name, "dark");
        assert!(theme.validate().is_ok());
        
        // 深色主题的背景应该是深色
        let bg_color = theme.get_background_color();
        assert!(bg_color.r < 0.5 && bg_color.g < 0.5 && bg_color.b < 0.5);
    }

    #[test]
    fn test_all_presets_valid() {
        for name in ThemePresets::list_preset_names() {
            let theme = ThemePresets::get_preset(name).unwrap();
            assert!(theme.validate().is_ok(), "主题 {} 验证失败", name);
        }
    }

    #[test]
    fn test_component_themes() {
        let theme = ThemePresets::scientific();
        
        // 验证组件主题存在
        assert!(theme.get_component(&ComponentType::ScatterPlot).is_some());
        assert!(theme.get_component(&ComponentType::LinePlot).is_some());
        assert!(theme.get_component(&ComponentType::BarPlot).is_some());
        assert!(theme.get_component(&ComponentType::Axis).is_some());
    }

    #[test]
    fn test_high_contrast_theme() {
        let theme = ThemePresets::high_contrast();
        
        // 高对比度主题应该使用黑白颜色
        let primary = theme.get_primary_color(&ComponentType::ScatterPlot);
        let background = theme.get_background_color();
        
        // 计算对比度（简化检查）
        let primary_luminance = 0.299 * primary.r + 0.587 * primary.g + 0.114 * primary.b;
        let bg_luminance = 0.299 * background.r + 0.587 * background.g + 0.114 * background.b;
        let contrast = (primary_luminance - bg_luminance).abs();
        
        assert!(contrast > 0.5, "高对比度主题的对比度不足");
    }

    #[test]
    fn test_preset_names_and_descriptions() {
        let names = ThemePresets::list_preset_names();
        assert!(!names.is_empty());
        
        for name in names {
            assert!(ThemePresets::get_preset(name).is_some());
            assert!(ThemePresets::get_theme_description(name).is_some());
        }
    }

    #[test]
    fn test_colorblind_friendly_palette() {
        let theme = ThemePresets::colorblind_friendly();
        
        // 色盲友好主题应该避免红绿组合
        let series_colors = (0..6).map(|i| 
            theme.get_custom(&format!("series_color_{}", i))
                .and_then(|v| v.as_color())
                .unwrap_or(Color::rgb(0.0, 0.0, 0.0))
        ).collect::<Vec<_>>();
        
        // 应该包含蓝色和橙色（色盲友好的对比色）
        let has_blue = series_colors.iter().any(|c| c.b > 0.5 && c.r < 0.3 && c.g < 0.6);
        let has_orange = series_colors.iter().any(|c| c.r > 0.6 && c.g > 0.3 && c.b < 0.3);
        
        assert!(has_blue, "色盲友好主题应该包含蓝色");
        assert!(has_orange, "色盲友好主题应该包含橙色");
    }

    #[test]
    fn test_print_friendly_grayscale() {
        let theme = ThemePresets::print_friendly();
        
        // 打印友好主题应该使用灰度色彩
        let primary = theme.get_primary_color(&ComponentType::ScatterPlot);
        let tolerance = 0.1;
        
        assert!(
            (primary.r - primary.g).abs() < tolerance && (primary.g - primary.b).abs() < tolerance,
            "打印友好主题应该使用灰度颜色"
        );
    }
}
