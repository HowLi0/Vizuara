use serde::{Deserialize, Serialize};

/// RGBA 颜色表示
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32, 
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// 创建新的颜色
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// RGB 颜色（不透明）
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    /// 从 hex 字符串创建颜色 (如 "#ff0000")
    pub fn from_hex(hex: &str) -> Result<Self, crate::VizuaraError> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(crate::VizuaraError::InvalidColor(format!("Invalid hex color: {}", hex)));
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| 
            crate::VizuaraError::InvalidColor(format!("Invalid hex color: {}", hex)))?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| 
            crate::VizuaraError::InvalidColor(format!("Invalid hex color: {}", hex)))?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| 
            crate::VizuaraError::InvalidColor(format!("Invalid hex color: {}", hex)))?;
            
        Ok(Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
    }
    
    /// 预定义颜色常量
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

/// 线条样式
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

/// 点的样式
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MarkerStyle {
    Circle,
    Square, 
    Triangle,
    Cross,
    Plus,
    Diamond,
}

/// 视觉样式配置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Style {
    /// 填充颜色
    pub fill_color: Option<Color>,
    /// 边框颜色
    pub stroke_color: Option<Color>,
    /// 边框宽度
    pub stroke_width: f32,
    /// 线条样式
    pub line_style: LineStyle,
    /// 点标记样式
    pub marker_style: MarkerStyle,
    /// 点大小
    pub marker_size: f32,
    /// 透明度 (0.0 - 1.0)
    pub opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill_color: Some(Color::BLUE),
            stroke_color: Some(Color::BLACK),
            stroke_width: 1.0,
            line_style: LineStyle::Solid,
            marker_style: MarkerStyle::Circle,
            marker_size: 3.0,
            opacity: 1.0,
        }
    }
}

impl Style {
    /// 创建新的样式
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置填充颜色
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }
    
    /// 设置边框颜色和宽度
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }
    
    /// 设置点标记样式
    pub fn marker(mut self, style: MarkerStyle, size: f32) -> Self {
        self.marker_style = style;
        self.marker_size = size;
        self
    }
    
    /// 设置透明度
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

// 为Color实现运算符重载
impl std::ops::Add for Color {
    type Output = Color;
    
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    
    fn mul(self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a * scalar,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;
    
    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a,
        }
    }
}
