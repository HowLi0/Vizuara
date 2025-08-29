//! 3D坐标轴系统
//! 
//! 为科学计算的3D数据可视化提供坐标轴显示功能

use nalgebra::{Point3, Vector3};
use vizuara_core::{Color, LinearScale, Scale};

/// 3D坐标轴方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis3DDirection {
    X,
    Y,
    Z,
}

/// 3D坐标轴网格类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridType {
    /// 无网格
    None,
    /// 主网格线
    Major,
    /// 主网格线和次网格线
    MajorMinor,
}

/// 3D坐标轴样式
#[derive(Debug, Clone)]
pub struct Axis3DStyle {
    /// 轴线颜色
    pub axis_color: Color,
    /// 轴线宽度
    pub axis_width: f32,
    /// 刻度线颜色
    pub tick_color: Color,
    /// 刻度线长度
    pub tick_length: f32,
    /// 网格线颜色
    pub grid_color: Color,
    /// 网格线宽度
    pub grid_width: f32,
    /// 次网格线颜色
    pub minor_grid_color: Color,
    /// 次网格线宽度
    pub minor_grid_width: f32,
    /// 标签颜色
    pub label_color: Color,
    /// 标签字体大小
    pub label_size: f32,
    /// 标题字体大小
    pub title_size: f32,
}

impl Default for Axis3DStyle {
    fn default() -> Self {
        Self {
            axis_color: Color::rgb(0.8, 0.8, 0.8),      // 更亮的轴线
            axis_width: 3.0,                             // 更粗的轴线
            tick_color: Color::rgb(0.9, 0.9, 0.9),      // 更亮的刻度线
            tick_length: 0.15,                          // 更长的刻度线
            grid_color: Color::rgba(0.6, 0.6, 0.6, 0.7), // 更明显的网格
            grid_width: 1.5,                             // 更粗的网格线
            minor_grid_color: Color::rgba(0.7, 0.7, 0.7, 0.4),
            minor_grid_width: 0.8,                       // 稍粗的次网格线
            label_color: Color::rgb(1.0, 1.0, 1.0),     // 白色标签（更清晰）
            label_size: 14.0,                           // 更大的标签
            title_size: 16.0,                           // 更大的标题
        }
    }
}

/// 3D坐标轴
#[derive(Debug, Clone)]
pub struct Axis3D {
    /// 轴方向
    direction: Axis3DDirection,
    /// 数值范围
    scale: LinearScale,
    /// 轴的起点
    origin: Point3<f32>,
    /// 轴的长度
    length: f32,
    /// 轴标题
    title: Option<String>,
    /// 主刻度数量
    major_tick_count: usize,
    /// 次刻度数量（在每个主刻度间）
    minor_tick_count: usize,
    /// 网格类型
    grid_type: GridType,
    /// 样式
    style: Axis3DStyle,
    /// 是否显示刻度标签
    show_labels: bool,
    /// 是否显示轴线
    show_axis: bool,
}

impl Axis3D {
    /// 创建新的3D坐标轴
    pub fn new(
        direction: Axis3DDirection,
        scale: LinearScale,
        origin: Point3<f32>,
        length: f32,
    ) -> Self {
        Self {
            direction,
            scale,
            origin,
            length,
            title: None,
            major_tick_count: 5,
            minor_tick_count: 4,
            grid_type: GridType::Major,
            style: Axis3DStyle::default(),
            show_labels: true,
            show_axis: true,
        }
    }

    /// 设置轴标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置主刻度数量
    pub fn major_ticks(mut self, count: usize) -> Self {
        self.major_tick_count = count;
        self
    }

    /// 设置次刻度数量
    pub fn minor_ticks(mut self, count: usize) -> Self {
        self.minor_tick_count = count;
        self
    }

    /// 设置网格类型
    pub fn grid(mut self, grid_type: GridType) -> Self {
        self.grid_type = grid_type;
        self
    }

