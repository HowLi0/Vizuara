use nalgebra::{Matrix3, Vector2, Vector3};
use serde::{Deserialize, Serialize};
use vizuara_core::{
    coords::{LogicalPosition, WorldPosition},
    error::Result,
};

/// 表示2D视口的变换矩阵和坐标转换
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    /// 变换矩阵：从世界坐标到屏幕坐标
    transform: Matrix3<f64>,
    /// 逆变换矩阵：从屏幕坐标到世界坐标
    inverse_transform: Matrix3<f64>,
    /// 视口的像素尺寸
    size: Vector2<u32>,
    /// 世界坐标系的可见区域边界
    bounds: ViewBounds,
}

/// 世界坐标系的可见区域边界
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl Viewport {
    /// 创建新的视口
    pub fn new(width: u32, height: u32, bounds: ViewBounds) -> Self {
        let size = Vector2::new(width, height);
        let (transform, inverse_transform) = Self::calculate_transforms(size, &bounds);

        Self {
            transform,
            inverse_transform,
            size,
            bounds,
        }
    }

    /// 从数据范围自动计算视口
    pub fn from_data_range(
        width: u32,
        height: u32,
        data_min: (f64, f64),
        data_max: (f64, f64),
        margin_percent: f64,
    ) -> Self {
        let margin_x = (data_max.0 - data_min.0) * margin_percent;
        let margin_y = (data_max.1 - data_min.1) * margin_percent;

        let bounds = ViewBounds {
            min_x: data_min.0 - margin_x,
            max_x: data_max.0 + margin_x,
            min_y: data_min.1 - margin_y,
            max_y: data_max.1 + margin_y,
        };

        Self::new(width, height, bounds)
    }

    /// 将屏幕坐标转换为世界坐标
    pub fn screen_to_world(&self, screen_pos: LogicalPosition) -> WorldPosition {
        let screen_vec = Vector3::new(screen_pos.x, screen_pos.y, 1.0);
        let world_vec = self.inverse_transform * screen_vec;
        WorldPosition {
            x: world_vec[0],
            y: world_vec[1],
        }
    }

    /// 将世界坐标转换为屏幕坐标
    pub fn world_to_screen(&self, world_pos: WorldPosition) -> LogicalPosition {
        let world_vec = Vector3::new(world_pos.x, world_pos.y, 1.0);
        let screen_vec = self.transform * world_vec;
        LogicalPosition {
            x: screen_vec[0],
            y: screen_vec[1],
        }
    }

    /// 缩放视口（以指定点为中心）
    pub fn zoom_at_point(&mut self, zoom_factor: f64, center: LogicalPosition) -> Result<()> {
        if zoom_factor <= 0.0 {
            return Err(format!("缩放因子必须为正数，当前值: {}", zoom_factor).into());
        }

        // 将中心点转换为世界坐标
        let world_center = self.screen_to_world(center);

        // 计算新的边界
        let width = self.bounds.max_x - self.bounds.min_x;
        let height = self.bounds.max_y - self.bounds.min_y;

        let new_width = width / zoom_factor;
        let new_height = height / zoom_factor;

        self.bounds = ViewBounds {
            min_x: world_center.x - new_width / 2.0,
            max_x: world_center.x + new_width / 2.0,
            min_y: world_center.y - new_height / 2.0,
            max_y: world_center.y + new_height / 2.0,
        };

        self.update_transforms();
        Ok(())
    }

    /// 平移视口
    pub fn pan(&mut self, delta_screen: Vector2<f64>) -> Result<()> {
        // 将屏幕坐标的偏移转换为世界坐标的偏移
        let origin_world = self.screen_to_world(LogicalPosition { x: 0.0, y: 0.0 });
        let delta_world_pos = self.screen_to_world(LogicalPosition {
            x: delta_screen.x,
            y: delta_screen.y,
        });

        let delta_world = Vector2::new(
            delta_world_pos.x - origin_world.x,
            delta_world_pos.y - origin_world.y,
        );

        self.bounds.min_x -= delta_world.x;
        self.bounds.max_x -= delta_world.x;
        self.bounds.min_y -= delta_world.y;
        self.bounds.max_y -= delta_world.y;

        self.update_transforms();
        Ok(())
    }

    /// 适应指定的世界坐标边界
    pub fn fit_bounds(&mut self, bounds: ViewBounds) {
        self.bounds = bounds;
        self.update_transforms();
    }

    /// 重置为默认视图
    pub fn reset(&mut self, bounds: ViewBounds) {
        self.bounds = bounds;
        self.update_transforms();
    }

    /// 调整视口大小
    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = Vector2::new(width, height);
        self.update_transforms();
    }

    /// 获取当前视口边界
    pub fn bounds(&self) -> &ViewBounds {
        &self.bounds
    }

    /// 获取视口大小
    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    /// 获取缩放级别（相对于初始视图）
    pub fn zoom_level(&self) -> f64 {
        let width = self.bounds.max_x - self.bounds.min_x;
        // 简化计算：使用宽度作为缩放参考
        1.0 / width
    }

    /// 检查世界坐标点是否在可见区域内
    pub fn contains_world_point(&self, point: WorldPosition) -> bool {
        point.x >= self.bounds.min_x
            && point.x <= self.bounds.max_x
            && point.y >= self.bounds.min_y
            && point.y <= self.bounds.max_y
    }

    /// 计算变换矩阵
    fn calculate_transforms(
        size: Vector2<u32>,
        bounds: &ViewBounds,
    ) -> (Matrix3<f64>, Matrix3<f64>) {
        let width = size.x as f64;
        let height = size.y as f64;

        let world_width = bounds.max_x - bounds.min_x;
        let world_height = bounds.max_y - bounds.min_y;

        // 计算缩放因子
        let scale_x = width / world_width;
        let scale_y = height / world_height;

        // 注意：在屏幕坐标系中，Y轴通常向下，而在数据坐标系中Y轴向上
        // 所以我们需要翻转Y轴
        let transform = Matrix3::new(
            scale_x,
            0.0,
            -bounds.min_x * scale_x,
            0.0,
            -scale_y,
            bounds.max_y * scale_y,
            0.0,
            0.0,
            1.0,
        );

        let inverse_transform = transform.try_inverse().unwrap_or_else(|| {
            // 如果无法计算逆矩阵，返回单位矩阵
            Matrix3::identity()
        });

        (transform, inverse_transform)
    }

    /// 更新变换矩阵
    fn update_transforms(&mut self) {
        let (transform, inverse_transform) = Self::calculate_transforms(self.size, &self.bounds);
        self.transform = transform;
        self.inverse_transform = inverse_transform;
    }
}

