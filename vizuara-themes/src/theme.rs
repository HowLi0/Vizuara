use crate::{ComponentType, ThemeError, ThemeProperty, ThemeResult, ThemeValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vizuara_core::{Color, Style};

/// 主题定义
///
/// 包含所有组件的样式配置，支持主题继承和自定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 主题描述
    pub description: String,
    /// 主题版本
    pub version: String,
    /// 父主题名称（用于继承）
    pub parent: Option<String>,
    /// 主题作者
    pub author: Option<String>,
    /// 组件样式配置
    pub components: HashMap<ComponentType, ComponentTheme>,
    /// 全局设置
    pub globals: HashMap<ThemeProperty, ThemeValue>,
    /// 自定义属性
    pub custom: HashMap<String, ThemeValue>,
}

impl Theme {
    /// 创建新的主题
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: "1.0.0".to_string(),
            parent: None,
            author: None,
            components: HashMap::new(),
            globals: HashMap::new(),
            custom: HashMap::new(),
        }
    }

    /// 设置主题版本
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// 设置父主题
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// 设置主题作者
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// 添加组件主题
    pub fn add_component(&mut self, component_type: ComponentType, theme: ComponentTheme) {
        self.components.insert(component_type, theme);
    }

    /// 设置全局属性
    pub fn set_global(&mut self, property: ThemeProperty, value: ThemeValue) {
        self.globals.insert(property, value);
    }

    /// 设置自定义属性
    pub fn set_custom(&mut self, key: impl Into<String>, value: ThemeValue) {
        self.custom.insert(key.into(), value);
    }

    /// 获取组件主题
    pub fn get_component(&self, component_type: &ComponentType) -> Option<&ComponentTheme> {
        self.components.get(component_type)
    }

    /// 获取全局属性
    pub fn get_global(&self, property: &ThemeProperty) -> Option<&ThemeValue> {
        self.globals.get(property)
    }

    /// 获取自定义属性
    pub fn get_custom(&self, key: &str) -> Option<&ThemeValue> {
        self.custom.get(key)
    }

    /// 将主题应用到样式
    pub fn apply_to_style(&self, component_type: &ComponentType, base_style: Style) -> Style {
        let mut style = base_style;

        // 应用全局样式
        if let Some(ThemeValue::Color(color)) = self.get_global(&ThemeProperty::PrimaryColor) {
            style = style.fill_color(*color);
        }

        if let Some(ThemeValue::Number(width)) = self.get_global(&ThemeProperty::LineWidth) {
            if let Some(ThemeValue::Color(color)) = self.get_global(&ThemeProperty::BorderColor) {
                style = style.stroke(*color, *width);
            }
        }

        // 应用组件特定样式
        if let Some(component_theme) = self.get_component(component_type) {
            style = component_theme.apply_to_style(style);
        }

        style
    }

    /// 获取组件的主要颜色
    pub fn get_primary_color(&self, component_type: &ComponentType) -> Color {
        // 优先使用组件特定颜色
        if let Some(component_theme) = self.get_component(component_type) {
            if let Some(color) = component_theme.get_color(&ThemeProperty::PrimaryColor) {
                return color;
            }
        }

        // 回退到全局主要颜色
        if let Some(ThemeValue::Color(color)) = self.get_global(&ThemeProperty::PrimaryColor) {
            return *color;
        }

        // 默认颜色
        Color::rgb(0.2, 0.6, 0.8)
    }

    /// 获取组件的次要颜色
    pub fn get_secondary_color(&self, component_type: &ComponentType) -> Color {
        if let Some(component_theme) = self.get_component(component_type) {
            if let Some(color) = component_theme.get_color(&ThemeProperty::SecondaryColor) {
                return color;
            }
        }

        if let Some(ThemeValue::Color(color)) = self.get_global(&ThemeProperty::SecondaryColor) {
            return *color;
        }

        Color::rgb(0.8, 0.4, 0.2)
    }

    /// 获取背景颜色
    pub fn get_background_color(&self) -> Color {
        if let Some(ThemeValue::Color(color)) = self.get_global(&ThemeProperty::BackgroundColor) {
            return *color;
        }
        Color::rgb(1.0, 1.0, 1.0) // 默认白色背景
    }

    /// 获取文本颜色
    pub fn get_text_color(&self) -> Color {
        if let Some(ThemeValue::Color(color)) = self.get_global(&ThemeProperty::TextColor) {
            return *color;
        }
        Color::rgb(0.2, 0.2, 0.2) // 默认深灰色文本
    }

    /// 验证主题配置的有效性
    pub fn validate(&self) -> ThemeResult<()> {
        if self.name.is_empty() {
            return Err(ThemeError::InvalidTheme("主题名称不能为空".to_string()));
        }

        if self.version.is_empty() {
            return Err(ThemeError::InvalidTheme("主题版本不能为空".to_string()));
        }

        // 验证必需的全局属性
        let required_globals = [
            ThemeProperty::PrimaryColor,
            ThemeProperty::BackgroundColor,
            ThemeProperty::TextColor,
        ];

        for property in &required_globals {
            if !self.globals.contains_key(property) {
                return Err(ThemeError::InvalidTheme(format!(
                    "缺少必需的全局属性: {:?}",
                    property
                )));
            }
        }

        Ok(())
    }
}

