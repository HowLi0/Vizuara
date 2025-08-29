//! 3D体积渲染
//!
//! 用于科学数据的体积可视化，如医学成像、流体仿真等

use crate::BoundingBox3D;
use nalgebra::Point3;
use vizuara_core::Color;

/// 体积数据点
#[derive(Debug, Clone)]
pub struct VolumeData {
    /// 3D网格数据 (x, y, z, 标量值)
    pub data: Vec<Vec<Vec<f32>>>,
    /// 数据维度 (width, height, depth)
    pub dimensions: (usize, usize, usize),
    /// 物理空间的边界
    pub bounds: BoundingBox3D,
    /// 数据范围 (min_value, max_value)
    pub value_range: (f32, f32),
}

impl VolumeData {
    /// 从3D数组创建体积数据
    pub fn from_array(
        data: Vec<Vec<Vec<f32>>>,
        bounds: BoundingBox3D,
    ) -> Self {
        let depth = data.len();
        let height = if depth > 0 { data[0].len() } else { 0 };
        let width = if height > 0 { data[0][0].len() } else { 0 };

        // 计算数据范围
        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;

        for z in &data {
            for y in z {
                for &value in y {
                    min_val = min_val.min(value);
                    max_val = max_val.max(value);
                }
            }
        }

        Self {
            data,
            dimensions: (width, height, depth),
            bounds,
            value_range: (min_val, max_val),
        }
    }

    /// 从函数生成体积数据
    pub fn from_function<F>(
        bounds: BoundingBox3D,
        resolution: (usize, usize, usize),
        func: F,
    ) -> Self
    where
        F: Fn(f32, f32, f32) -> f32,
    {
        let (x_min, x_max) = bounds.0;
        let (y_min, y_max) = bounds.1;
        let (z_min, z_max) = bounds.2;
        let (width, height, depth) = resolution;

        let mut data = Vec::with_capacity(depth);
        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;

        for k in 0..depth {
            let z = z_min + (z_max - z_min) * k as f32 / (depth - 1) as f32;
            let mut z_slice = Vec::with_capacity(height);

            for j in 0..height {
                let y = y_min + (y_max - y_min) * j as f32 / (height - 1) as f32;
                let mut y_row = Vec::with_capacity(width);

                for i in 0..width {
                    let x = x_min + (x_max - x_min) * i as f32 / (width - 1) as f32;
                    let value = func(x, y, z);
                    
                    min_val = min_val.min(value);
                    max_val = max_val.max(value);
                    y_row.push(value);
                }
                z_slice.push(y_row);
            }
            data.push(z_slice);
        }

        Self {
            data,
            dimensions: resolution,
            bounds,
            value_range: (min_val, max_val),
        }
    }

    /// 获取指定位置的值
    pub fn get_value(&self, x: usize, y: usize, z: usize) -> Option<f32> {
        self.data.get(z)?.get(y)?.get(x).copied()
    }

    /// 三线性插值获取任意位置的值
    pub fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        let (x_min, x_max) = self.bounds.0;
        let (y_min, y_max) = self.bounds.1;
        let (z_min, z_max) = self.bounds.2;

        // 将世界坐标转换为数组索引
        let fx = (x - x_min) / (x_max - x_min) * (self.dimensions.0 - 1) as f32;
        let fy = (y - y_min) / (y_max - y_min) * (self.dimensions.1 - 1) as f32;
        let fz = (z - z_min) / (z_max - z_min) * (self.dimensions.2 - 1) as f32;

        // 边界检查
        if fx < 0.0 || fy < 0.0 || fz < 0.0 
            || fx >= (self.dimensions.0 - 1) as f32
            || fy >= (self.dimensions.1 - 1) as f32
            || fz >= (self.dimensions.2 - 1) as f32 {
            return 0.0;
        }