    /// 设置样式
    pub fn style(mut self, style: Axis3DStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置是否显示标签
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// 设置是否显示轴线
    pub fn show_axis(mut self, show: bool) -> Self {
        self.show_axis = show;
        self
    }

    /// 获取轴的方向向量
    pub fn direction_vector(&self) -> Vector3<f32> {
        match self.direction {
            Axis3DDirection::X => Vector3::new(1.0, 0.0, 0.0),
            Axis3DDirection::Y => Vector3::new(0.0, 1.0, 0.0),
            Axis3DDirection::Z => Vector3::new(0.0, 0.0, 1.0),
        }
    }

    /// 获取轴的终点
    pub fn end_point(&self) -> Point3<f32> {
        self.origin + self.direction_vector() * self.length
    }

    /// 将数值转换为轴上的位置
    pub fn value_to_position(&self, value: f32) -> f32 {
        let normalized = self.scale.normalize(value);
        normalized * self.length
    }

    /// 获取轴上某个位置的3D点
    pub fn position_to_point(&self, position: f32) -> Point3<f32> {
        self.origin + self.direction_vector() * position
    }

    /// 获取主刻度位置
    pub fn major_tick_positions(&self) -> Vec<f32> {
        let ticks = self.scale.ticks(self.major_tick_count);
        ticks.into_iter().map(|v| self.value_to_position(v)).collect()
    }

    /// 获取次刻度位置
    pub fn minor_tick_positions(&self) -> Vec<f32> {
        if self.minor_tick_count == 0 {
            return Vec::new();
        }

        let major_positions = self.major_tick_positions();
        let mut minor_positions = Vec::new();

        for i in 0..major_positions.len() - 1 {
            let start = major_positions[i];
            let end = major_positions[i + 1];
            let step = (end - start) / (self.minor_tick_count + 1) as f32;

            for j in 1..=self.minor_tick_count {
                minor_positions.push(start + step * j as f32);
            }
        }

        minor_positions
    }

    /// 获取刻度标签
    pub fn tick_labels(&self) -> Vec<(f32, String)> {
        if !self.show_labels {
            return Vec::new();
        }

        let ticks = self.scale.ticks(self.major_tick_count);
        ticks
            .into_iter()
            .map(|v| (self.value_to_position(v), format!("{:.1}", v)))
            .collect()
    }
}

/// 3D坐标系统
#[derive(Debug, Clone)]
pub struct CoordinateSystem3D {
    /// X轴
    pub x_axis: Axis3D,
    /// Y轴
    pub y_axis: Axis3D,
    /// Z轴
    pub z_axis: Axis3D,
    /// 坐标系原点
    pub origin: Point3<f32>,
    /// 是否显示原点标记
    pub show_origin: bool,
    /// 原点标记大小
    pub origin_size: f32,
    /// 原点标记颜色
    pub origin_color: Color,
    /// 是否显示坐标面（xy, xz, yz平面的网格）
    pub show_planes: bool,
    /// 坐标面透明度
    pub plane_alpha: f32,
    /// 是否显示坐标轴盒子
    pub show_box: bool,
    /// 是否显示刻度值
    pub show_tick_labels: bool,
    /// 是否显示轴标题
    pub show_axis_titles: bool,
}

impl CoordinateSystem3D {
    /// 创建新的3D坐标系统
    pub fn new(
        x_range: (f32, f32),
        y_range: (f32, f32),
        z_range: (f32, f32),
        origin: Point3<f32>,
        scale: f32,
    ) -> Self {
        let x_scale = LinearScale::new(x_range.0, x_range.1);
        let y_scale = LinearScale::new(y_range.0, y_range.1);
        let z_scale = LinearScale::new(z_range.0, z_range.1);

        let x_axis = Axis3D::new(
            Axis3DDirection::X,
            x_scale,
            origin,
            (x_range.1 - x_range.0) * scale,
        ).title("X");

        let y_axis = Axis3D::new(
            Axis3DDirection::Y,
            y_scale,
            origin,
            (y_range.1 - y_range.0) * scale,
        ).title("Y");

        let z_axis = Axis3D::new(
            Axis3DDirection::Z,
            z_scale,
            origin,
            (z_range.1 - z_range.0) * scale,
        ).title("Z");

        Self {
            x_axis,
            y_axis,
            z_axis,
            origin,
            show_origin: true,
            origin_size: 0.05,
            origin_color: Color::rgb(1.0, 0.0, 0.0),
            show_planes: true,
            plane_alpha: 0.1,
            show_box: true,
            show_tick_labels: true,
            show_axis_titles: true,
        }
    }

