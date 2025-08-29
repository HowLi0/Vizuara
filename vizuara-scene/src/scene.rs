use nalgebra::Point2;
use vizuara_components::{Axis, AxisDirection};
use vizuara_core::{LinearScale, Primitive};
use vizuara_plots::{
    AreaChart, BarPlot, BoxPlot, ContourPlot, DensityPlot, Heatmap, Histogram, LinePlot,
    ParallelCoordinates, PieChart, PlotArea, RadarChart, SankeyDiagram, ScatterPlot, Treemap,
    ViolinPlot,
};

/// 场景：包含坐标轴和多个图表的绘图区域
pub struct Scene {
    plot_area: PlotArea,
    x_axis: Option<Axis>,
    y_axis: Option<Axis>,
    plots: Vec<Box<dyn PlotRenderer>>,
    title: Option<String>,
}

/// 图表渲染器 trait
pub trait PlotRenderer {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive>;
}

// 为 ScatterPlot 实现 PlotRenderer
impl PlotRenderer for ScatterPlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 LinePlot 实现 PlotRenderer
impl PlotRenderer for LinePlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 BarPlot 实现 PlotRenderer
impl PlotRenderer for BarPlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 Histogram 实现 PlotRenderer
impl PlotRenderer for Histogram {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 Heatmap 实现 PlotRenderer
impl PlotRenderer for Heatmap {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 BoxPlot 实现 PlotRenderer
impl PlotRenderer for BoxPlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 AreaChart 实现 PlotRenderer
impl PlotRenderer for AreaChart {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 ViolinPlot 实现 PlotRenderer
impl PlotRenderer for ViolinPlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 PieChart 实现 PlotRenderer
impl PlotRenderer for PieChart {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 RadarChart 实现 PlotRenderer
impl PlotRenderer for RadarChart {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 DensityPlot 实现 PlotRenderer
impl PlotRenderer for DensityPlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        self.generate_primitives(plot_area)
    }
}

// 为 ContourPlot 实现 PlotRenderer
impl PlotRenderer for ContourPlot {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        // 调用 ContourPlot 自己的方法
        ContourPlot::generate_primitives(self, plot_area)
    }
}

// 为 SankeyDiagram 实现 PlotRenderer
impl PlotRenderer for SankeyDiagram {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        // 调用 SankeyDiagram 自己的方法
        SankeyDiagram::generate_primitives(self, plot_area)
    }
}

// 为 Treemap 实现 PlotRenderer
impl PlotRenderer for Treemap {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        // 调用 Treemap 自己的方法
        Treemap::generate_primitives(self, plot_area)
    }
}

// 为 ParallelCoordinates 实现 PlotRenderer
impl PlotRenderer for ParallelCoordinates {
    fn generate_primitives(&self, plot_area: PlotArea) -> Vec<Primitive> {
        // 调用 ParallelCoordinates 自己的方法
        ParallelCoordinates::generate_primitives(self, plot_area)
    }
}

impl Scene {
    /// 创建新的场景
    pub fn new(plot_area: PlotArea) -> Self {
        Self {
            plot_area,
            x_axis: None,
            y_axis: None,
            plots: Vec::new(),
            title: None,
        }
    }

    /// 添加 X 轴
    pub fn add_x_axis(mut self, scale: LinearScale, title: Option<String>) -> Self {
        let axis_y = self.plot_area.y + self.plot_area.height + 20.0; // 轴在绘图区域下方
        let mut axis = Axis::new(
            AxisDirection::Horizontal,
            scale,
            (self.plot_area.x, axis_y),
            self.plot_area.width,
        );

        if let Some(title) = title {
            axis = axis.title(title);
        }

        self.x_axis = Some(axis);
        self
    }

    /// 添加 Y 轴
    pub fn add_y_axis(mut self, scale: LinearScale, title: Option<String>) -> Self {
        let axis_x = self.plot_area.x - 20.0; // 轴在绘图区域左侧
        let mut axis = Axis::new(
            AxisDirection::Vertical,
            scale,
            (axis_x, self.plot_area.y),
            self.plot_area.height,
        );

        if let Some(title) = title {
            axis = axis.title(title);
        }

        self.y_axis = Some(axis);
        self
    }

