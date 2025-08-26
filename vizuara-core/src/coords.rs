use nalgebra::{Point2, Point3, Matrix3, Matrix4};
use serde::{Deserialize, Serialize};

/// 坐标系统抽象
pub trait CoordinateSystem {
    /// 将数据坐标转换为屏幕坐标
    fn data_to_screen(&self, data_point: Point2<f32>) -> Point2<f32>;
    
    /// 将屏幕坐标转换为数据坐标
    fn screen_to_data(&self, screen_point: Point2<f32>) -> Point2<f32>;
    
    /// 获取坐标系的变换矩阵
    fn transform_matrix(&self) -> Matrix3<f32>;
}

/// 2D 笛卡尔坐标系
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartesianCoords {
    /// 数据范围 (x_min, y_min, x_max, y_max)
    pub data_bounds: (f32, f32, f32, f32),
    /// 屏幕范围 (x_min, y_min, x_max, y_max)
    pub screen_bounds: (f32, f32, f32, f32),
    /// 是否翻转 Y 轴（屏幕坐标系通常 Y 轴向下）
    pub flip_y: bool,
}

impl CartesianCoords {
    /// 创建新的笛卡尔坐标系
    pub fn new(
        data_bounds: (f32, f32, f32, f32),
        screen_bounds: (f32, f32, f32, f32),
    ) -> Self {
        Self {
            data_bounds,
            screen_bounds,
            flip_y: true,
        }
    }
    
    /// 计算 X 轴的缩放比例
    pub fn x_scale(&self) -> f32 {
        let data_width = self.data_bounds.2 - self.data_bounds.0;
        let screen_width = self.screen_bounds.2 - self.screen_bounds.0;
        screen_width / data_width
    }
    
    /// 计算 Y 轴的缩放比例
    pub fn y_scale(&self) -> f32 {
        let data_height = self.data_bounds.3 - self.data_bounds.1;
        let screen_height = self.screen_bounds.3 - self.screen_bounds.1;
        screen_height / data_height
    }
}

impl CoordinateSystem for CartesianCoords {
    fn data_to_screen(&self, data_point: Point2<f32>) -> Point2<f32> {
        let x = (data_point.x - self.data_bounds.0) * self.x_scale() + self.screen_bounds.0;
        let y = if self.flip_y {
            self.screen_bounds.3 - (data_point.y - self.data_bounds.1) * self.y_scale()
        } else {
            (data_point.y - self.data_bounds.1) * self.y_scale() + self.screen_bounds.1
        };
        Point2::new(x, y)
    }
    
    fn screen_to_data(&self, screen_point: Point2<f32>) -> Point2<f32> {
        let x = (screen_point.x - self.screen_bounds.0) / self.x_scale() + self.data_bounds.0;
        let y = if self.flip_y {
            self.data_bounds.3 - (screen_point.y - self.screen_bounds.1) / self.y_scale()
        } else {
            (screen_point.y - self.screen_bounds.1) / self.y_scale() + self.data_bounds.1
        };
        Point2::new(x, y)
    }
    
    fn transform_matrix(&self) -> Matrix3<f32> {
        let sx = self.x_scale();
        let sy = if self.flip_y { -self.y_scale() } else { self.y_scale() };
        let tx = self.screen_bounds.0 - self.data_bounds.0 * sx;
        let ty = if self.flip_y {
            self.screen_bounds.3 + self.data_bounds.1 * sy
        } else {
            self.screen_bounds.1 - self.data_bounds.1 * sy
        };
        
        Matrix3::new(
            sx, 0.0, tx,
            0.0, sy, ty,
            0.0, 0.0, 1.0,
        )
    }
}

/// 3D 坐标系统
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cartesian3DCoords {
    /// 数据边界框 (x_min, y_min, z_min, x_max, y_max, z_max)
    pub data_bounds: (f32, f32, f32, f32, f32, f32),
    /// 视图矩阵
    pub view_matrix: Matrix4<f32>,
    /// 投影矩阵
    pub projection_matrix: Matrix4<f32>,
}

impl Cartesian3DCoords {
    /// 创建新的 3D 坐标系
    pub fn new(
        data_bounds: (f32, f32, f32, f32, f32, f32),
        view_matrix: Matrix4<f32>,
        projection_matrix: Matrix4<f32>,
    ) -> Self {
        Self {
            data_bounds,
            view_matrix,
            projection_matrix,
        }
    }
    
    /// 将 3D 点投影到 2D 屏幕坐标
    pub fn project_3d_to_2d(&self, point: Point3<f32>) -> Point2<f32> {
        let homogeneous = self.projection_matrix * self.view_matrix * point.to_homogeneous();
        let ndc = homogeneous.xyz() / homogeneous.w;
        Point2::new(ndc.x, ndc.y)
    }
}
