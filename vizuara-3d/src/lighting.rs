//! 3D 光照系统
//!
//! 提供现代化的PBR光照模型和多种光源类型

use nalgebra::{Point3, Vector3};
use vizuara_core::Color;

/// 光源类型
#[derive(Debug, Clone)]
pub enum LightType {
    /// 平行光 (太阳光)
    Directional { direction: Vector3<f32> },
    /// 点光源
    Point { position: Point3<f32>, radius: f32 },
    /// 聚光灯
    Spot {
        position: Point3<f32>,
        direction: Vector3<f32>,
        inner_angle: f32,
        outer_angle: f32,
    },
}

/// 光源
#[derive(Debug, Clone)]
pub struct Light {
    /// 光源类型
    pub light_type: LightType,
    /// 光源颜色
    pub color: Color,
    /// 光照强度
    pub intensity: f32,
    /// 是否启用
    pub enabled: bool,
}

impl Light {
    /// 创建平行光
    pub fn directional(direction: Vector3<f32>, color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional {
                direction: direction.normalize(),
            },
            color,
            intensity,
            enabled: true,
        }
    }

    /// 创建点光源
    pub fn point(position: Point3<f32>, color: Color, intensity: f32, radius: f32) -> Self {
        Self {
            light_type: LightType::Point { position, radius },
            color,
            intensity,
            enabled: true,
        }
    }

    /// 创建聚光灯
    pub fn spot(
        position: Point3<f32>,
        direction: Vector3<f32>,
        color: Color,
        intensity: f32,
        inner_angle: f32,
        outer_angle: f32,
    ) -> Self {
        Self {
            light_type: LightType::Spot {
                position,
                direction: direction.normalize(),
                inner_angle,
                outer_angle,
            },
            color,
            intensity,
            enabled: true,
        }
    }

    /// 创建默认场景光照 (白色平行光从上方照射)
    pub fn default_scene() -> Vec<Self> {
        vec![
            // 主光源 - 从右上方照射
            Light::directional(
                Vector3::new(-0.3, -0.8, -0.5),
                Color::rgb(1.0, 1.0, 0.95),
                2.5,
            ),
            // 环境光 - 柔和的蓝色填充光
            Light::directional(Vector3::new(0.2, 0.4, 0.8), Color::rgb(0.4, 0.6, 1.0), 0.8),
            // 背光 - 轮廓光
            Light::directional(Vector3::new(0.5, 0.2, 0.8), Color::rgb(1.0, 0.8, 0.6), 0.6),
        ]
    }
}

/// PBR 材质
#[derive(Debug, Clone)]
pub struct Material {
    /// 基础颜色 (反照率)
    pub albedo: Color,
    /// 金属度 (0.0 = 电介质, 1.0 = 金属)
    pub metallic: f32,
    /// 粗糙度 (0.0 = 镜面, 1.0 = 完全粗糙)
    pub roughness: f32,
    /// 环境遮蔽
    pub ao: f32,
    /// 自发光
    pub emissive: Color,
}

impl Material {
    /// 创建新材质
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            metallic: 0.0,
            roughness: 0.5,
            ao: 1.0,
            emissive: Color::rgb(0.0, 0.0, 0.0),
        }
    }

    /// 设置金属度
    pub fn metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic.clamp(0.0, 1.0);
        self
    }

    /// 设置粗糙度
    pub fn roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness.clamp(0.0, 1.0);
        self
    }

    /// 设置环境遮蔽
    pub fn ao(mut self, ao: f32) -> Self {
        self.ao = ao.clamp(0.0, 1.0);
        self
    }

    /// 设置自发光
    pub fn emissive(mut self, emissive: Color) -> Self {
        self.emissive = emissive;
        self
    }

    /// 创建预设材质
    ///
    /// 塑料材质
    pub fn plastic(color: Color) -> Self {
        Self::new(color).metallic(0.0).roughness(0.7)
    }

    /// 金属材质
    pub fn metal(color: Color) -> Self {
        Self::new(color).metallic(1.0).roughness(0.2)
    }

    /// 玻璃材质
    pub fn glass(color: Color) -> Self {
        Self::new(color).metallic(0.0).roughness(0.0)
    }

    /// 陶瓷材质
    pub fn ceramic(color: Color) -> Self {
        Self::new(color).metallic(0.0).roughness(0.8)
    }

    /// 科学数据可视化的默认材质
    pub fn data_visualization() -> Vec<Self> {
        vec![
            // 数据点材质 - 鲜明的塑料质感
            Material::plastic(Color::rgb(0.2, 0.6, 1.0)),
            // 表面材质 - 半透明玻璃效果
            Material::glass(Color::rgb(0.8, 0.9, 1.0)).roughness(0.3),
            // 网格材质 - 金属线框
            Material::metal(Color::rgb(0.7, 0.7, 0.8)).roughness(0.6),
            // 高亮材质 - 自发光
            Material::new(Color::rgb(1.0, 0.4, 0.2)).emissive(Color::rgb(0.3, 0.1, 0.0)),
        ]
    }
}

