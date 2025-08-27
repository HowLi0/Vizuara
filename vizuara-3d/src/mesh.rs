use vizuara_core::{Primitive, Color};
use nalgebra::{Point3, Vector3, Point2};

/// 3D 三角形面片
#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
    pub normals: [Vector3<f32>; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) -> Self {
        // 计算面法向量
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(&edge2).normalize();

        Self {
            vertices: [v0, v1, v2],
            normals: [normal, normal, normal], // 平面法向量
            color: Color::rgb(0.7, 0.7, 0.9),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// 计算三角形重心
    pub fn centroid(&self) -> Point3<f32> {
        Point3::new(
            (self.vertices[0].x + self.vertices[1].x + self.vertices[2].x) / 3.0,
            (self.vertices[0].y + self.vertices[1].y + self.vertices[2].y) / 3.0,
            (self.vertices[0].z + self.vertices[1].z + self.vertices[2].z) / 3.0,
        )
    }
}

/// 3D 网格
#[derive(Debug, Clone)]
pub struct Mesh3D {
    triangles: Vec<Triangle>,
    bounding_box: Option<((f32, f32), (f32, f32), (f32, f32))>,
}

impl Mesh3D {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            bounding_box: None,
        }
    }

    /// 添加三角形
    pub fn add_triangle(mut self, triangle: Triangle) -> Self {
        self.triangles.push(triangle);
        self.update_bounding_box();
        self
    }

    /// 从顶点和索引创建网格
    pub fn from_vertices_indices(
        vertices: &[Point3<f32>],
        indices: &[usize],
        color: Color
    ) -> Self {
        let mut mesh = Self::new();

        // 每3个索引组成一个三角形
        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                let triangle = Triangle::new(
                    vertices[chunk[0]],
                    vertices[chunk[1]],
                    vertices[chunk[2]]
                ).color(color);
                mesh = mesh.add_triangle(triangle);
            }
        }

        mesh
    }

    /// 创建立方体网格
    pub fn cube(size: f32) -> Self {
        let s = size / 2.0;
        
        // 8个顶点
        let vertices = [
            Point3::new(-s, -s, -s), // 0
            Point3::new( s, -s, -s), // 1
            Point3::new( s,  s, -s), // 2
            Point3::new(-s,  s, -s), // 3
            Point3::new(-s, -s,  s), // 4
            Point3::new( s, -s,  s), // 5
            Point3::new( s,  s,  s), // 6
            Point3::new(-s,  s,  s), // 7
        ];

        // 12个三角形 (每个面2个)
        let indices = [
            // 前面
            0, 1, 2,  0, 2, 3,
            // 后面  
            4, 6, 5,  4, 7, 6,
            // 左面
            0, 3, 7,  0, 7, 4,
            // 右面
            1, 5, 6,  1, 6, 2,
            // 上面
            3, 2, 6,  3, 6, 7,
            // 下面
            0, 4, 5,  0, 5, 1,
        ];

        Self::from_vertices_indices(&vertices, &indices, Color::rgb(0.8, 0.8, 0.9))
    }

    /// 创建球体网格 (简化版 - icosphere)
    pub fn sphere(radius: f32, _subdivisions: usize) -> Self {
        // 简化版：创建八面体然后细分
        let vertices = [
            Point3::new( 0.0,  radius,  0.0), // 顶点
            Point3::new( 0.0, -radius,  0.0), // 底点
            Point3::new( radius,  0.0,  0.0), // 前点
            Point3::new(-radius,  0.0,  0.0), // 后点
            Point3::new( 0.0,  0.0,  radius), // 右点
            Point3::new( 0.0,  0.0, -radius), // 左点
        ];

        let indices = [
            // 上半球
            0, 2, 4,  0, 4, 3,  0, 3, 5,  0, 5, 2,
            // 下半球
            1, 4, 2,  1, 3, 4,  1, 5, 3,  1, 2, 5,
        ];

        // TODO: 实现细分算法
        Self::from_vertices_indices(&vertices, &indices, Color::rgb(0.7, 0.9, 0.7))
    }

    /// 获取三角形数量
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// 获取边界框
    pub fn bounds(&self) -> Option<((f32, f32), (f32, f32), (f32, f32))> {
        self.bounding_box
    }

    /// 获取顶点数量
    pub fn vertex_count(&self) -> usize {
        self.triangles.len() * 3
    }

    /// 获取指定索引的顶点
    pub fn vertex_at(&self, index: usize) -> Option<Point3<f32>> {
        let triangle_index = index / 3;
        let vertex_index = index % 3;
        
        self.triangles.get(triangle_index)
            .map(|t| t.vertices[vertex_index])
    }

    /// 获取指定索引的三角形（返回顶点索引）
    pub fn triangle_at(&self, index: usize) -> Option<(usize, usize, usize)> {
        if index < self.triangles.len() {
            let base = index * 3;
            Some((base, base + 1, base + 2))
        } else {
            None
        }
    }

    /// 更新边界框
    fn update_bounding_box(&mut self) {
        if self.triangles.is_empty() {
            self.bounding_box = None;
            return;
        }

        let first_vertex = &self.triangles[0].vertices[0];
        let mut min_x = first_vertex.x;
        let mut max_x = first_vertex.x;
        let mut min_y = first_vertex.y;
        let mut max_y = first_vertex.y;
        let mut min_z = first_vertex.z;
        let mut max_z = first_vertex.z;

        for triangle in &self.triangles {
            for vertex in &triangle.vertices {
                min_x = min_x.min(vertex.x);
                max_x = max_x.max(vertex.x);
                min_y = min_y.min(vertex.y);
                max_y = max_y.max(vertex.y);
                min_z = min_z.min(vertex.z);
                max_z = max_z.max(vertex.z);
            }
        }

        self.bounding_box = Some(((min_x, max_x), (min_y, max_y), (min_z, max_z)));
    }

    /// 生成渲染图元 (线框模式)
    pub fn generate_wireframe_primitives(&self, plot_area: &crate::Plot3DArea) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        // 创建模型-视图-投影矩阵
        let mvp = plot_area.projection_matrix * plot_area.view_matrix;

        for triangle in &self.triangles {
            // 投影三个顶点
            let mut screen_points = Vec::new();
            
            for vertex in &triangle.vertices {
                let world_pos = nalgebra::Vector4::new(vertex.x, vertex.y, vertex.z, 1.0);
                let clip_pos = mvp * world_pos;

                if clip_pos.w != 0.0 {
                    let ndc_x = clip_pos.x / clip_pos.w;
                    let ndc_y = clip_pos.y / clip_pos.w;
                    let ndc_z = clip_pos.z / clip_pos.w;

                    // 检查是否在视锥体内
                    if ndc_x >= -1.0 && ndc_x <= 1.0 && 
                       ndc_y >= -1.0 && ndc_y <= 1.0 && 
                       ndc_z >= 0.0 && ndc_z <= 1.0 {
                        
                        let screen_x = (ndc_x + 1.0) * 400.0;
                        let screen_y = (1.0 - ndc_y) * 300.0;
                        screen_points.push(Point2::new(screen_x, screen_y));
                    }
                }
            }

            // 如果所有顶点都在屏幕内，绘制三角形边
            if screen_points.len() == 3 {
                // 绘制三条边
                primitives.push(Primitive::Line {
                    start: screen_points[0],
                    end: screen_points[1],
                });
                primitives.push(Primitive::Line {
                    start: screen_points[1],
                    end: screen_points[2],
                });
                primitives.push(Primitive::Line {
                    start: screen_points[2],
                    end: screen_points[0],
                });
            }
        }

        primitives
    }
}