    /// 设置轴标题
    pub fn axis_titles(mut self, x: &str, y: &str, z: &str) -> Self {
        self.x_axis = self.x_axis.title(x);
        self.y_axis = self.y_axis.title(y);
        self.z_axis = self.z_axis.title(z);
        self
    }

    /// 设置网格类型
    pub fn grid(mut self, grid_type: GridType) -> Self {
        self.x_axis = self.x_axis.grid(grid_type);
        self.y_axis = self.y_axis.grid(grid_type);
        self.z_axis = self.z_axis.grid(grid_type);
        self
    }

    /// 设置是否显示原点
    pub fn show_origin(mut self, show: bool) -> Self {
        self.show_origin = show;
        self
    }

    /// 设置是否显示坐标面
    pub fn show_planes(mut self, show: bool) -> Self {
        self.show_planes = show;
        self
    }

    /// 设置坐标面透明度
    pub fn plane_alpha(mut self, alpha: f32) -> Self {
        self.plane_alpha = alpha.clamp(0.0, 1.0);
        self
    }

    /// 设置是否显示坐标轴盒子
    pub fn show_box(mut self, show: bool) -> Self {
        self.show_box = show;
        self
    }

    /// 设置是否显示刻度标签
    pub fn show_tick_labels(mut self, show: bool) -> Self {
        self.show_tick_labels = show;
        self.x_axis = self.x_axis.show_labels(show);
        self.y_axis = self.y_axis.show_labels(show);
        self.z_axis = self.z_axis.show_labels(show);
        self
    }

    /// 设置是否显示轴标题
    pub fn show_axis_titles(mut self, show: bool) -> Self {
        self.show_axis_titles = show;
        self
    }

    /// 设置刻度数量
    pub fn tick_count(mut self, major: usize, minor: usize) -> Self {
        self.x_axis = self.x_axis.major_ticks(major).minor_ticks(minor);
        self.y_axis = self.y_axis.major_ticks(major).minor_ticks(minor);
        self.z_axis = self.z_axis.major_ticks(major).minor_ticks(minor);
        self
    }

    /// 将3D数据点转换为坐标系中的位置
    pub fn data_to_coords(&self, data_point: Point3<f32>) -> Point3<f32> {
        let x = self.x_axis.value_to_position(data_point.x);
        let y = self.y_axis.value_to_position(data_point.y);
        let z = self.z_axis.value_to_position(data_point.z);
        self.origin + Vector3::new(x, y, z)
    }