/// 光照计算器
pub struct LightingCalculator {
    lights: Vec<Light>,
    ambient_color: Color,
    ambient_intensity: f32,
}

impl LightingCalculator {
    /// 创建新的光照计算器
    pub fn new() -> Self {
        Self {
            lights: Light::default_scene(),
            ambient_color: Color::rgb(0.1, 0.1, 0.15),
            ambient_intensity: 0.3,
        }
    }

    /// 添加光源
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// 设置环境光
    pub fn ambient_light(&mut self, color: Color, intensity: f32) {
        self.ambient_color = color;
        self.ambient_intensity = intensity;
    }

    /// 获取所有光源
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    /// 计算光照 (简化的CPU版本，实际渲染在GPU中进行)
    pub fn calculate_lighting(
        &self,
        surface_point: Point3<f32>,
        surface_normal: Vector3<f32>,
        view_direction: Vector3<f32>,
        material: &Material,
    ) -> Color {
        let mut final_color = Color::rgb(0.0, 0.0, 0.0);

        // 环境光
        final_color = final_color + (self.ambient_color * self.ambient_intensity);

        // 计算每个光源的贡献
        for light in &self.lights {
            if !light.enabled {
                continue;
            }

            let light_contribution = match &light.light_type {
                LightType::Directional { direction } => self.calculate_directional_light(
                    *direction,
                    &light.color,
                    light.intensity,
                    surface_normal,
                    view_direction,
                    material,
                ),
                LightType::Point { position, .. } => {
                    let light_direction = (position - surface_point).normalize();
                    self.calculate_point_light(
                        light_direction,
                        &light.color,
                        light.intensity,
                        surface_normal,
                        view_direction,
                        material,
                    )
                }
                LightType::Spot { position, .. } => {
                    let light_direction = (position - surface_point).normalize();
                    // 简化的聚光灯计算
                    self.calculate_point_light(
                        light_direction,
                        &light.color,
                        light.intensity,
                        surface_normal,
                        view_direction,
                        material,
                    )
                }
            };

            final_color = final_color + light_contribution;
        }

        // 添加自发光
        final_color = final_color + material.emissive;

        final_color
    }

    /// 计算平行光照明
    fn calculate_directional_light(
        &self,
        light_direction: Vector3<f32>,
        light_color: &Color,
        light_intensity: f32,
        surface_normal: Vector3<f32>,
        view_direction: Vector3<f32>,
        material: &Material,
    ) -> Color {
        // Lambert 漫反射
        let diffuse_strength = (-light_direction).dot(&surface_normal).max(0.0);
        let diffuse = material.albedo * *light_color * diffuse_strength * light_intensity;

        // Blinn-Phong 镜面反射
        let halfway = (-light_direction + view_direction).normalize();
        let spec_strength = halfway
            .dot(&surface_normal)
            .max(0.0)
            .powf(32.0 * (1.0 - material.roughness));
        let specular = *light_color * spec_strength * light_intensity * (1.0 - material.roughness);

        diffuse + specular
    }

    /// 计算点光源照明
    fn calculate_point_light(
        &self,
        light_direction: Vector3<f32>,
        light_color: &Color,
        light_intensity: f32,
        surface_normal: Vector3<f32>,
        view_direction: Vector3<f32>,
        material: &Material,
    ) -> Color {
        // 与平行光相同的计算，实际应用中会加入距离衰减
        self.calculate_directional_light(
            -light_direction,
            light_color,
            light_intensity,
            surface_normal,
            view_direction,
            material,
        )
    }
}

impl Default for LightingCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_creation() {
        let dir_light =
            Light::directional(Vector3::new(0.0, -1.0, 0.0), Color::rgb(1.0, 1.0, 1.0), 1.0);

        assert!(dir_light.enabled);
        assert_eq!(dir_light.intensity, 1.0);
    }

    #[test]
    fn test_material_creation() {
        let plastic = Material::plastic(Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(plastic.metallic, 0.0);
        assert_eq!(plastic.roughness, 0.7);

        let metal = Material::metal(Color::rgb(0.8, 0.8, 0.8));
        assert_eq!(metal.metallic, 1.0);
        assert_eq!(metal.roughness, 0.2);
    }

    #[test]
    fn test_lighting_calculator() {
        let mut calculator = LightingCalculator::new();
        assert!(!calculator.lights().is_empty());

        calculator.add_light(Light::point(
            Point3::new(0.0, 0.0, 5.0),
            Color::rgb(1.0, 1.0, 1.0),
            1.0,
            10.0,
        ));

        assert_eq!(calculator.lights().len(), 4); // 3 default + 1 added
    }
}
