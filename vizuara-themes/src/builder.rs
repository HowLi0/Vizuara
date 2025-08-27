use vizuara_core::Color;
use crate::{Theme, ComponentTheme, ColorPalette, ComponentType, ThemeProperty, ThemeValue, ThemeResult};

/// HSV 转 RGB 实用函数
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let h = h % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::rgb(r + m, g + m, b + m)
}

/// 主题构建器
/// 
/// 提供流式API来创建和定制主题
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// 创建新的主题构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            theme: Theme::new(name, "Custom theme created with ThemeBuilder"),
        }
    }

    /// 基于现有主题创建构建器
    pub fn from_theme(theme: Theme) -> Self {
        Self { theme }
    }

    /// 基于预设主题创建构建器
    pub fn from_preset(preset_name: &str) -> Option<Self> {
        crate::ThemePresets::get_preset(preset_name)
            .map(|theme| Self::from_theme(theme))
    }

    /// 设置主题描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.theme.description = description.into();
        self
    }

    /// 设置主题版本
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.theme.version = version.into();
        self
    }

    /// 设置主题作者
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.theme.author = Some(author.into());
        self
    }

    /// 设置父主题
    pub fn parent(mut self, parent: impl Into<String>) -> Self {
        self.theme.parent = Some(parent.into());
        self
    }

    /// 设置全局主要颜色
    pub fn primary_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::PrimaryColor, ThemeValue::Color(color));
        self
    }

    /// 设置全局次要颜色
    pub fn secondary_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::SecondaryColor, ThemeValue::Color(color));
        self
    }

    /// 设置全局强调颜色
    pub fn accent_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::AccentColor, ThemeValue::Color(color));
        self
    }

    /// 设置背景颜色
    pub fn background_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::BackgroundColor, ThemeValue::Color(color));
        self
    }

    /// 设置文本颜色
    pub fn text_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::TextColor, ThemeValue::Color(color));
        self
    }

    /// 设置边框颜色
    pub fn border_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::BorderColor, ThemeValue::Color(color));
        self
    }

    /// 设置网格颜色
    pub fn grid_color(mut self, color: Color) -> Self {
        self.theme.set_global(ThemeProperty::GridColor, ThemeValue::Color(color));
        self
    }

    /// 设置全局字体大小
    pub fn font_size(mut self, size: f32) -> Self {
        self.theme.set_global(ThemeProperty::FontSize, ThemeValue::Number(size));
        self
    }

    /// 设置全局线条宽度
    pub fn line_width(mut self, width: f32) -> Self {
        self.theme.set_global(ThemeProperty::LineWidth, ThemeValue::Number(width));
        self
    }

    /// 设置全局点大小
    pub fn point_size(mut self, size: f32) -> Self {
        self.theme.set_global(ThemeProperty::PointSize, ThemeValue::Number(size));
        self
    }

    /// 设置透明度
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.theme.set_global(ThemeProperty::Opacity, ThemeValue::Number(opacity.clamp(0.0, 1.0)));
        self
    }

    /// 应用调色板
    pub fn palette(mut self, palette: ColorPalette) -> Self {
        self.theme.set_global(ThemeProperty::PrimaryColor, ThemeValue::Color(palette.primary));
        self.theme.set_global(ThemeProperty::SecondaryColor, ThemeValue::Color(palette.secondary));
        self.theme.set_global(ThemeProperty::AccentColor, ThemeValue::Color(palette.accent));
        self.theme.set_global(ThemeProperty::BackgroundColor, ThemeValue::Color(palette.background));
        self.theme.set_global(ThemeProperty::TextColor, ThemeValue::Color(palette.text));
        
        // 存储系列颜色
        for (i, color) in palette.series.iter().enumerate() {
            self.theme.set_custom(format!("series_color_{}", i), ThemeValue::Color(*color));
        }
        
        self.theme.set_custom("palette_name", ThemeValue::String(palette.name));
        self
    }

    /// 配置散点图主题
    pub fn scatter_plot<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("ScatterPlot");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::ScatterPlot, component_theme);
        self
    }

    /// 配置折线图主题
    pub fn line_plot<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("LinePlot");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::LinePlot, component_theme);
        self
    }

    /// 配置柱状图主题
    pub fn bar_plot<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("BarPlot");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::BarPlot, component_theme);
        self
    }

    /// 配置直方图主题
    pub fn histogram<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Histogram");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Histogram, component_theme);
        self
    }

    /// 配置热力图主题
    pub fn heatmap<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Heatmap");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Heatmap, component_theme);
        self
    }

    /// 配置箱线图主题
    pub fn box_plot<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("BoxPlot");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::BoxPlot, component_theme);
        self
    }

    /// 配置3D散点图主题
    pub fn scatter_3d<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Scatter3D");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Scatter3D, component_theme);
        self
    }

    /// 配置3D表面图主题
    pub fn surface_3d<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Surface3D");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Surface3D, component_theme);
        self
    }

    /// 配置坐标轴主题
    pub fn axis<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Axis");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Axis, component_theme);
        self
    }

    /// 配置网格主题
    pub fn grid<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Grid");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Grid, component_theme);
        self
    }

    /// 配置图例主题
    pub fn legend<F>(mut self, config: F) -> Self 
    where
        F: FnOnce(ComponentThemeBuilder) -> ComponentThemeBuilder,
    {
        let builder = ComponentThemeBuilder::new("Legend");
        let component_theme = config(builder).build();
        self.theme.add_component(ComponentType::Legend, component_theme);
        self
    }

    /// 添加自定义属性
    pub fn custom_property(mut self, key: impl Into<String>, value: ThemeValue) -> Self {
        self.theme.set_custom(key, value);
        self
    }

    /// 构建主题
    pub fn build(self) -> ThemeResult<Theme> {
        self.theme.validate()?;
        Ok(self.theme)
    }

    /// 构建并注册主题到管理器
    pub fn build_and_register(self) -> ThemeResult<()> {
        let theme = self.build()?;
        crate::ThemeManager::instance().register_theme(theme)
    }

    /// 构建、注册并切换到此主题
    pub fn build_register_and_apply(self) -> ThemeResult<()> {
        let theme = self.build()?;
        let theme_name = theme.name.clone();
        crate::ThemeManager::instance().register_theme(theme)?;
        crate::ThemeManager::instance().switch_theme(&theme_name)
    }
}