    /// 添加散点图
    pub fn add_scatter_plot(mut self, plot: ScatterPlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加折线图
    pub fn add_line_plot(mut self, plot: LinePlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加柱状图
    pub fn add_bar_plot(mut self, plot: BarPlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加直方图
    pub fn add_histogram(mut self, plot: Histogram) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加热力图
    pub fn add_heatmap(mut self, plot: Heatmap) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加箱线图
    pub fn add_boxplot(mut self, plot: BoxPlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加面积图
    pub fn add_area_chart(mut self, plot: AreaChart) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加小提琴图
    pub fn add_violin_plot(mut self, plot: ViolinPlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加饼图
    pub fn add_pie_chart(mut self, plot: PieChart) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加雷达图
    pub fn add_radar_chart(mut self, plot: RadarChart) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加密度图
    pub fn add_density_plot(mut self, plot: DensityPlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加等高线图
    pub fn add_contour_plot(mut self, plot: ContourPlot) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加桑基图
    pub fn add_sankey_diagram(mut self, plot: SankeyDiagram) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加树状图
    pub fn add_treemap(mut self, plot: Treemap) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 添加平行坐标图
    pub fn add_parallel_coordinates(mut self, plot: ParallelCoordinates) -> Self {
        self.plots.push(Box::new(plot));
        self
    }

    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 生成所有渲染图元
    pub fn generate_primitives(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        // 1. 绘制标题
        if let Some(ref title) = self.title {
            primitives.push(Primitive::Text {
                position: Point2::new(
                    self.plot_area.x + self.plot_area.width / 2.0,
                    self.plot_area.y - 40.0,
                ),
                content: title.clone(),
                size: 16.0,
                color: vizuara_core::Color::rgb(0.1, 0.1, 0.1),
                h_align: vizuara_core::HorizontalAlign::Center,
                v_align: vizuara_core::VerticalAlign::Bottom,
            });
        }

        // 2. 绘制坐标轴
        if let Some(ref x_axis) = self.x_axis {
            primitives.extend(x_axis.generate_primitives());
        }

        if let Some(ref y_axis) = self.y_axis {
            primitives.extend(y_axis.generate_primitives());
        }

        // 3. 绘制绘图区域边框
        primitives.push(Primitive::Rectangle {
            min: Point2::new(self.plot_area.x, self.plot_area.y),
            max: Point2::new(
                self.plot_area.x + self.plot_area.width,
                self.plot_area.y + self.plot_area.height,
            ),
        });

        // 4. 绘制所有图表
        for plot in &self.plots {
            primitives.extend(plot.generate_primitives(self.plot_area));
        }

        primitives
    }

    /// 获取绘图区域
    pub fn plot_area(&self) -> PlotArea {
        self.plot_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizuara_core::Color;

    #[test]
    fn test_scene_creation() {
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let scene = Scene::new(plot_area);

        assert_eq!(scene.plot_area.x, 100.0);
        assert_eq!(scene.plot_area.width, 400.0);
    }

    #[test]
    fn test_scene_with_axes() {
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let x_scale = LinearScale::new(0.0, 10.0);
        let y_scale = LinearScale::new(0.0, 20.0);

        let scene = Scene::new(plot_area)
            .add_x_axis(x_scale, Some("X Axis".to_string()))
            .add_y_axis(y_scale, Some("Y Axis".to_string()))
            .title("Test Chart");

        let primitives = scene.generate_primitives();
        assert!(!primitives.is_empty());
    }

    #[test]
    fn test_scene_with_scatter_plot() {
        let plot_area = PlotArea::new(100.0, 100.0, 400.0, 300.0);
        let data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 1.0)];
        let scatter = ScatterPlot::new()
            .data(&data)
            .color(Color::rgb(1.0, 0.0, 0.0))
            .auto_scale();

        let scene = Scene::new(plot_area).add_scatter_plot(scatter);

        let primitives = scene.generate_primitives();
        assert!(!primitives.is_empty());
    }
}
