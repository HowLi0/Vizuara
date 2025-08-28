//! 树状图实现
//!
//! 用于可视化层次数据，通过矩形面积表示数值大小

use crate::PlotArea;
use vizuara_core::{Color, Primitive, HorizontalAlign, VerticalAlign};
use nalgebra::Point2;

/// 树状图数据项
#[derive(Debug, Clone)]
pub struct TreemapItem {
    pub id: String,
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
    pub children: Vec<TreemapItem>,
    pub parent: Option<String>,
}

/// 树状图样式
#[derive(Debug, Clone)]
pub struct TreemapStyle {
    pub border_width: f32,
    pub border_color: Color,
    pub padding: f32,
    pub show_labels: bool,
    pub label_size: f32,
    pub label_color: Color,
    pub min_label_size: f32,
    pub color_scheme: ColorScheme,
}

/// 颜色方案
#[derive(Debug, Clone)]
pub enum ColorScheme {
    Category10,
    Blues,
    Reds,
    Greens,
    Custom(Vec<Color>),
}

impl Default for TreemapStyle {
    fn default() -> Self {
        Self {
            border_width: 1.0,
            border_color: Color::rgb(1.0, 1.0, 1.0),
            padding: 2.0,
            show_labels: true,
            label_size: 12.0,
            label_color: Color::rgb(0.1, 0.1, 0.1),
            min_label_size: 20.0,
            color_scheme: ColorScheme::Category10,
        }
    }
}

/// 布局后的矩形
#[derive(Debug, Clone)]
struct LayoutRect {
    item_id: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    #[allow(dead_code)]
    color: Color,
    #[allow(dead_code)]
    level: usize,
}

/// 树状图
#[derive(Debug, Clone)]
pub struct Treemap {
    root: Option<TreemapItem>,
    style: TreemapStyle,
    title: Option<String>,
    layout_rects: Vec<LayoutRect>,
}

impl Treemap {
    /// 创建新的树状图
    pub fn new() -> Self {
        Self {
            root: None,
            style: TreemapStyle::default(),
            title: None,
            layout_rects: Vec::new(),
        }
    }

    /// 设置根节点
    pub fn root(mut self, root: TreemapItem) -> Self {
        self.root = Some(root);
        self
    }