/// 组件主题构建器
pub struct ComponentThemeBuilder {
    component_theme: ComponentTheme,
}

impl ComponentThemeBuilder {
    /// 创建新的组件主题构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            component_theme: ComponentTheme::new(name),
        }
    }

    /// 设置主要颜色
    pub fn primary_color(mut self, color: Color) -> Self {
        self.component_theme.set_property(ThemeProperty::PrimaryColor, ThemeValue::Color(color));
        self
    }

    /// 设置次要颜色
    pub fn secondary_color(mut self, color: Color) -> Self {
        self.component_theme.set_property(ThemeProperty::SecondaryColor, ThemeValue::Color(color));
        self
    }

    /// 设置强调颜色
    pub fn accent_color(mut self, color: Color) -> Self {
        self.component_theme.set_property(ThemeProperty::AccentColor, ThemeValue::Color(color));
        self
    }

    /// 设置边框颜色
    pub fn border_color(mut self, color: Color) -> Self {
        self.component_theme.set_property(ThemeProperty::BorderColor, ThemeValue::Color(color));
        self
    }

    /// 设置线条宽度
    pub fn line_width(mut self, width: f32) -> Self {
        self.component_theme.set_property(ThemeProperty::LineWidth, ThemeValue::Number(width));
        self
    }

    /// 设置点大小
    pub fn point_size(mut self, size: f32) -> Self {
        self.component_theme.set_property(ThemeProperty::PointSize, ThemeValue::Number(size));
        self
    }

    /// 设置字体大小
    pub fn font_size(mut self, size: f32) -> Self {
        self.component_theme.set_property(ThemeProperty::FontSize, ThemeValue::Number(size));
        self
    }

    /// 设置透明度
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.component_theme.set_property(ThemeProperty::Opacity, ThemeValue::Number(opacity.clamp(0.0, 1.0)));
        self
    }

    /// 设置圆角半径
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.component_theme.set_property(ThemeProperty::BorderRadius, ThemeValue::Number(radius));
        self
    }

    /// 设置悬停状态的颜色
    pub fn hover_color(mut self, color: Color) -> Self {
        self.component_theme.set_state_property("hover", ThemeProperty::PrimaryColor, ThemeValue::Color(color));
        self
    }

    /// 设置激活状态的颜色
    pub fn active_color(mut self, color: Color) -> Self {
        self.component_theme.set_state_property("active", ThemeProperty::PrimaryColor, ThemeValue::Color(color));
        self
    }

    /// 设置禁用状态的颜色
    pub fn disabled_color(mut self, color: Color) -> Self {
        self.component_theme.set_state_property("disabled", ThemeProperty::PrimaryColor, ThemeValue::Color(color));
        self
    }

    /// 构建组件主题
    pub fn build(self) -> ComponentTheme {
        self.component_theme
    }
}

