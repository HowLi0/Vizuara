//! 桑基图实现
//!
//! 用于可视化流量数据，显示从源到目标的流动

use crate::PlotArea;
use vizuara_core::{Color, Primitive, HorizontalAlign, VerticalAlign};
use nalgebra::Point2;
use std::collections::{HashMap, HashSet};

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

use crate::PlotArea;
use vizuara_core::{Color, Primitive, HorizontalAlign, VerticalAlign};
use nalgebra::Point2;
use std::collections::{HashMap, HashSet};

/// 桑基图节点
#[derive(Debug, Clone)]
pub struct SankeyNode {
    pub id: String,
    pub label: String,
    pub color: Color,
    pub x: Option<f32>,  // 可选的固定x位置
    pub y: Option<f32>,  // 可选的固定y位置
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

/// 内部布局用的链接结构
#[derive(Debug, Clone)]
struct LayoutLink {
    source_id: String,
    target_id: String,
    #[allow(dead_code)]
    value: f32,
    color: Color,
    #[allow(dead_code)]
    label: Option<String>,
    #[allow(dead_code)]
    source_y: f32,
    #[allow(dead_code)]
    target_y: f32,
    #[allow(dead_code)]
    thickness: f32,
}

/// 内部布局用的节点结构
#[derive(Debug, Clone)]
struct LayoutNode {
    id: String,
    #[allow(dead_code)]
    x: f32,
    #[allow(dead_code)]
    y: f32,
    #[allow(dead_code)]
    width: f32,
    #[allow(dead_code)]
    height: f32,
    color: Color,
    #[allow(dead_code)]
    label: Option<String>,
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
            label_color: Color::rgb(0.1, 0.1, 0.1),
            default_node_color: Color::rgb(0.6, 0.6, 0.6),
            default_link_color: Color::rgba(0.4, 0.4, 0.4, 0.6),
        }
    }
}
    source_y: f32,
    target_y: f32,
    thickness: f32,
}

/// 桑基图
#[derive(Debug, Clone)]
pub struct SankeyDiagram {
    nodes: Vec<SankeyNode>,
    links: Vec<SankeyLink>,
    style: SankeyStyle,
    title: Option<String>,
    layout_nodes: Vec<LayoutNode>,
    layout_links: Vec<LayoutLink>,
}

impl SankeyDiagram {
    /// 创建新的桑基图
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            style: SankeyStyle::default(),
            title: None,
            layout_nodes: Vec::new(),
            layout_links: Vec::new(),
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

    /// 批量添加节点
    pub fn nodes(mut self, nodes: Vec<SankeyNode>) -> Self {
        self.nodes = nodes;
        self
    }

    /// 批量添加链接
    pub fn links(mut self, links: Vec<SankeyLink>) -> Self {
        self.links = links;
        self
    }