        // 三线性插值
        let x0 = fx.floor() as usize;
        let y0 = fy.floor() as usize;
        let z0 = fz.floor() as usize;
        let x1 = (x0 + 1).min(self.dimensions.0 - 1);
        let y1 = (y0 + 1).min(self.dimensions.1 - 1);
        let z1 = (z0 + 1).min(self.dimensions.2 - 1);

        let dx = fx - x0 as f32;
        let dy = fy - y0 as f32;
        let dz = fz - z0 as f32;

        // 8个角的值
        let v000 = self.get_value(x0, y0, z0).unwrap_or(0.0);
        let v100 = self.get_value(x1, y0, z0).unwrap_or(0.0);
        let v010 = self.get_value(x0, y1, z0).unwrap_or(0.0);
        let v110 = self.get_value(x1, y1, z0).unwrap_or(0.0);
        let v001 = self.get_value(x0, y0, z1).unwrap_or(0.0);
        let v101 = self.get_value(x1, y0, z1).unwrap_or(0.0);
        let v011 = self.get_value(x0, y1, z1).unwrap_or(0.0);
        let v111 = self.get_value(x1, y1, z1).unwrap_or(0.0);

        // 三线性插值
        let c00 = v000 * (1.0 - dx) + v100 * dx;
        let c01 = v001 * (1.0 - dx) + v101 * dx;
        let c10 = v010 * (1.0 - dx) + v110 * dx;
        let c11 = v011 * (1.0 - dx) + v111 * dx;

        let c0 = c00 * (1.0 - dy) + c10 * dy;
        let c1 = c01 * (1.0 - dy) + c11 * dy;

        c0 * (1.0 - dz) + c1 * dz
    }
}

/// 传输函数 - 将标量值映射到颜色和透明度
#[derive(Debug, Clone)]
pub struct TransferFunction {
    /// 控制点 (标量值, 颜色, 透明度)
    control_points: Vec<(f32, Color, f32)>,
}

impl TransferFunction {
    /// 创建新的传输函数
    pub fn new() -> Self {
        Self {
            control_points: vec![
                (0.0, Color::rgb(0.0, 0.0, 1.0), 0.0),   // 蓝色，透明
                (0.5, Color::rgb(0.0, 1.0, 0.0), 0.5),   // 绿色，半透明
                (1.0, Color::rgb(1.0, 0.0, 0.0), 1.0),   // 红色，不透明
            ],
        }
    }

    /// 添加控制点
    pub fn add_control_point(mut self, value: f32, color: Color, alpha: f32) -> Self {
        self.control_points.push((value, color, alpha));
        // 按值排序
        self.control_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self
    }

    /// 根据标量值获取颜色和透明度
    pub fn sample(&self, value: f32) -> (Color, f32) {
        if self.control_points.is_empty() {
            return (Color::rgb(1.0, 1.0, 1.0), 1.0);
        }

        // 查找插值区间
        if value <= self.control_points[0].0 {
            let (_, color, alpha) = self.control_points[0];
            return (color, alpha);
        }

        for i in 1..self.control_points.len() {
            if value <= self.control_points[i].0 {
                let (v0, c0, a0) = self.control_points[i - 1];
                let (v1, c1, a1) = self.control_points[i];

                // 线性插值
                let t = (value - v0) / (v1 - v0);
                let color = Color::new(
                    c0.r * (1.0 - t) + c1.r * t,
                    c0.g * (1.0 - t) + c1.g * t,
                    c0.b * (1.0 - t) + c1.b * t,
                    c0.a * (1.0 - t) + c1.a * t,
                );
                let alpha = a0 * (1.0 - t) + a1 * t;
                return (color, alpha);
            }
        }

        // 超出范围，返回最后一个控制点
        let (_, color, alpha) = self.control_points.last().unwrap();
        (*color, *alpha)
    }
}

/// 体积渲染器
#[derive(Debug, Clone)]
pub struct VolumeRenderer {
    /// 体积数据
    volume_data: VolumeData,
    /// 传输函数
    transfer_function: TransferFunction,
    /// 采样步长
    step_size: f32,
    /// 最大采样步数
    max_steps: usize,
}