    /// 从扁平数据创建
    pub fn from_flat_data(mut self, data: &[(&str, f32, Option<&str>)]) -> Self {
        let mut items: Vec<TreemapItem> = data.iter().enumerate().map(|(i, &(label, value, parent))| {
            TreemapItem {
                id: format!("item_{}", i),
                label: label.to_string(),
                value,
                color: None,
                children: Vec::new(),
                parent: parent.map(|p| p.to_string()),
            }
        }).collect();

        // 构建树形结构
        let mut root_items = Vec::new();
        
        for i in 0..items.len() {
            if items[i].parent.is_none() {
                // 找到所有子项
                let mut children = Vec::new();
                for j in 0..items.len() {
                    if items[j].parent.as_ref() == Some(&items[i].label) {
                        children.push(items[j].clone());
                    }
                }
                items[i].children = children;
                root_items.push(items[i].clone());
            }
        }

        if root_items.len() == 1 {
            self.root = Some(root_items.into_iter().next().unwrap());
        } else if !root_items.is_empty() {
            // 创建虚拟根节点
            let total_value = root_items.iter().map(|item| item.value).sum();
            self.root = Some(TreemapItem {
                id: "root".to_string(),
                label: "Root".to_string(),
                value: total_value,
                color: None,
                children: root_items,
                parent: None,
            });
        }

        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置边框样式
    pub fn border(mut self, width: f32, color: Color) -> Self {
        self.style.border_width = width;
        self.style.border_color = color;
        self
    }

    /// 设置内边距
    pub fn padding(mut self, padding: f32) -> Self {
        self.style.padding = padding;
        self
    }

    /// 设置是否显示标签
    pub fn show_labels(mut self, show: bool, size: f32) -> Self {
        self.style.show_labels = show;
        self.style.label_size = size;
        self
    }

    /// 设置颜色方案
    pub fn color_scheme(mut self, scheme: ColorScheme) -> Self {
        self.style.color_scheme = scheme;
        self
    }

    /// 根据索引和层级获取颜色
    fn get_color(&self, index: usize, _level: usize) -> Color {
        match &self.style.color_scheme {
            ColorScheme::Category10 => {
                let colors = [
                    Color::rgb(0.12, 0.47, 0.71),
                    Color::rgb(1.0, 0.50, 0.05),
                    Color::rgb(0.17, 0.63, 0.17),
                    Color::rgb(0.84, 0.15, 0.16),
                    Color::rgb(0.58, 0.40, 0.74),
                    Color::rgb(0.55, 0.34, 0.29),
                    Color::rgb(0.89, 0.47, 0.76),
                    Color::rgb(0.50, 0.50, 0.50),
                    Color::rgb(0.74, 0.74, 0.13),
                    Color::rgb(0.09, 0.75, 0.81),
                ];
                colors[index % colors.len()]
            }
            ColorScheme::Blues => {
                let intensity = 0.3 + (index % 5) as f32 * 0.15;
                Color::rgb(intensity * 0.3, intensity * 0.6, intensity)
            }
            ColorScheme::Reds => {
                let intensity = 0.3 + (index % 5) as f32 * 0.15;
                Color::rgb(intensity, intensity * 0.3, intensity * 0.3)
            }
            ColorScheme::Greens => {
                let intensity = 0.3 + (index % 5) as f32 * 0.15;
                Color::rgb(intensity * 0.3, intensity, intensity * 0.3)
            }
            ColorScheme::Custom(colors) => {
                if colors.is_empty() {
                    Color::rgb(0.7, 0.7, 0.7)
                } else {
                    colors[index % colors.len()]
                }
            }
        }
    }

    /// 计算布局（使用Squarified算法的简化版本）
    fn compute_layout(&mut self, plot_area: PlotArea) {
        self.layout_rects.clear();
        
        if let Some(root) = self.root.clone() {
            let rect = Rectangle {
                x: plot_area.x + self.style.padding,
                y: plot_area.y + self.style.padding,
                width: plot_area.width - 2.0 * self.style.padding,
                height: plot_area.height - 2.0 * self.style.padding,
            };

            self.layout_item(&root, rect, 0, 0);
        }
    }

    /// 递归布局单个项目
    fn layout_item(&mut self, item: &TreemapItem, rect: Rectangle, level: usize, color_index: usize) {
        if item.children.is_empty() {
            // 叶子节点，创建矩形
            let color = item.color.unwrap_or_else(|| self.get_color(color_index, level));
            
            self.layout_rects.push(LayoutRect {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                item: item.clone(),
                color,
                level,
            });
        } else {
            // 非叶子节点，递归布局子项
            let total_value: f32 = item.children.iter().map(|child| child.value).sum();
            
            if total_value <= 0.0 {
                return;
            }

            let subrects = self.squarify(&item.children, rect, total_value);
            
            for (i, (child, subrect)) in item.children.iter().zip(subrects.iter()).enumerate() {
                self.layout_item(child, *subrect, level + 1, color_index + i);
            }
        }
    }

    /// Squarified算法的简化实现
    fn squarify(&self, items: &[TreemapItem], rect: Rectangle, total_value: f32) -> Vec<Rectangle> {
        let mut result = Vec::new();
        let mut remaining_rect = rect;
        let mut remaining_value = total_value;

        for item in items {
            if remaining_value <= 0.0 {
                break;
            }

            let ratio = item.value / remaining_value;
            let (sub_rect, new_remaining) = self.split_rect(remaining_rect, ratio);
            
            result.push(sub_rect);
            remaining_rect = new_remaining;
            remaining_value -= item.value;
        }

        result
    }

    /// 分割矩形
    fn split_rect(&self, rect: Rectangle, ratio: f32) -> (Rectangle, Rectangle) {
        let padding = self.style.padding;
        
        if rect.width > rect.height {
            // 水平分割
            let split_width = rect.width * ratio;
            let sub_rect = Rectangle {
                x: rect.x,
                y: rect.y,
                width: split_width - padding,
                height: rect.height,
            };
            let remaining = Rectangle {
                x: rect.x + split_width,
                y: rect.y,
                width: rect.width - split_width,
                height: rect.height,
            };
            (sub_rect, remaining)
        } else {
            // 垂直分割
            let split_height = rect.height * ratio;
            let sub_rect = Rectangle {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: split_height - padding,
            };
            let remaining = Rectangle {
                x: rect.x,
                y: rect.y + split_height,
                width: rect.width,
                height: rect.height - split_height,
            };
            (sub_rect, remaining)
        }
    }

    /// 生成渲染图元
    pub fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        if self.root.is_none() {
            return primitives;
        }

        // 计算布局
        let mut treemap = self.clone();
        treemap.compute_layout(plot_area);

        // 绘制矩形
        for layout_rect in &treemap.layout_rects {
            // 绘制填充矩形
            primitives.push(Primitive::Rectangle {
                min: Point2::new(layout_rect.x, layout_rect.y),
                max: Point2::new(
                    layout_rect.x + layout_rect.width,
                    layout_rect.y + layout_rect.height,
                ),
            });

            // 绘制边框
            if self.style.border_width > 0.0 {
                primitives.push(Primitive::Rectangle {
                    min: Point2::new(layout_rect.x, layout_rect.y),
                    max: Point2::new(
                        layout_rect.x + layout_rect.width,
                        layout_rect.y + layout_rect.height,
                    ),
                });
            }

            // 绘制标签
            if self.style.show_labels && 
               layout_rect.width > self.style.min_label_size && 
               layout_rect.height > self.style.min_label_size {
                
                let label_text = if layout_rect.item.value > 0.0 {
                    format!("{}\n{:.1}", layout_rect.item.label, layout_rect.item.value)
                } else {
                    layout_rect.item.label.clone()
                };

                primitives.push(Primitive::Text {
                    position: Point2::new(
                        layout_rect.x + layout_rect.width / 2.0,
                        layout_rect.y + layout_rect.height / 2.0,
                    ),
                    content: label_text,
                    size: self.style.label_size,
                    color: self.style.label_color,
                    h_align: HorizontalAlign::Center,
                    v_align: VerticalAlign::Middle,
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

    /// 获取总节点数
    pub fn node_count(&self) -> usize {
        self.count_nodes(self.root.as_ref())
    }

    /// 递归计算节点数
    fn count_nodes(&self, item: Option<&TreemapItem>) -> usize {
        match item {
            Some(item) => {
                1 + item.children.iter().map(|child| self.count_nodes(Some(child))).sum::<usize>()
            }
            None => 0,
        }
    }

    /// 获取总值
    pub fn total_value(&self) -> f32 {
        self.root.as_ref().map(|root| root.value).unwrap_or(0.0)
    }
}

impl Default for Treemap {
    fn default() -> Self {
        Self::new()
    }
}

/// 内部矩形结构
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treemap_creation() {
        let treemap = Treemap::new();
        assert_eq!(treemap.node_count(), 0);
        assert_eq!(treemap.total_value(), 0.0);
    }

    #[test]
    fn test_from_flat_data() {
        let data = [
            ("Root", 100.0, None),
            ("Child1", 60.0, Some("Root")),
            ("Child2", 40.0, Some("Root")),
        ];
        
        let treemap = Treemap::new().from_flat_data(&data);
        assert_eq!(treemap.node_count(), 3);
        assert_eq!(treemap.total_value(), 100.0);
    }

    #[test]
    fn test_treemap_item() {
        let root = TreemapItem {
            id: "root".to_string(),
            label: "根节点".to_string(),
            value: 100.0,
            color: Some(Color::rgb(0.5, 0.5, 0.5)),
            children: vec![
                TreemapItem {
                    id: "child1".to_string(),
                    label: "子节点1".to_string(),
                    value: 60.0,
                    color: None,
                    children: Vec::new(),
                    parent: Some("root".to_string()),
                },
                TreemapItem {
                    id: "child2".to_string(),
                    label: "子节点2".to_string(),
                    value: 40.0,
                    color: None,
                    children: Vec::new(),
                    parent: Some("root".to_string()),
                },
            ],
            parent: None,
        };

        let treemap = Treemap::new().root(root);
        assert_eq!(treemap.node_count(), 3);
        assert_eq!(treemap.total_value(), 100.0);
    }

    #[test]
    fn test_treemap_primitives() {
        let data = [
            ("总计", 100.0, None),
            ("类别A", 60.0, Some("总计")),
            ("类别B", 40.0, Some("总计")),
        ];
        
        let treemap = Treemap::new()
            .from_flat_data(&data)
            .title("测试树状图");
        
        let plot_area = PlotArea::new(0.0, 0.0, 400.0, 300.0);
        let primitives = treemap.generate_primitives(plot_area);
        assert!(!primitives.is_empty());
    }

    #[test]
    fn test_color_schemes() {
        let treemap = Treemap::new();
        
        // 测试不同颜色方案
        let color1 = treemap.get_color(0, 0);
        let color2 = treemap.get_color(1, 0);
        assert_ne!(color1.r, color2.r); // 应该是不同的颜色
    }
}