impl ViewBounds {
    /// 创建新的视图边界
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// 从中心点和尺寸创建边界
    pub fn from_center_and_size(center: (f64, f64), width: f64, height: f64) -> Self {
        Self {
            min_x: center.0 - width / 2.0,
            max_x: center.0 + width / 2.0,
            min_y: center.1 - height / 2.0,
            max_y: center.1 + height / 2.0,
        }
    }

    /// 获取边界的宽度
    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    /// 获取边界的高度
    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    /// 获取边界的中心点
    pub fn center(&self) -> (f64, f64) {
        (
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }

    /// 检查是否包含指定点
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// 扩展边界以包含指定点
    pub fn expand_to_include(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    /// 按比例扩展边界
    pub fn expand_by_factor(&mut self, factor: f64) {
        let center = self.center();
        let width = self.width() * factor;
        let height = self.height() * factor;

        self.min_x = center.0 - width / 2.0;
        self.max_x = center.0 + width / 2.0;
        self.min_y = center.1 - height / 2.0;
        self.max_y = center.1 + height / 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_creation() {
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        let viewport = Viewport::new(800, 600, bounds);

        assert_eq!(viewport.size(), Vector2::new(800, 600));
        assert_eq!(viewport.bounds().width(), 10.0);
        assert_eq!(viewport.bounds().height(), 10.0);
    }

    #[test]
    fn test_coordinate_transformation() {
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        let viewport = Viewport::new(800, 600, bounds);

        // 测试往返转换的精度
        let original_world = WorldPosition { x: 5.0, y: 5.0 };
        let screen_point = viewport.world_to_screen(original_world);
        let back_to_world = viewport.screen_to_world(screen_point);

        assert!((back_to_world.x - original_world.x).abs() < 1e-10);
        assert!((back_to_world.y - original_world.y).abs() < 1e-10);

        // 测试世界坐标原点转换
        let world_origin = viewport.screen_to_world(LogicalPosition { x: 0.0, y: 600.0 });
        assert!((world_origin.x - 0.0).abs() < 1e-10);
        assert!((world_origin.y - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_zoom() {
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        let mut viewport = Viewport::new(800, 600, bounds);

        let center = LogicalPosition { x: 400.0, y: 300.0 };
        viewport.zoom_at_point(2.0, center).unwrap();

        // 缩放2倍后，视图范围应该减半
        assert!((viewport.bounds().width() - 5.0).abs() < 1e-10);
        assert!((viewport.bounds().height() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_pan() {
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        let mut viewport = Viewport::new(800, 600, bounds);

        let original_bounds = viewport.bounds().clone();
        let delta = Vector2::new(100.0, 50.0);
        viewport.pan(delta).unwrap();

        // 平移后边界应该发生变化
        assert_ne!(viewport.bounds(), &original_bounds);
    }

    #[test]
    fn test_view_bounds() {
        let mut bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);

        assert_eq!(bounds.width(), 10.0);
        assert_eq!(bounds.height(), 10.0);
        assert_eq!(bounds.center(), (5.0, 5.0));
        assert!(bounds.contains(5.0, 5.0));
        assert!(!bounds.contains(15.0, 5.0));

        bounds.expand_to_include(15.0, 15.0);
        assert_eq!(bounds.max_x, 15.0);
        assert_eq!(bounds.max_y, 15.0);
    }

    #[test]
    fn test_viewport_from_data_range() {
        let viewport = Viewport::from_data_range(800, 600, (0.0, 0.0), (100.0, 200.0), 0.1);

        // 应该有10%的边距
        assert!((viewport.bounds().min_x - (-10.0)).abs() < 1e-10);
        assert!((viewport.bounds().max_x - 110.0).abs() < 1e-10);
        assert!((viewport.bounds().min_y - (-20.0)).abs() < 1e-10);
        assert!((viewport.bounds().max_y - 220.0).abs() < 1e-10);
    }

    #[test]
    fn test_viewport_resize() {
        let bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        let mut viewport = Viewport::new(800, 600, bounds);

        viewport.resize(1600, 1200);
        assert_eq!(viewport.size(), Vector2::new(1600, 1200));

        // 坐标转换应该仍然正常工作
        let world_point = WorldPosition { x: 5.0, y: 5.0 };
        let screen_point = viewport.world_to_screen(world_point);
        let back_to_world = viewport.screen_to_world(screen_point);

        assert!((back_to_world.x - 5.0).abs() < 1e-10);
        assert!((back_to_world.y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounds_operations() {
        let bounds = ViewBounds::from_center_and_size((5.0, 5.0), 10.0, 8.0);

        assert_eq!(bounds.min_x, 0.0);
        assert_eq!(bounds.max_x, 10.0);
        assert_eq!(bounds.min_y, 1.0);
        assert_eq!(bounds.max_y, 9.0);
        assert_eq!(bounds.center(), (5.0, 5.0));

        let mut expandable_bounds = ViewBounds::new(0.0, 10.0, 0.0, 10.0);
        expandable_bounds.expand_by_factor(1.5);

        assert!((expandable_bounds.width() - 15.0).abs() < 1e-10);
        assert!((expandable_bounds.height() - 15.0).abs() < 1e-10);
    }
}