impl VolumeRenderer {
    /// 创建新的体积渲染器
    pub fn new(volume_data: VolumeData) -> Self {
        Self {
            volume_data,
            transfer_function: TransferFunction::new(),
            step_size: 0.01,
            max_steps: 500,
        }
    }

    /// 设置传输函数
    pub fn transfer_function(mut self, tf: TransferFunction) -> Self {
        self.transfer_function = tf;
        self
    }

    /// 设置采样参数
    pub fn sampling(mut self, step_size: f32, max_steps: usize) -> Self {
        self.step_size = step_size;
        self.max_steps = max_steps;
        self
    }

    /// 光线投射渲染
    pub fn ray_cast(&self, ray_origin: Point3<f32>, ray_direction: nalgebra::Vector3<f32>) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0, 0.0);
        let mut current_pos = ray_origin;
        let step_vec = ray_direction.normalize() * self.step_size;

        for _step in 0..self.max_steps {
            // 采样体积数据
            let density = self.volume_data.sample(current_pos.x, current_pos.y, current_pos.z);
            
            // 归一化密度值
            let normalized_density = (density - self.volume_data.value_range.0) 
                / (self.volume_data.value_range.1 - self.volume_data.value_range.0);
            
            if normalized_density > 0.001 {
                // 获取颜色和透明度
                let (sample_color, alpha) = self.transfer_function.sample(normalized_density);
                
                // Alpha混合
                let source_alpha = alpha * normalized_density * self.step_size;
                color.r += sample_color.r * source_alpha * (1.0 - color.a);
                color.g += sample_color.g * source_alpha * (1.0 - color.a);
                color.b += sample_color.b * source_alpha * (1.0 - color.a);
                color.a += source_alpha * (1.0 - color.a);

                // 早期终止（不透明度足够高）
                if color.a > 0.95 {
                    break;
                }
            }

            // 移动到下一个采样点
            current_pos += step_vec;

            // 边界检查
            let (x_min, x_max) = self.volume_data.bounds.0;
            let (y_min, y_max) = self.volume_data.bounds.1;
            let (z_min, z_max) = self.volume_data.bounds.2;

            if current_pos.x < x_min || current_pos.x > x_max
                || current_pos.y < y_min || current_pos.y > y_max
                || current_pos.z < z_min || current_pos.z > z_max {
                break;
            }
        }

        color
    }

    /// 获取体积数据引用
    pub fn volume_data(&self) -> &VolumeData {
        &self.volume_data
    }

    /// 获取传输函数引用
    pub fn get_transfer_function(&self) -> &TransferFunction {
        &self.transfer_function
    }
}

impl Default for TransferFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_data_creation() {
        let data = vec![
            vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            vec![vec![5.0, 6.0], vec![7.0, 8.0]],
        ];
        let bounds = ((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0));
        
        let volume = VolumeData::from_array(data, bounds);
        assert_eq!(volume.dimensions, (2, 2, 2));
        assert_eq!(volume.value_range, (1.0, 8.0));
    }

    #[test]
    fn test_volume_data_from_function() {
        let bounds = ((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0));
        let volume = VolumeData::from_function(bounds, (10, 10, 10), |x, y, z| {
            (x * x + y * y + z * z).sqrt()
        });
        
        assert_eq!(volume.dimensions, (10, 10, 10));
        assert!(volume.value_range.0 >= 0.0);
    }

    #[test]
    fn test_transfer_function() {
        let tf = TransferFunction::new()
            .add_control_point(0.2, Color::rgb(1.0, 0.0, 0.0), 0.1)
            .add_control_point(0.8, Color::rgb(0.0, 0.0, 1.0), 0.9);
        
        let (color, alpha) = tf.sample(0.5);
        assert!(color.r > 0.0 || color.b > 0.0);
        assert!(alpha > 0.0 && alpha < 1.0);
    }
}