    /// 获取坐标系的边界框
    pub fn bounding_box(&self) -> (Point3<f32>, Point3<f32>) {
        let min = self.origin;
        let max = Point3::new(
            self.origin.x + self.x_axis.length,
            self.origin.y + self.y_axis.length,
            self.origin.z + self.z_axis.length,
        );
        (min, max)
    }
}

/// 3D坐标轴渲染数据
#[derive(Debug, Clone)]
pub struct Axis3DRenderData {
    /// 轴线顶点
    pub axis_lines: Vec<Point3<f32>>,
    /// 主刻度线顶点
    pub major_ticks: Vec<Point3<f32>>,
    /// 次刻度线顶点
    pub minor_ticks: Vec<Point3<f32>>,
    /// 网格线顶点
    pub grid_lines: Vec<Point3<f32>>,
    /// 次网格线顶点（当 GridType::MajorMinor 时生成）
    pub minor_grid_lines: Vec<Point3<f32>>,
    /// 坐标轴盒子边框线
    pub box_lines: Vec<Point3<f32>>,
    /// 坐标面顶点（三角形，用于填充坐标面）
    pub plane_triangles: Vec<Point3<f32>>,
    /// 坐标面颜色
    pub plane_colors: Vec<[f32; 4]>,
    /// 刻度标签（位置、文本和轴方向）
    pub tick_labels: Vec<(Point3<f32>, String, Axis3DDirection)>,
    /// 轴标题（位置、文本和轴方向）
    pub axis_titles: Vec<(Point3<f32>, String, Axis3DDirection)>,
    /// 原点标记
    pub origin_marker: Option<Point3<f32>>,
}

impl CoordinateSystem3D {
    /// 生成渲染数据
    pub fn generate_render_data(&self) -> Axis3DRenderData {
        let mut render_data = Axis3DRenderData {
            axis_lines: Vec::new(),
            major_ticks: Vec::new(),
            minor_ticks: Vec::new(),
            grid_lines: Vec::new(),
            minor_grid_lines: Vec::new(),
            box_lines: Vec::new(),
            plane_triangles: Vec::new(),
            plane_colors: Vec::new(),
            tick_labels: Vec::new(),
            axis_titles: Vec::new(),
            origin_marker: if self.show_origin { Some(self.origin) } else { None },
        };

        // 生成轴线
        if self.x_axis.show_axis {
            render_data.axis_lines.push(self.x_axis.origin);
            render_data.axis_lines.push(self.x_axis.end_point());
        }
        if self.y_axis.show_axis {
            render_data.axis_lines.push(self.y_axis.origin);
            render_data.axis_lines.push(self.y_axis.end_point());
        }
        if self.z_axis.show_axis {
            render_data.axis_lines.push(self.z_axis.origin);
            render_data.axis_lines.push(self.z_axis.end_point());
        }

        // 生成刻度和标签
        self.generate_axis_ticks_and_labels(&self.x_axis, &mut render_data);
        self.generate_axis_ticks_and_labels(&self.y_axis, &mut render_data);
        self.generate_axis_ticks_and_labels(&self.z_axis, &mut render_data);

        // 生成网格
        if self.x_axis.grid_type != GridType::None {
            self.generate_axis_grid(&self.x_axis, &mut render_data);
        }
        if self.y_axis.grid_type != GridType::None {
            self.generate_axis_grid(&self.y_axis, &mut render_data);
        }
        if self.z_axis.grid_type != GridType::None {
            self.generate_axis_grid(&self.z_axis, &mut render_data);
        }

        // 生成坐标面（类似MATLAB的坐标轴盒子）
        if self.show_planes {
            self.generate_coordinate_planes(&mut render_data);
        }

        // 生成坐标轴盒子
        if self.show_box {
            self.generate_coordinate_box(&mut render_data);
        }

        // 生成轴标题
        if self.show_axis_titles {
            self.generate_axis_titles(&mut render_data);
        }

        render_data
    }

    /// 生成单个轴的刻度和标签
    fn generate_axis_ticks_and_labels(&self, axis: &Axis3D, render_data: &mut Axis3DRenderData) {
        let perpendicular1 = match axis.direction {
            Axis3DDirection::X => Vector3::new(0.0, 1.0, 0.0),
            Axis3DDirection::Y => Vector3::new(1.0, 0.0, 0.0),
            Axis3DDirection::Z => Vector3::new(1.0, 0.0, 0.0),
        };

        let perpendicular2 = match axis.direction {
            Axis3DDirection::X => Vector3::new(0.0, 0.0, 1.0),
            Axis3DDirection::Y => Vector3::new(0.0, 0.0, 1.0),
            Axis3DDirection::Z => Vector3::new(0.0, 1.0, 0.0),
        };

        // 主刻度
        for position in axis.major_tick_positions() {
            let tick_point = axis.position_to_point(position);
            let tick_end1 = tick_point + perpendicular1 * axis.style.tick_length;
            let tick_end2 = tick_point + perpendicular2 * axis.style.tick_length;
            
            render_data.major_ticks.push(tick_point);
            render_data.major_ticks.push(tick_end1);
            render_data.major_ticks.push(tick_point);
            render_data.major_ticks.push(tick_end2);
        }

        // 次刻度
        for position in axis.minor_tick_positions() {
            let tick_point = axis.position_to_point(position);
            let tick_length = axis.style.tick_length * 0.5;
            let tick_end1 = tick_point + perpendicular1 * tick_length;
            let tick_end2 = tick_point + perpendicular2 * tick_length;
            
            render_data.minor_ticks.push(tick_point);
            render_data.minor_ticks.push(tick_end1);
            render_data.minor_ticks.push(tick_point);
            render_data.minor_ticks.push(tick_end2);
        }

        // 刻度标签
        if self.show_tick_labels {
            for (position, label) in axis.tick_labels() {
                let label_point = axis.position_to_point(position);
                let label_offset = (perpendicular1 + perpendicular2).normalize() * axis.style.tick_length * 2.5;
                render_data.tick_labels.push((label_point + label_offset, label, axis.direction));
            }
        }
    }

