// 支持光照的3D着色器 (WGSL)

// 顶点输入
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

// 顶点输出 / 片段输入
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

// 相机和变换矩阵
struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_position: vec3<f32>,
    _padding: f32,
}

// 光源数据（std140 风格对齐，填充到 80 字节）
struct Light {
    position: vec3<f32>,     // 12 bytes
    light_type: f32,         // 4 bytes  - 16 bytes total
    direction: vec3<f32>,    // 12 bytes
    intensity: f32,          // 4 bytes  - 32 bytes total
    color: vec3<f32>,        // 12 bytes
    enabled: f32,            // 4 bytes  - 48 bytes total
    radius: f32,             // 4 bytes
    inner_angle: f32,        // 4 bytes
    _padding: vec2<f32>,     // 8 bytes  - 64 bytes total
    _extra_pad: vec3<f32>,   // 12 bytes
    _pad_end: f32,           // 4 bytes，显式补齐到 80 字节
}

// 材质数据 (确保16字节对齐)
struct Material {
    albedo: vec3<f32>,
    metallic: f32,
    roughness: f32,
    ao: f32,
    _padding1: vec2<f32>,
    emissive: vec3<f32>,
    _padding2: f32,
}

// 光照统一缓冲区（头部 32 字节，lights 数组 8*80 = 640 字节，总共 672 字节）
struct LightingUniform {
    ambient_color: vec3<f32>, // 12 bytes
    ambient_intensity: f32,   // 4 bytes   - 16 bytes total
    num_lights: f32,          // 4 bytes
    _padding: f32,            // 4 bytes
    _padding2: f32,           // 4 bytes   - 32 bytes total
    lights: array<Light, 8>, // 8 * 80字节 = 640字节，总计 = 672字节
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> lighting: LightingUniform;

@group(2) @binding(0)
var<uniform> material: Material;

// 顶点着色器
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // 世界空间位置
    out.world_position = in.position;
    
    // 世界空间法向量 (假设没有非均匀缩放)
    out.world_normal = normalize(in.normal);
    
    // 投影到裁剪空间
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    
    // 传递颜色
    out.color = in.color;
    
    return out;
}

// 计算平行光照明
fn calculate_directional_light(
    light: Light,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    material_data: Material
) -> vec3<f32> {
    let light_dir = normalize(-light.direction);
    
    // 漫反射 (Lambert)
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let diffuse = material_data.albedo * light.color * n_dot_l * light.intensity;
    
    // 镜面反射 (Blinn-Phong)
    let halfway = normalize(light_dir + view_dir);
    let n_dot_h = max(dot(normal, halfway), 0.0);
    let spec_power = 32.0 * (1.0 - material_data.roughness);
    let spec_strength = pow(n_dot_h, spec_power);
    let specular = light.color * spec_strength * light.intensity * (1.0 - material_data.roughness);
    
    return diffuse + specular;
}

// 计算点光源照明
fn calculate_point_light(
    light: Light,
    world_pos: vec3<f32>,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    material_data: Material
) -> vec3<f32> {
    let light_vec = light.position - world_pos;
    let distance = length(light_vec);
    let light_dir = light_vec / distance;
    
    // 距离衰减
    let attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);
    
    // 漫反射
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let diffuse = material_data.albedo * light.color * n_dot_l * light.intensity * attenuation;
    
    // 镜面反射
    let halfway = normalize(light_dir + view_dir);
    let n_dot_h = max(dot(normal, halfway), 0.0);
    let spec_power = 32.0 * (1.0 - material_data.roughness);
    let spec_strength = pow(n_dot_h, spec_power);
    let specular = light.color * spec_strength * light.intensity * attenuation * (1.0 - material_data.roughness);
    
    return diffuse + specular;
}

// 计算聚光灯照明
fn calculate_spot_light(
    light: Light,
    world_pos: vec3<f32>,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    material_data: Material
) -> vec3<f32> {
    let light_vec = light.position - world_pos;
    let distance = length(light_vec);
    let light_dir = light_vec / distance;
    
    // 检查是否在聚光灯锥内
    let spot_factor = dot(-light_dir, normalize(light.direction));
    let cos_inner = cos(light.inner_angle);
    let cos_outer = cos(light.radius); // 复用radius字段作为外角
    
    if spot_factor < cos_outer {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    
    // 边缘渐变
    let spot_intensity = smoothstep(cos_outer, cos_inner, spot_factor);
    
    // 距离衰减
    let attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);
    
    // 计算光照 (与点光源相同，但乘以聚光灯强度)
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let diffuse = material_data.albedo * light.color * n_dot_l * light.intensity * attenuation * spot_intensity;
    
    let halfway = normalize(light_dir + view_dir);
    let n_dot_h = max(dot(normal, halfway), 0.0);
    let spec_power = 32.0 * (1.0 - material_data.roughness);
    let spec_strength = pow(n_dot_h, spec_power);
    let specular = light.color * spec_strength * light.intensity * attenuation * spot_intensity * (1.0 - material_data.roughness);
    
    return diffuse + specular;
}

// 片段着色器
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let view_dir = normalize(camera.camera_position - in.world_position);
    
    // 环境光
    var final_color = lighting.ambient_color * lighting.ambient_intensity * material.albedo;
    
    // 计算所有光源的贡献
    let num_lights_u32 = u32(lighting.num_lights);
    for (var i: u32 = 0u; i < num_lights_u32 && i < 8u; i = i + 1u) {
        let light = lighting.lights[i];
        
        if light.enabled == 0.0 {
            continue;
        }
        
        var light_contribution: vec3<f32>;
        
        let light_type_i = i32(light.light_type);
        switch light_type_i {
            case 0: { // Directional
                light_contribution = calculate_directional_light(light, normal, view_dir, material);
            }
            case 1: { // Point
                light_contribution = calculate_point_light(light, in.world_position, normal, view_dir, material);
            }
            case 2: { // Spot
                light_contribution = calculate_spot_light(light, in.world_position, normal, view_dir, material);
            }
            default: {
                light_contribution = vec3<f32>(0.0, 0.0, 0.0);
            }
        }
        
        final_color = final_color + light_contribution;
    }
    
    // 添加自发光
    final_color = final_color + material.emissive;
    
    // 简单的tone mapping和gamma校正
    final_color = final_color / (final_color + vec3<f32>(1.0, 1.0, 1.0));
    final_color = pow(final_color, vec3<f32>(1.0/2.2, 1.0/2.2, 1.0/2.2));
    
    return vec4<f32>(final_color, 1.0);
}
