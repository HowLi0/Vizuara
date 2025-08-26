use serde::{Deserialize, Serialize};
use nalgebra::{Point2, Point3};
use crate::Color;

/// 水平对齐
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HorizontalAlign { Left, Center, Right }

/// 垂直对齐
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VerticalAlign { Top, Middle, Baseline, Bottom }

/// 渲染图元的基础枚举
/// 这些是渲染器能够处理的基本几何元素
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    /// 单个点
    Point(Point2<f32>),
    /// 多个点（用于散点图）
    Points(Vec<Point2<f32>>),
    /// 线段
    Line { start: Point2<f32>, end: Point2<f32> },
    /// 连续线条（用于折线图）
    LineStrip(Vec<Point2<f32>>),
    /// 矩形
    Rectangle {
        min: Point2<f32>,
        max: Point2<f32>,
    },
    /// 带样式的矩形（包含填充与可选描边）
    RectangleStyled {
        min: Point2<f32>,
        max: Point2<f32>,
        fill: Color,
        stroke: Option<(Color, f32)>,
    },
    /// 圆形
    Circle {
        center: Point2<f32>,
        radius: f32,
    },
    /// 文本（带颜色与对齐）
    Text {
        position: Point2<f32>,
        content: String,
        size: f32,
        color: Color,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
    },
    /// 三角形列表（用于复杂几何）
    TriangleList(Vec<Point2<f32>>),
    /// 3D点（用于3D可视化）
    Point3D(Point3<f32>),
    /// 3D线条
    Line3D { start: Point3<f32>, end: Point3<f32> },
}

impl Primitive {
    /// 获取图元的边界框
    pub fn bounds(&self) -> Option<(Point2<f32>, Point2<f32>)> {
        match self {
            Primitive::Point(p) => Some((*p, *p)),
            Primitive::Points(points) => {
                if points.is_empty() {
                    return None;
                }
                let mut min_x = points[0].x;
                let mut min_y = points[0].y;
                let mut max_x = points[0].x;
                let mut max_y = points[0].y;
                
                for point in points {
                    min_x = min_x.min(point.x);
                    min_y = min_y.min(point.y);
                    max_x = max_x.max(point.x);
                    max_y = max_y.max(point.y);
                }
                
                Some((Point2::new(min_x, min_y), Point2::new(max_x, max_y)))
            }
            Primitive::Line { start, end } => {
                let min_x = start.x.min(end.x);
                let min_y = start.y.min(end.y);
                let max_x = start.x.max(end.x);
                let max_y = start.y.max(end.y);
                Some((Point2::new(min_x, min_y), Point2::new(max_x, max_y)))
            }
            Primitive::LineStrip(points) => {
                if points.is_empty() {
                    return None;
                }
                let mut min_x = points[0].x;
                let mut min_y = points[0].y;
                let mut max_x = points[0].x;
                let mut max_y = points[0].y;
                
                for point in points {
                    min_x = min_x.min(point.x);
                    min_y = min_y.min(point.y);
                    max_x = max_x.max(point.x);
                    max_y = max_y.max(point.y);
                }
                
                Some((Point2::new(min_x, min_y), Point2::new(max_x, max_y)))
            }
            Primitive::Rectangle { min, max } => Some((*min, *max)),
            Primitive::RectangleStyled { min, max, .. } => Some((*min, *max)),
            Primitive::Circle { center, radius } => {
                let min = Point2::new(center.x - radius, center.y - radius);
                let max = Point2::new(center.x + radius, center.y + radius);
                Some((min, max))
            }
            Primitive::Text { position, .. } => Some((*position, *position)),
            Primitive::TriangleList(points) => {
                if points.is_empty() {
                    return None;
                }
                let mut min_x = points[0].x;
                let mut min_y = points[0].y;
                let mut max_x = points[0].x;
                let mut max_y = points[0].y;
                
                for point in points {
                    min_x = min_x.min(point.x);
                    min_y = min_y.min(point.y);
                    max_x = max_x.max(point.x);
                    max_y = max_y.max(point.y);
                }
                
                Some((Point2::new(min_x, min_y), Point2::new(max_x, max_y)))
            }
            // 3D 图元暂时投影到 2D
            Primitive::Point3D(p) => Some((Point2::new(p.x, p.y), Point2::new(p.x, p.y))),
            Primitive::Line3D { start, end } => {
                let min_x = start.x.min(end.x);
                let min_y = start.y.min(end.y);
                let max_x = start.x.max(end.x);
                let max_y = start.y.max(end.y);
                Some((Point2::new(min_x, min_y), Point2::new(max_x, max_y)))
            }
        }
    }
}