    /// 生成单个轴的网格
    fn generate_axis_grid(&self, axis: &Axis3D, render_data: &mut Axis3DRenderData) {
        let (other_axis1, other_axis2) = match axis.direction {
            Axis3DDirection::X => (&self.y_axis, &self.z_axis),
            Axis3DDirection::Y => (&self.x_axis, &self.z_axis),
            Axis3DDirection::Z => (&self.x_axis, &self.y_axis),
        };

        // 主网格线
        for position in axis.major_tick_positions() {
            let grid_point = axis.position_to_point(position);
            
            // 沿其他两个轴的方向绘制网格线
            let end1 = grid_point + other_axis1.direction_vector() * other_axis1.length;
            let end2 = grid_point + other_axis2.direction_vector() * other_axis2.length;
            
            render_data.grid_lines.push(grid_point);
            render_data.grid_lines.push(end1);
            render_data.grid_lines.push(grid_point);
            render_data.grid_lines.push(end2);
        }

        // 次网格线（仅当要求绘制主+次网格时）
        if axis.grid_type == GridType::MajorMinor {
            for position in axis.minor_tick_positions() {
                let grid_point = axis.position_to_point(position);

                let end1 = grid_point + other_axis1.direction_vector() * other_axis1.length;
                let end2 = grid_point + other_axis2.direction_vector() * other_axis2.length;

                render_data.minor_grid_lines.push(grid_point);
                render_data.minor_grid_lines.push(end1);
                render_data.minor_grid_lines.push(grid_point);
                render_data.minor_grid_lines.push(end2);
            }
        }
    }

    /// 生成坐标面（类似MATLAB的坐标轴盒子）
    fn generate_coordinate_planes(&self, render_data: &mut Axis3DRenderData) {
        let origin = self.origin;
        let x_end = origin + Vector3::new(self.x_axis.length, 0.0, 0.0);
        let y_end = origin + Vector3::new(0.0, self.y_axis.length, 0.0);
        let z_end = origin + Vector3::new(0.0, 0.0, self.z_axis.length);
        
        // XY平面（Z=0处的半透明面）
        let xy_plane_color = [0.8, 0.8, 0.9, self.plane_alpha];
        render_data.plane_triangles.extend_from_slice(&[
            origin,
            x_end,
            Point3::new(x_end.x, y_end.y, origin.z),
            
            origin,
            Point3::new(x_end.x, y_end.y, origin.z),
            y_end,
        ]);
        for _ in 0..6 {
            render_data.plane_colors.push(xy_plane_color);
        }

        // XZ平面（Y=0处的半透明面）
        let xz_plane_color = [0.9, 0.8, 0.8, self.plane_alpha];
        render_data.plane_triangles.extend_from_slice(&[
            origin,
            x_end,
            Point3::new(x_end.x, origin.y, z_end.z),
            
            origin,
            Point3::new(x_end.x, origin.y, z_end.z),
            z_end,
        ]);
        for _ in 0..6 {
            render_data.plane_colors.push(xz_plane_color);
        }

        // YZ平面（X=0处的半透明面）
        let yz_plane_color = [0.8, 0.9, 0.8, self.plane_alpha];
        render_data.plane_triangles.extend_from_slice(&[
            origin,
            y_end,
            Point3::new(origin.x, y_end.y, z_end.z),
            
            origin,
            Point3::new(origin.x, y_end.y, z_end.z),
            z_end,
        ]);
        for _ in 0..6 {
            render_data.plane_colors.push(yz_plane_color);
        }

        // 添加坐标轴盒子的边框线
        let box_corners = [
            origin, // 原点
            x_end,  // X轴端点
            y_end,  // Y轴端点
            z_end,  // Z轴端点
            Point3::new(x_end.x, y_end.y, origin.z), // XY平面对角
            Point3::new(x_end.x, origin.y, z_end.z), // XZ平面对角
            Point3::new(origin.x, y_end.y, z_end.z), // YZ平面对角
            Point3::new(x_end.x, y_end.y, z_end.z),  // 立方体远端角点
        ];

        // 添加盒子的12条边
        let box_edges = [
            // 底面4条边
            (0, 1), (0, 2), (1, 4), (2, 4),
            // 顶面4条边
            (3, 5), (3, 6), (5, 7), (6, 7),
            // 垂直4条边
            (0, 3), (1, 5), (2, 6), (4, 7),
        ];

        for &(start_idx, end_idx) in &box_edges {
            render_data.grid_lines.push(box_corners[start_idx]);
            render_data.grid_lines.push(box_corners[end_idx]);
        }
    }