    /// 从简单数据创建
    pub fn from_data(mut self, data: &[(&str, &str, f32)]) -> Self {
        let mut node_set = HashSet::new();
        
        // 收集所有节点
        for &(source, target, _) in data {
            node_set.insert(source);
            node_set.insert(target);
        }

        // 创建节点
        self.nodes = node_set.into_iter().enumerate().map(|(i, id)| {
            let hue = (i as f32 / 6.0) % 1.0;
            let color = Color::rgb(
                (hue * 6.0).sin().abs(),
                ((hue * 6.0) + 2.0).sin().abs(),
                ((hue * 6.0) + 4.0).sin().abs(),
            );
            
            SankeyNode {
                id: id.to_string(),
                label: id.to_string(),
                color,
                x: None,
                y: None,
            }
        }).collect();

        // 创建链接
        self.links = data.iter().map(|&(source, target, value)| {
            SankeyLink {
                source: source.to_string(),
                target: target.to_string(),
                value,
                color: None,
                label: None,
            }
        }).collect();

        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
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

    /// 设置是否显示标签
    pub fn show_labels(mut self, nodes: bool, links: bool) -> Self {
        self.style.show_node_labels = nodes;
        self.style.show_link_labels = links;
        self
    }

    /// 计算布局
    fn compute_layout(&mut self, plot_area: PlotArea) {
        if self.nodes.is_empty() || self.links.is_empty() {
            return;
        }

        // 计算节点层级
        let levels = self.compute_node_levels();
        let max_level = levels.values().max().copied().unwrap_or(0);

        // 计算每个节点的总流量
        let mut node_values: HashMap<String, (f32, f32)> = HashMap::new(); // (incoming, outgoing)
        
        for node in &self.nodes {
            node_values.insert(node.id.clone(), (0.0, 0.0));
        }

        for link in &self.links {
            if let Some((_, outgoing)) = node_values.get_mut(&link.source) {
                *outgoing += link.value;
            }
            if let Some((incoming, _)) = node_values.get_mut(&link.target) {
                *incoming += link.value;
            }
        }

        // 布局节点
        self.layout_nodes.clear();
        let available_width = plot_area.width - self.style.node_width * (max_level + 1) as f32;
        let level_spacing = if max_level > 0 { available_width / max_level as f32 } else { 0.0 };

        for node in &self.nodes {
            let level = levels.get(&node.id).copied().unwrap_or(0);
            let (incoming, outgoing) = node_values.get(&node.id).copied().unwrap_or((0.0, 0.0));
            let total_value = incoming.max(outgoing);
            
            let x = plot_area.x + level as f32 * (level_spacing + self.style.node_width);
            let height = (total_value / self.get_max_flow()) * plot_area.height * 0.8;
            let y = plot_area.y + (plot_area.height - height) / 2.0; // 简化定位

            self.layout_nodes.push(LayoutNode {
                id: node.id.clone(),
                label: node.label.clone(),
                color: node.color,
                x,
                y,
                height,
                incoming_value: incoming,
                outgoing_value: outgoing,
            });
        }

        // 布局链接
        self.layout_links.clear();
        for link in &self.links {
            let source_node = self.layout_nodes.iter().find(|n| n.id == link.source);
            let target_node = self.layout_nodes.iter().find(|n| n.id == link.target);

            if let (Some(source), Some(target)) = (source_node, target_node) {
                let thickness = (link.value / self.get_max_flow()) * plot_area.height * 0.1;
                let color = link.color.unwrap_or(self.style.default_link_color);

                self.layout_links.push(LayoutLink {
                    source: link.source.clone(),
                    target: link.target.clone(),
                    value: link.value,
                    color,
                    label: link.label.clone(),
                    source_y: source.y + source.height / 2.0,
                    target_y: target.y + target.height / 2.0,
                    thickness,
                });
            }
        }
    }

    /// 计算节点层级
    fn compute_node_levels(&self) -> HashMap<String, usize> {
        let mut levels = HashMap::new();
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

        // 初始化
        for node in &self.nodes {
            levels.insert(node.id.clone(), 0);
            dependencies.insert(node.id.clone(), Vec::new());
        }

        // 构建依赖关系
        for link in &self.links {
            if let Some(deps) = dependencies.get_mut(&link.target) {
                deps.push(link.source.clone());
            }
        }

        // 简单的拓扑排序来分配层级
        let mut changed = true;
        while changed {
            changed = false;
            for (node, deps) in &dependencies {
                let max_dep_level = deps.iter()
                    .filter_map(|dep| levels.get(dep))
                    .max()
                    .copied()
                    .unwrap_or(0);
                
                let new_level = if deps.is_empty() { 0 } else { max_dep_level + 1 };
                
                if levels.get(node).copied().unwrap_or(0) < new_level {
                    levels.insert(node.clone(), new_level);
                    changed = true;
                }
            }
        }

        levels
    }

    /// 获取最大流量值
    fn get_max_flow(&self) -> f32 {
        self.links.iter().map(|l| l.value).fold(0.0, f32::max)
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.nodes.is_empty() {
            return primitives;
        }

        // 计算布局
        let mut diagram = self.clone();
        diagram.compute_layout(plot_area);

        // 绘制链接（贝塞尔曲线）
        for link in &diagram.layout_links {
            let source_node = diagram.layout_nodes.iter().find(|n| n.id == link.source);
            let target_node = diagram.layout_nodes.iter().find(|n| n.id == link.target);

            if let (Some(source), Some(target)) = (source_node, target_node) {
                // 创建贝塞尔曲线路径
                let control_point_offset = (target.x - source.x) * 0.5;
                
                // 简化为直线连接（实际应该用贝塞尔曲线）
                let points = vec![
                    Point2::new(source.x + self.style.node_width, link.source_y),
                    Point2::new(source.x + self.style.node_width + control_point_offset, link.source_y),
                    Point2::new(target.x - control_point_offset, link.target_y),
                    Point2::new(target.x, link.target_y),
                ];

                // 使用多边形近似贝塞尔曲线
                let mut flow_points = Vec::new();
                let thickness_half = link.thickness / 2.0;
                
                for point in &points {
                    flow_points.push(Point2::new(point.x, point.y - thickness_half));
                }
                for point in points.iter().rev() {
                    flow_points.push(Point2::new(point.x, point.y + thickness_half));
                }

                primitives.push(Primitive::Polygon {
                    points: flow_points,
                    fill: Color::rgba(link.color.r, link.color.g, link.color.b, self.style.link_opacity),
                    stroke: None,
                });
            }
        }

        // 绘制节点
        for node in &diagram.layout_nodes {
            primitives.push(Primitive::Rectangle {
                min: Point2::new(node.x, node.y),
                max: Point2::new(node.x + self.style.node_width, node.y + node.height),
            });

            // 绘制节点标签
            if self.style.show_node_labels {
                primitives.push(Primitive::Text {
                    position: Point2::new(
                        node.x + self.style.node_width / 2.0,
                        node.y - 5.0,
                    ),
                    content: node.label.clone(),
                    size: self.style.label_size,
                    color: self.style.label_color,
                    h_align: HorizontalAlign::Center,
                    v_align: VerticalAlign::Bottom,
                });
            }
        }

        // 绘制标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(
                    plot_area.x + plot_area.width / 2.0,
                    plot_area.y - 20.0,
                ),
                content: title.clone(),
                size: 16.0,
                color: Color::rgb(0.1, 0.1, 0.1),
                h_align: HorizontalAlign::Center,
                v_align: VerticalAlign::Bottom,
            });
        }

        primitives
    }

    /// 获取节点数量
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 获取链接数量
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// 获取总流量
    pub fn total_flow(&self) -> f32 {
        self.links.iter().map(|l| l.value).sum()
    }
}

