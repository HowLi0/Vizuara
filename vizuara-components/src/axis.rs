use vizuara_core::{Primitive, Scale, LinearScale, Color};
use nalgebra::Point2;

/// 坐标轴方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisDirection {
    Horizontal,
    Vertical,
}

/// 坐标轴组件
#[derive(Debug, Clone)]
pub struct Axis {
    direction: AxisDirection,
    scale: LinearScale,
    position: (f32, f32), // 轴的起始位置
    length: f32,          // 轴的长度
    title: Option<String>,
    tick_count: usize,
    style: AxisStyle,
}

/// 坐标轴样式
#[derive(Debug, Clone)]
pub struct AxisStyle {
    pub axis_color: Color,
    pub tick_color: Color,
    pub label_color: Color,
    pub tick_length: f32,
    pub label_size: f32,
    pub title_size: f32,
}

impl Default for AxisStyle {
    fn default() -> Self {
        Self {
            axis_color: Color::rgb(0.2, 0.2, 0.2),
            tick_color: Color::rgb(0.4, 0.4, 0.4),
            label_color: Color::rgb(0.1, 0.1, 0.1),
            tick_length: 5.0,
            label_size: 12.0,
            title_size: 14.0,
        }
    }
}

impl Axis {
    /// 创建新的坐标轴
    pub fn new(
        direction: AxisDirection,
        scale: LinearScale,
        position: (f32, f32),
        length: f32,
    ) -> Self {
        Self {
            direction,
            scale,
            position,
            length,
            title: None,
            tick_count: 5,
            style: AxisStyle::default(),
        }
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置刻度数量
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }

    /// 设置样式
    pub fn style(mut self, style: AxisStyle) -> Self {
        self.style = style;
        self
    }

    /// 生成坐标轴的渲染图元
    pub fn generate_primitives(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        // 1. 绘制主轴线
        let (start, end) = self.axis_line_points();
        primitives.push(Primitive::Line { start, end });

        // 2. 生成刻度和标签
        let ticks = self.scale.ticks(self.tick_count);
        for &tick_value in &ticks {
            let position = self.value_to_position(tick_value);
            
            // 刻度线
            let (tick_start, tick_end) = self.tick_line_points(position);
            primitives.push(Primitive::Line {
                start: tick_start,
                end: tick_end,
            });

            // 刻度标签
            let label_position = self.label_position(position);
            let label_text = format!("{:.1}", tick_value);
            primitives.push(Primitive::Text {
                position: label_position,
                content: label_text,
                size: self.style.label_size,
                color: self.style.label_color,
                h_align: match self.direction { AxisDirection::Horizontal => vizuara_core::HorizontalAlign::Center, AxisDirection::Vertical => vizuara_core::HorizontalAlign::Right },
                v_align: match self.direction { AxisDirection::Horizontal => vizuara_core::VerticalAlign::Top, AxisDirection::Vertical => vizuara_core::VerticalAlign::Middle },
            });
        }

        // 3. 添加轴标题（如果有）
        if let Some(ref title) = self.title {
            let title_position = self.title_position();
            primitives.push(Primitive::Text {
                position: title_position,
                content: title.clone(),
                size: self.style.title_size,
                color: self.style.label_color,
                h_align: match self.direction { AxisDirection::Horizontal => vizuara_core::HorizontalAlign::Center, AxisDirection::Vertical => vizuara_core::HorizontalAlign::Right },
                v_align: match self.direction { AxisDirection::Horizontal => vizuara_core::VerticalAlign::Top, AxisDirection::Vertical => vizuara_core::VerticalAlign::Middle },
            });
        }

        primitives
    }

    /// 计算轴线的起点和终点
    fn axis_line_points(&self) -> (Point2<f32>, Point2<f32>) {
        let (x, y) = self.position;
        match self.direction {
            AxisDirection::Horizontal => (
                Point2::new(x, y),
                Point2::new(x + self.length, y),
            ),
            AxisDirection::Vertical => (
                Point2::new(x, y),
                Point2::new(x, y + self.length),
            ),
        }
    }

    /// 将数据值转换为轴上的位置
    fn value_to_position(&self, value: f32) -> f32 {
        let normalized = self.scale.normalize(value);
        match self.direction {
            AxisDirection::Horizontal => self.position.0 + normalized * self.length,
            AxisDirection::Vertical => self.position.1 + normalized * self.length,
        }
    }

    /// 计算刻度线的起点和终点
    fn tick_line_points(&self, position: f32) -> (Point2<f32>, Point2<f32>) {
        match self.direction {
            AxisDirection::Horizontal => (
                Point2::new(position, self.position.1),
                Point2::new(position, self.position.1 - self.style.tick_length),
            ),
            AxisDirection::Vertical => (
                Point2::new(self.position.0, position),
                Point2::new(self.position.0 - self.style.tick_length, position),
            ),
        }
    }

    /// 计算标签位置
    fn label_position(&self, position: f32) -> Point2<f32> {
        match self.direction {
            AxisDirection::Horizontal => Point2::new(
                position,
                self.position.1 - self.style.tick_length - self.style.label_size,
            ),
            AxisDirection::Vertical => Point2::new(
                self.position.0 - self.style.tick_length - 30.0, // 为文本留出空间
                position,
            ),
        }
    }

    /// 计算标题位置
    fn title_position(&self) -> Point2<f32> {
        match self.direction {
            AxisDirection::Horizontal => Point2::new(
                self.position.0 + self.length / 2.0,
                self.position.1 - self.style.tick_length - self.style.label_size - self.style.title_size - 10.0,
            ),
            AxisDirection::Vertical => Point2::new(
                self.position.0 - self.style.tick_length - 60.0,
                self.position.1 + self.length / 2.0,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_creation() {
        let scale = LinearScale::new(0.0, 10.0);
        let axis = Axis::new(
            AxisDirection::Horizontal,
            scale,
            (100.0, 500.0),
            400.0,
        );

        assert_eq!(axis.direction, AxisDirection::Horizontal);
        assert_eq!(axis.position, (100.0, 500.0));
        assert_eq!(axis.length, 400.0);
    }

    #[test]
    fn test_primitive_generation() {
        let scale = LinearScale::new(0.0, 10.0);
        let axis = Axis::new(
            AxisDirection::Horizontal,
            scale,
            (100.0, 500.0),
            400.0,
        )
        .title("X Axis")
        .tick_count(5);

        let primitives = axis.generate_primitives();
        
        // 应该包含：1个主轴线 + 5个刻度线 + 5个标签 + 1个标题 = 12个图元
        assert_eq!(primitives.len(), 12);
    }
}