    /// 生成坐标轴盒子（更清晰的边框）
    fn generate_coordinate_box(&self, render_data: &mut Axis3DRenderData) {
        let origin = self.origin;
        let x_end = origin + Vector3::new(self.x_axis.length, 0.0, 0.0);
        let y_end = origin + Vector3::new(0.0, self.y_axis.length, 0.0);
        let z_end = origin + Vector3::new(0.0, 0.0, self.z_axis.length);
        
        let box_corners = [
            origin, // 0: 原点
            x_end,  // 1: X轴端点
            y_end,  // 2: Y轴端点
            z_end,  // 3: Z轴端点
            Point3::new(x_end.x, y_end.y, origin.z), // 4: XY平面对角
            Point3::new(x_end.x, origin.y, z_end.z), // 5: XZ平面对角
            Point3::new(origin.x, y_end.y, z_end.z), // 6: YZ平面对角
            Point3::new(x_end.x, y_end.y, z_end.z),  // 7: 立方体远端角点
        ];

        // 盒子的12条边（更粗的线条）
        let box_edges = [
            // 底面4条边 (z = z_min)
            (0, 1), (1, 4), (4, 2), (2, 0),
            // 顶面4条边 (z = z_max)
            (3, 5), (5, 7), (7, 6), (6, 3),
            // 垂直4条边
            (0, 3), (1, 5), (2, 6), (4, 7),
        ];

        for &(start_idx, end_idx) in &box_edges {
            render_data.box_lines.push(box_corners[start_idx]);
            render_data.box_lines.push(box_corners[end_idx]);
        }
    }

    /// 生成轴标题
    fn generate_axis_titles(&self, render_data: &mut Axis3DRenderData) {
        // X轴标题
        if let Some(ref title) = self.x_axis.title {
            let title_point = self.x_axis.position_to_point(self.x_axis.length / 2.0);
            let title_offset = Vector3::new(0.0, -0.3, -0.3);
            render_data.axis_titles.push((title_point + title_offset, title.clone(), Axis3DDirection::X));
        }

        // Y轴标题
        if let Some(ref title) = self.y_axis.title {
            let title_point = self.y_axis.position_to_point(self.y_axis.length / 2.0);
            let title_offset = Vector3::new(-0.3, 0.0, -0.3);
            render_data.axis_titles.push((title_point + title_offset, title.clone(), Axis3DDirection::Y));
        }

        // Z轴标题
        if let Some(ref title) = self.z_axis.title {
            let title_point = self.z_axis.position_to_point(self.z_axis.length / 2.0);
            let title_offset = Vector3::new(-0.3, -0.3, 0.0);
            render_data.axis_titles.push((title_point + title_offset, title.clone(), Axis3DDirection::Z));
        }
    }
}