impl Default for SankeyDiagram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sankey_creation() {
        let diagram = SankeyDiagram::new();
        assert_eq!(diagram.node_count(), 0);
        assert_eq!(diagram.link_count(), 0);
    }

    #[test]
    fn test_from_data() {
        let data = [
            ("A", "X", 10.0),
            ("B", "X", 20.0),
            ("B", "Y", 15.0),
        ];
        
        let diagram = SankeyDiagram::new().from_data(&data);
        assert_eq!(diagram.node_count(), 4); // A, B, X, Y
        assert_eq!(diagram.link_count(), 3);
        assert_eq!(diagram.total_flow(), 45.0);
    }

    #[test]
    fn test_sankey_primitives() {
        let data = [
            ("源头A", "中转", 100.0),
            ("源头B", "中转", 80.0),
            ("中转", "目标", 180.0),
        ];
        
        let diagram = SankeyDiagram::new()
            .from_data(&data)
            .title("测试桑基图");
        
        let plot_area = PlotArea::new(0.0, 0.0, 600.0, 400.0);
        let primitives = diagram.generate_primitives(plot_area);
        assert!(!primitives.is_empty());
    }

    #[test]
    fn test_node_levels() {
        let data = [
            ("A", "C", 10.0),
            ("B", "C", 20.0),
            ("C", "D", 30.0),
        ];
        
        let diagram = SankeyDiagram::new().from_data(&data);
        let levels = diagram.compute_node_levels();
        
        assert_eq!(levels.get("A"), Some(&0));
        assert_eq!(levels.get("B"), Some(&0));
        assert_eq!(levels.get("C"), Some(&1));
        assert_eq!(levels.get("D"), Some(&2));
    }
}
