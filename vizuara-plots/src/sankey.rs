//! 桑基图实现
//!
//! 用于可视化流量数据，显示从源到目标的流动

use crate::PlotArea;
use nalgebra::Point2;
use vizuara_core::{Color, HorizontalAlign, Primitive, VerticalAlign};

/// 布局计算结果类型别名
/// 格式: (id, x, y, width, height)
type NodeLayout = Vec<(String, f32, f32, f32, f32)>;
/// 格式: (x1, y1, x2, y2, value)
type LinkLayout = Vec<(f32, f32, f32, f32, f32)>;

/// 桑基图节点
#[derive(Debug, Clone)]
pub struct SankeyNode {
    pub id: String,
    pub color: Color,
    pub label: Option<String>,
}

impl SankeyNode {
    /// 创建新节点
    pub fn new(id: String, color: Color) -> Self {
        Self {
            id: id.clone(),
            color,
            label: Some(id),
        }
    }
}

/// 桑基图链接
#[derive(Debug, Clone)]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: f32,
    pub color: Option<Color>,
    pub label: Option<String>,
}

impl SankeyLink {
    /// 创建新链接
    pub fn new(source: String, target: String, value: f32, color: Color) -> Self {
        Self {
            source,
            target,
            value,
            color: Some(color),
            label: None,
        }
    }
}

/// 桑基图样式
#[derive(Debug, Clone)]
pub struct SankeyStyle {
    pub node_width: f32,
    pub node_padding: f32,
    pub link_opacity: f32,
    pub show_node_labels: bool,
    pub show_link_labels: bool,
    pub label_size: f32,
    pub label_color: Color,
    pub default_node_color: Color,
    pub default_link_color: Color,
}

impl Default for SankeyStyle {
    fn default() -> Self {
        Self {
            node_width: 20.0,
            node_padding: 10.0,
            link_opacity: 0.6,
            show_node_labels: true,
            show_link_labels: false,
            label_size: 12.0,
            label_color: Color::rgb(0.2, 0.2, 0.2),
            default_node_color: Color::rgb(0.6, 0.6, 0.6),
            default_link_color: Color::rgba(0.4, 0.4, 0.4, 0.6),
        }
    }
}

/// 桑基图
#[derive(Debug, Clone)]
pub struct SankeyDiagram {
    nodes: Vec<SankeyNode>,
    links: Vec<SankeyLink>,
    style: SankeyStyle,
    title: Option<String>,
}

impl Default for SankeyDiagram {
    fn default() -> Self {
        Self::new()
    }
}

impl SankeyDiagram {
    /// 创建新的桑基图
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            style: SankeyStyle::default(),
            title: None,
        }
    }

    /// 添加节点
    pub fn add_node(mut self, node: SankeyNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// 添加链接
    pub fn add_link(mut self, link: SankeyLink) -> Self {
        self.links.push(link);
        self
    }

    /// 设置标题
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// 设置节点宽度
    pub fn node_width(mut self, width: f32) -> Self {
        self.style.node_width = width;
        self
    }

    /// 设置节点间距
    pub fn node_padding(mut self, padding: f32) -> Self {
        self.style.node_padding = padding;
        self
    }

    /// 设置链接透明度
    pub fn link_opacity(mut self, opacity: f32) -> Self {
        self.style.link_opacity = opacity;
        self
    }

    /// 设置是否显示数值
    pub fn show_values(mut self, show: bool) -> Self {
        self.style.show_link_labels = show;
        self
    }

    /// 计算简单的层次布局
    fn compute_layout(&self, plot_area: PlotArea) -> (NodeLayout, LinkLayout) {
        let mut nodes_layout = Vec::new();
        let mut links_layout = Vec::new();

        // 简单的垂直分布
        let available_height = plot_area.height - 100.0;
        let node_height = available_height / self.nodes.len() as f32;

        for (i, node) in self.nodes.iter().enumerate() {
            let y = plot_area.y + 50.0 + i as f32 * node_height;
            let x = if i < self.nodes.len() / 2 {
                plot_area.x + 50.0 // 源节点
            } else {
                plot_area.x + plot_area.width - 50.0 - self.style.node_width // 目标节点
            };

            nodes_layout.push((
                node.id.clone(),
                x,
                y,
                self.style.node_width,
                node_height * 0.8,
            ));
        }

        // 简单的链接布局
        for link in &self.links {
            // 找到源和目标节点的位置
            let source_pos = nodes_layout
                .iter()
                .find(|(id, _, _, _, _)| id == &link.source);
            let target_pos = nodes_layout
                .iter()
                .find(|(id, _, _, _, _)| id == &link.target);

            if let (Some((_, sx, sy, sw, sh)), Some((_, tx, ty, _, _))) = (source_pos, target_pos) {
                links_layout.push((
                    sx + sw,
                    sy + sh / 2.0,
                    *tx,
                    ty + sh / 2.0,
                    link.value / 100.0, // 简单的粗细映射
                ));
            }
        }

        (nodes_layout, links_layout)
    }
}

impl SankeyDiagram {
    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        let (nodes_layout, links_layout) = self.compute_layout(plot_area);

        // 渲染链接（在节点下方）
        for (i, (x1, y1, x2, y2, thickness)) in links_layout.iter().enumerate() {
            let link = &self.links[i];
            let color = link.color.unwrap_or(self.style.default_link_color);

            // 简单的直线连接
            let points = vec![Point2::new(*x1, *y1), Point2::new(*x2, *y2)];

            primitives.push(Primitive::Polyline {
                points,
                color,
                width: thickness.max(2.0),
            });
        }

        // 渲染节点
        for (node_id, x, y, width, height) in nodes_layout {
            let node = self.nodes.iter().find(|n| n.id == node_id).unwrap();

            primitives.push(Primitive::RectangleStyled {
                min: Point2::new(x, y),
                max: Point2::new(x + width, y + height),
                fill: node.color,
                stroke: Some((Color::rgb(0.3, 0.3, 0.3), 1.0)),
            });

            // 添加标签
            if self.style.show_node_labels {
                if let Some(label) = &node.label {
                    primitives.push(Primitive::Text {
                        position: Point2::new(x + width / 2.0, y + height / 2.0),
                        content: label.clone(),
                        size: self.style.label_size,
                        color: self.style.label_color,
                        h_align: HorizontalAlign::Center,
                        v_align: VerticalAlign::Middle,
                    });
                }
            }
        }

        // 添加标题
        if let Some(title) = &self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(plot_area.x + plot_area.width / 2.0, plot_area.y + 20.0),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Middle,
            });
        }

        primitives
    }
}
