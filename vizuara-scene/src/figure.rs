use crate::Scene;
use vizuara_core::{Primitive, Result};

/// 图形对象：整个可视化的顶层容器
pub struct Figure {
    scenes: Vec<Scene>,
    width: f32,
    height: f32,
    title: Option<String>,
}

impl Figure {
    /// 创建新的图形对象
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            scenes: Vec::new(),
            width,
            height,
            title: None,
        }
    }

    // 删除与 Default trait 混淆的同名方法，改为实现标准 Default

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 添加场景
    pub fn add_scene(mut self, scene: Scene) -> Self {
        self.scenes.push(scene);
        self
    }

    /// 生成所有渲染图元
    pub fn generate_primitives(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        // 添加整体标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: nalgebra::Point2::new(self.width / 2.0, 30.0),
                content: title.clone(),
                size: 20.0,
                color: vizuara_core::Color::rgb(0.1, 0.1, 0.1),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Bottom,
            });
        }

        // 添加所有场景的图元
        for scene in &self.scenes {
            primitives.extend(scene.generate_primitives());
        }

        primitives
    }

    /// 在窗口中渲染
    pub async fn show(self) -> Result<()> {
        // 暂时返回成功，真实的窗口渲染需要在应用层实现
        // 避免循环依赖
        println!("📊 Figure 准备就绪，包含 {} 个场景", self.scenes.len());
        println!("💡 请使用 vizuara_window::show_figure(figure) 来显示");
        Ok(())
    }

    /// 获取尺寸
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// 获取场景数量
    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self::new(800.0, 600.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizuara_core::{Color, LinearScale};
    use vizuara_plots::{PlotArea, ScatterPlot};

    #[test]
    fn test_figure_creation() {
        let figure = Figure::new(800.0, 600.0);
        assert_eq!(figure.size(), (800.0, 600.0));
        assert_eq!(figure.scene_count(), 0);
    }

    #[test]
    fn test_figure_with_scene() {
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let scene = Scene::new(plot_area);

        let figure = Figure::new(800.0, 600.0)
            .title("Test Figure")
            .add_scene(scene);

        assert_eq!(figure.scene_count(), 1);

        let primitives = figure.generate_primitives();
        assert!(!primitives.is_empty());
    }

    #[test]
    fn test_complete_example() {
        // 创建测试数据
        let data = vec![(1.0, 2.0), (2.0, 3.5), (3.0, 1.8), (4.0, 4.2)];

        // 创建散点图
        let scatter = ScatterPlot::new()
            .data(&data)
            .color(Color::rgb(0.8, 0.2, 0.2))
            .size(8.0)
            .auto_scale();

        // 创建场景
        let plot_area = PlotArea::new(100.0, 100.0, 600.0, 400.0);
        let x_scale = LinearScale::new(0.0, 5.0);
        let y_scale = LinearScale::new(0.0, 5.0);

        let scene = Scene::new(plot_area)
            .add_x_axis(x_scale, Some("X Value".to_string()))
            .add_y_axis(y_scale, Some("Y Value".to_string()))
            .add_scatter_plot(scatter)
            .title("Scatter Plot Example");

        // 创建图形
        let figure = Figure::new(800.0, 600.0)
            .title("Example Visualization")
            .add_scene(scene);

        // 验证能够生成图元
        let primitives = figure.generate_primitives();
        assert!(!primitives.is_empty());

        println!(
            "Generated {} primitives for complete example",
            primitives.len()
        );
    }
}