impl Default for Mesh3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_creation() {
        let triangle = Triangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0)
        );

        assert_eq!(triangle.vertices[0], Point3::new(0.0, 0.0, 0.0));
        assert_eq!(triangle.vertices[1], Point3::new(1.0, 0.0, 0.0));
        assert_eq!(triangle.vertices[2], Point3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_triangle_centroid() {
        let triangle = Triangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(0.0, 3.0, 0.0)
        );

        let centroid = triangle.centroid();
        assert_eq!(centroid, Point3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_mesh3d_creation() {
        let mesh = Mesh3D::new();
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_mesh3d_add_triangle() {
        let triangle = Triangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0)
        );

        let mesh = Mesh3D::new().add_triangle(triangle);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_cube_mesh() {
        let cube = Mesh3D::cube(2.0);
        assert_eq!(cube.triangle_count(), 12); // 6 faces * 2 triangles

        let bounds = cube.bounds().unwrap();
        assert_eq!(bounds.0, (-1.0, 1.0)); // X bounds
        assert_eq!(bounds.1, (-1.0, 1.0)); // Y bounds
        assert_eq!(bounds.2, (-1.0, 1.0)); // Z bounds
    }

    #[test]
    fn test_sphere_mesh() {
        let sphere = Mesh3D::sphere(1.0, 0);
        assert_eq!(sphere.triangle_count(), 8); // 八面体有8个面
    }

    #[test]
    fn test_mesh_from_vertices_indices() {
        let vertices = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let indices = [0, 1, 2];

        let mesh = Mesh3D::from_vertices_indices(&vertices, &indices, Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(mesh.triangle_count(), 1);
    }
}