/// 调色板构建器
pub struct PaletteBuilder {
    palette: ColorPalette,
}

impl PaletteBuilder {
    /// 创建新的调色板构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            palette: ColorPalette::new(name, "Custom palette created with PaletteBuilder"),
        }
    }

    /// 设置描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.palette.description = description.into();
        self
    }

    /// 设置主要颜色
    pub fn primary(mut self, color: Color) -> Self {
        self.palette.primary = color;
        self
    }

    /// 设置次要颜色
    pub fn secondary(mut self, color: Color) -> Self {
        self.palette.secondary = color;
        self
    }

    /// 设置强调颜色
    pub fn accent(mut self, color: Color) -> Self {
        self.palette.accent = color;
        self
    }

    /// 设置背景颜色
    pub fn background(mut self, color: Color) -> Self {
        self.palette.background = color;
        self
    }

    /// 设置表面颜色
    pub fn surface(mut self, color: Color) -> Self {
        self.palette.surface = color;
        self
    }

    /// 设置文本颜色
    pub fn text(mut self, color: Color) -> Self {
        self.palette.text = color;
        self
    }

    /// 设置错误颜色
    pub fn error(mut self, color: Color) -> Self {
        self.palette.error = color;
        self
    }

    /// 设置警告颜色
    pub fn warning(mut self, color: Color) -> Self {
        self.palette.warning = color;
        self
    }

    /// 设置成功颜色
    pub fn success(mut self, color: Color) -> Self {
        self.palette.success = color;
        self
    }

    /// 设置信息颜色
    pub fn info(mut self, color: Color) -> Self {
        self.palette.info = color;
        self
    }

    /// 设置系列颜色
    pub fn series(mut self, colors: Vec<Color>) -> Self {
        self.palette.series = colors;
        self
    }

    /// 添加系列颜色
    pub fn add_series_color(mut self, color: Color) -> Self {
        self.palette.series.push(color);
        self
    }

    /// 从HSV生成系列颜色
    pub fn generate_series_hsv(mut self, count: usize, saturation: f32, value: f32) -> Self {
        self.palette.series.clear();
        for i in 0..count {
            let hue = (i as f32 * 360.0 / count as f32) % 360.0;
            self.palette.series.push(hsv_to_rgb(hue, saturation, value));
        }
        self
    }

    /// 构建调色板
    pub fn build(self) -> ColorPalette {
        self.palette
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_builder() {
        let theme = ThemeBuilder::new("test_theme")
            .description("Test theme built with builder")
            .version("2.0.0")
            .author("Test Author")
            .primary_color(Color::rgb(1.0, 0.0, 0.0))
            .secondary_color(Color::rgb(0.0, 1.0, 0.0))
            .background_color(Color::rgb(1.0, 1.0, 1.0))
            .text_color(Color::rgb(0.0, 0.0, 0.0))
            .font_size(14.0)
            .line_width(2.0)
            .build()
            .unwrap();

        assert_eq!(theme.name, "test_theme");
        assert_eq!(theme.description, "Test theme built with builder");
        assert_eq!(theme.version, "2.0.0");
        assert_eq!(theme.author, Some("Test Author".to_string()));
        
        assert_eq!(
            theme.get_global(&ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)))
        );
    }

    #[test]
    fn test_component_theme_builder() {
        let theme = ThemeBuilder::new("test_with_components")
            .background_color(Color::rgb(1.0, 1.0, 1.0))
            .text_color(Color::rgb(0.0, 0.0, 0.0))
            .primary_color(Color::rgb(0.5, 0.5, 0.5))
            .scatter_plot(|builder| {
                builder
                    .primary_color(Color::rgb(1.0, 0.0, 0.0))
                    .point_size(6.0)
                    .hover_color(Color::rgb(1.0, 0.5, 0.5))
            })
            .line_plot(|builder| {
                builder
                    .primary_color(Color::rgb(0.0, 1.0, 0.0))
                    .line_width(3.0)
            })
            .build()
            .unwrap();

        let scatter_theme = theme.get_component(&ComponentType::ScatterPlot).unwrap();
        assert_eq!(
            scatter_theme.get_color(&ThemeProperty::PrimaryColor),
            Some(Color::rgb(1.0, 0.0, 0.0))
        );
        assert_eq!(scatter_theme.get_number(&ThemeProperty::PointSize), Some(6.0));

        let line_theme = theme.get_component(&ComponentType::LinePlot).unwrap();
        assert_eq!(
            line_theme.get_color(&ThemeProperty::PrimaryColor),
            Some(Color::rgb(0.0, 1.0, 0.0))
        );
        assert_eq!(line_theme.get_number(&ThemeProperty::LineWidth), Some(3.0));
    }

    #[test]
    fn test_palette_builder() {
        let palette = PaletteBuilder::new("test_palette")
            .description("Test palette")
            .primary(Color::rgb(1.0, 0.0, 0.0))
            .secondary(Color::rgb(0.0, 1.0, 0.0))
            .background(Color::rgb(1.0, 1.0, 1.0))
            .text(Color::rgb(0.0, 0.0, 0.0))
            .add_series_color(Color::rgb(0.5, 0.5, 0.5))
            .add_series_color(Color::rgb(0.8, 0.2, 0.3))
            .build();

        assert_eq!(palette.name, "test_palette");
        assert_eq!(palette.primary, Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(palette.secondary, Color::rgb(0.0, 1.0, 0.0));
        assert!(palette.series.len() >= 2); // 至少包含我们添加的两个颜色
    }

    #[test]
    fn test_theme_with_palette() {
        let palette = PaletteBuilder::new("custom_palette")
            .primary(Color::rgb(0.2, 0.4, 0.8))
            .secondary(Color::rgb(0.8, 0.4, 0.2))
            .background(Color::rgb(0.95, 0.95, 0.95))
            .text(Color::rgb(0.1, 0.1, 0.1))
            .build();

        let theme = ThemeBuilder::new("palette_theme")
            .palette(palette)
            .build()
            .unwrap();

        assert_eq!(
            theme.get_global(&ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(0.2, 0.4, 0.8)))
        );
        assert_eq!(
            theme.get_global(&ThemeProperty::BackgroundColor),
            Some(&ThemeValue::Color(Color::rgb(0.95, 0.95, 0.95)))
        );
    }

    #[test]
    fn test_from_preset_builder() {
        let theme = ThemeBuilder::from_preset("dark")
            .unwrap()
            .description("Modified dark theme")
            .primary_color(Color::rgb(0.9, 0.1, 0.1))
            .build()
            .unwrap();

        assert_eq!(theme.description, "Modified dark theme");
        assert_eq!(
            theme.get_global(&ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(0.9, 0.1, 0.1)))
        );
    }

    #[test]
    fn test_palette_hsv_generation() {
        let palette = PaletteBuilder::new("hsv_palette")
            .generate_series_hsv(6, 0.8, 0.9)
            .build();

        assert_eq!(palette.series.len(), 6);
        
        // 验证颜色是HSV生成的（应该有不同的色相）
        for color in &palette.series {
            // 简化的HSV转换验证
            let max_channel = color.r.max(color.g).max(color.b);
            let min_channel = color.r.min(color.g).min(color.b);
            let saturation = if max_channel > 0.0 { (max_channel - min_channel) / max_channel } else { 0.0 };
            
            // 饱和度应该接近我们设置的值
            assert!((saturation - 0.8).abs() < 0.3, "饱和度不匹配: {}", saturation);
        }
    }

    #[test]
    fn test_component_states() {
        let theme = ThemeBuilder::new("state_test")
            .background_color(Color::rgb(1.0, 1.0, 1.0))
            .text_color(Color::rgb(0.0, 0.0, 0.0))
            .primary_color(Color::rgb(0.5, 0.5, 0.5))
            .scatter_plot(|builder| {
                builder
                    .primary_color(Color::rgb(0.2, 0.6, 0.8))
                    .hover_color(Color::rgb(0.4, 0.8, 1.0))
                    .active_color(Color::rgb(0.1, 0.4, 0.6))
                    .disabled_color(Color::rgb(0.7, 0.7, 0.7))
            })
            .build()
            .unwrap();

        let scatter_theme = theme.get_component(&ComponentType::ScatterPlot).unwrap();
        
        assert_eq!(
            scatter_theme.get_state_property("hover", &ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(0.4, 0.8, 1.0)))
        );
        assert_eq!(
            scatter_theme.get_state_property("active", &ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(0.1, 0.4, 0.6)))
        );
        assert_eq!(
            scatter_theme.get_state_property("disabled", &ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(0.7, 0.7, 0.7)))
        );
    }
}