/// 组件主题
///
/// 定义单个组件的样式配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTheme {
    /// 组件名称
    pub name: String,
    /// 组件属性配置
    pub properties: HashMap<ThemeProperty, ThemeValue>,
    /// 状态特定的样式（如悬停、激活等）
    pub states: HashMap<String, HashMap<ThemeProperty, ThemeValue>>,
}

impl ComponentTheme {
    /// 创建新的组件主题
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            properties: HashMap::new(),
            states: HashMap::new(),
        }
    }

    /// 设置属性
    pub fn set_property(&mut self, property: ThemeProperty, value: ThemeValue) {
        self.properties.insert(property, value);
    }

    /// 获取属性
    pub fn get_property(&self, property: &ThemeProperty) -> Option<&ThemeValue> {
        self.properties.get(property)
    }

    /// 获取颜色属性
    pub fn get_color(&self, property: &ThemeProperty) -> Option<Color> {
        self.get_property(property)?.as_color()
    }

    /// 获取数值属性
    pub fn get_number(&self, property: &ThemeProperty) -> Option<f32> {
        self.get_property(property)?.as_number()
    }

    /// 设置状态样式
    pub fn set_state_property(
        &mut self,
        state: impl Into<String>,
        property: ThemeProperty,
        value: ThemeValue,
    ) {
        self.states
            .entry(state.into())
            .or_default()
            .insert(property, value);
    }

    /// 获取状态样式
    pub fn get_state_property(&self, state: &str, property: &ThemeProperty) -> Option<&ThemeValue> {
        self.states.get(state)?.get(property)
    }

    /// 将组件主题应用到样式
    pub fn apply_to_style(&self, mut style: Style) -> Style {
        // 应用填充颜色
        if let Some(color) = self.get_color(&ThemeProperty::PrimaryColor) {
            style = style.fill_color(color);
        }

        // 应用边框样式
        if let Some(color) = self.get_color(&ThemeProperty::BorderColor) {
            let width = self.get_number(&ThemeProperty::LineWidth).unwrap_or(1.0);
            style = style.stroke(color, width);
        }

        style
    }

    /// 构建器模式：设置主要颜色
    pub fn with_primary_color(mut self, color: Color) -> Self {
        self.set_property(ThemeProperty::PrimaryColor, ThemeValue::Color(color));
        self
    }

    /// 构建器模式：设置次要颜色
    pub fn with_secondary_color(mut self, color: Color) -> Self {
        self.set_property(ThemeProperty::SecondaryColor, ThemeValue::Color(color));
        self
    }

    /// 构建器模式：设置边框颜色
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.set_property(ThemeProperty::BorderColor, ThemeValue::Color(color));
        self
    }

    /// 构建器模式：设置线条宽度
    pub fn with_line_width(mut self, width: f32) -> Self {
        self.set_property(ThemeProperty::LineWidth, ThemeValue::Number(width));
        self
    }

    /// 构建器模式：设置点大小
    pub fn with_point_size(mut self, size: f32) -> Self {
        self.set_property(ThemeProperty::PointSize, ThemeValue::Number(size));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("Test Theme", "A test theme")
            .with_version("1.2.0")
            .with_author("Test Author");

        assert_eq!(theme.name, "Test Theme");
        assert_eq!(theme.description, "A test theme");
        assert_eq!(theme.version, "1.2.0");
        assert_eq!(theme.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_theme_global_properties() {
        let mut theme = Theme::new("Test", "Test");

        theme.set_global(
            ThemeProperty::PrimaryColor,
            ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)),
        );
        theme.set_global(ThemeProperty::FontSize, ThemeValue::Number(14.0));

        assert_eq!(
            theme.get_global(&ThemeProperty::PrimaryColor),
            Some(&ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)))
        );
        assert_eq!(
            theme.get_global(&ThemeProperty::FontSize),
            Some(&ThemeValue::Number(14.0))
        );
    }

    #[test]
    fn test_component_theme() {
        let component_theme = ComponentTheme::new("ScatterPlot")
            .with_primary_color(Color::rgb(0.2, 0.6, 0.8))
            .with_line_width(2.0)
            .with_point_size(5.0);

        assert_eq!(component_theme.name, "ScatterPlot");
        assert_eq!(
            component_theme.get_color(&ThemeProperty::PrimaryColor),
            Some(Color::rgb(0.2, 0.6, 0.8))
        );
        assert_eq!(
            component_theme.get_number(&ThemeProperty::LineWidth),
            Some(2.0)
        );
        assert_eq!(
            component_theme.get_number(&ThemeProperty::PointSize),
            Some(5.0)
        );
    }

    #[test]
    fn test_theme_component_integration() {
        let mut theme = Theme::new("Test", "Test");

        let scatter_theme =
            ComponentTheme::new("ScatterPlot").with_primary_color(Color::rgb(1.0, 0.0, 0.0));

        theme.add_component(ComponentType::ScatterPlot, scatter_theme);

        let component = theme.get_component(&ComponentType::ScatterPlot).unwrap();
        assert_eq!(component.name, "ScatterPlot");
        assert_eq!(
            component.get_color(&ThemeProperty::PrimaryColor),
            Some(Color::rgb(1.0, 0.0, 0.0))
        );
    }

    #[test]
    fn test_theme_validation() {
        let mut theme = Theme::new("", "Test");
        assert!(theme.validate().is_err());

        theme.name = "Test".to_string();
        theme.set_global(
            ThemeProperty::PrimaryColor,
            ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)),
        );
        theme.set_global(
            ThemeProperty::BackgroundColor,
            ThemeValue::Color(Color::rgb(1.0, 1.0, 1.0)),
        );
        theme.set_global(
            ThemeProperty::TextColor,
            ThemeValue::Color(Color::rgb(0.0, 0.0, 0.0)),
        );

        assert!(theme.validate().is_ok());
    }

    #[test]
    fn test_get_primary_color() {
        let mut theme = Theme::new("Test", "Test");
        theme.set_global(
            ThemeProperty::PrimaryColor,
            ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)),
        );

        // 测试全局颜色
        assert_eq!(
            theme.get_primary_color(&ComponentType::ScatterPlot),
            Color::rgb(1.0, 0.0, 0.0)
        );

        // 测试组件特定颜色覆盖全局颜色
        let scatter_theme =
            ComponentTheme::new("ScatterPlot").with_primary_color(Color::rgb(0.0, 1.0, 0.0));
        theme.add_component(ComponentType::ScatterPlot, scatter_theme);

        assert_eq!(
            theme.get_primary_color(&ComponentType::ScatterPlot),
            Color::rgb(0.0, 1.0, 0.0)
        );
    }
}
