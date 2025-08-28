use nalgebra::{Matrix4, Point3, Vector3};

/// 3D 相机控制器
#[derive(Debug, Clone)]
pub struct Camera3D {
    /// 相机位置
    pub position: Point3<f32>,
    /// 目标点
    pub target: Point3<f32>,
    /// 上方向
    pub up: Vector3<f32>,
    /// 视野角度 (弧度)
    pub fov: f32,
    /// 宽高比
    pub aspect_ratio: f32,
    /// 近裁剪面
    pub near: f32,
    /// 远裁剪面
    pub far: f32,
}

impl Camera3D {
    /// 创建新的相机
    pub fn new() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: std::f32::consts::PI / 4.0, // 45度
            aspect_ratio: 4.0 / 3.0,
            near: 0.1,
            far: 100.0,
        }
    }

    /// 设置相机位置
    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = Point3::new(x, y, z);
        self
    }

    /// 设置目标点
    pub fn target(mut self, x: f32, y: f32, z: f32) -> Self {
        self.target = Point3::new(x, y, z);
        self
    }

    /// 设置视野角度 (角度)
    pub fn fov_degrees(mut self, degrees: f32) -> Self {
        self.fov = degrees.to_radians();
        self
    }

    /// 设置宽高比
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    /// 设置裁剪面
    pub fn clip_planes(mut self, near: f32, far: f32) -> Self {
        self.near = near;
        self.far = far;
        self
    }

    /// 获取视图矩阵
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    /// 获取投影矩阵
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect_ratio, self.fov, self.near, self.far)
    }

    /// 绕目标点旋转 (轨道控制)
    pub fn orbit(&mut self, horizontal_angle: f32, vertical_angle: f32) {
        // 计算当前相机到目标的向量
        let offset = self.position - self.target;
        let distance = offset.magnitude();

        // 转换为球坐标
        let mut theta = offset.z.atan2(offset.x); // 水平角度
        let mut phi = (offset.y / distance).asin(); // 垂直角度

        // 应用旋转
        theta += horizontal_angle;
        phi += vertical_angle;

        // 限制垂直角度
        phi = phi.clamp(
            -std::f32::consts::PI / 2.0 + 0.1,
            std::f32::consts::PI / 2.0 - 0.1,
        );

        // 转换回笛卡尔坐标
        let new_offset = Vector3::new(
            distance * phi.cos() * theta.cos(),
            distance * phi.sin(),
            distance * phi.cos() * theta.sin(),
        );

        self.position = self.target + new_offset;
    }

    /// 缩放 (改变距离)
    pub fn zoom(&mut self, factor: f32) {
        let direction = (self.position - self.target).normalize();
        let distance = (self.position - self.target).magnitude();
        let new_distance = (distance * factor).max(0.1);

        self.position = self.target + direction * new_distance;
    }

    /// 平移目标点
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward);

        let delta = right * delta_x + up * delta_y;
        self.position += delta;
        self.target += delta;
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera3D::new();
        assert_eq!(camera.position, Point3::new(0.0, 0.0, 5.0));
        assert_eq!(camera.target, Point3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_camera_configuration() {
        let camera = Camera3D::new()
            .position(1.0, 2.0, 3.0)
            .target(4.0, 5.0, 6.0)
            .fov_degrees(60.0)
            .aspect_ratio(16.0 / 9.0);

        assert_eq!(camera.position, Point3::new(1.0, 2.0, 3.0));
        assert_eq!(camera.target, Point3::new(4.0, 5.0, 6.0));
        assert_eq!(camera.aspect_ratio, 16.0 / 9.0);
    }

    #[test]
    fn test_camera_matrices() {
        let camera = Camera3D::new();

        // 测试矩阵生成不会panic
        let _view = camera.view_matrix();
        let _projection = camera.projection_matrix();
    }

    #[test]
    fn test_camera_zoom() {
        let mut camera = Camera3D::new();
        let initial_distance = (camera.position - camera.target).magnitude();

        camera.zoom(0.5); // 放大
        let new_distance = (camera.position - camera.target).magnitude();

        assert!(new_distance < initial_distance);
    }

    #[test]
    fn test_camera_orbit() {
        let mut camera = Camera3D::new();
        let initial_position = camera.position;

        camera.orbit(0.1, 0.1);

        // 位置应该发生变化
        assert_ne!(camera.position, initial_position);

        // 但距离应该保持不变
        let initial_distance = (initial_position - camera.target).magnitude();
        let new_distance = (camera.position - camera.target).magnitude();
        assert!((initial_distance - new_distance).abs() < 0.001);
    }
}
