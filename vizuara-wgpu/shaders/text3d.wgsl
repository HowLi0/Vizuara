// 3D文本渲染着色器
struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) char_code: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) char_code: u32,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.tex_coord = model.tex_coord;
    out.color = model.color;
    out.char_code = model.char_code;
    return out;
}

// 简化的字体渲染函数 - 生成数字和字母
fn sdf_digit(p: vec2<f32>, digit: i32) -> f32 {
    let size = 0.4;
    let thickness = 0.1;
    
    // 简化的7段数码管风格数字
    if (digit == 0) {
        let outline = length(p) - size;
        let hole = length(p) - (size - thickness);
        return max(outline, -hole);
    } else if (digit == 1) {
        let line = abs(p.x - 0.2) - thickness * 0.5;
        let bounds = step(-size, p.y) * step(p.y, size);
        return line * bounds + (1.0 - bounds);
    } else {
        // 其他数字简化为方框
        let box_sdf = max(abs(p.x) - size, abs(p.y) - size);
        return box_sdf;
    }
}

fn sdf_char(p: vec2<f32>, char_code: u32) -> f32 {
    // ASCII 字符简化渲染
    if (char_code >= 48u && char_code <= 57u) {
        // 数字 0-9
        return sdf_digit(p, i32(char_code - 48u));
    } else if (char_code >= 65u && char_code <= 90u) {
        // 大写字母 A-Z，简化为方框
        let box_sdf = max(abs(p.x) - 0.3, abs(p.y) - 0.4);
        return box_sdf;
    } else if (char_code >= 97u && char_code <= 122u) {
        // 小写字母 a-z，简化为小方框
        let box_sdf = max(abs(p.x) - 0.25, abs(p.y) - 0.3);
        return box_sdf;
    } else {
        // 其他字符，如句号、连字符等
        let dot_sdf = length(p) - 0.1;
        return dot_sdf;
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 将纹理坐标转换为字符空间
    let char_pos = (in.tex_coord - 0.5) * 2.0;
    // 根据传入的字符编码渲染不同字符
    let sdf = sdf_char(char_pos, in.char_code);
    
    // 抗锯齿边缘
    let alpha = 1.0 - smoothstep(-0.05, 0.05, sdf);
    
    // 对非常小的alpha直接丢弃，减少走样
    let final_alpha = in.color.a * alpha;
    if (final_alpha < 0.02) {
        discard;
    }
    return vec4<f32>(in.color.rgb, final_alpha);
}
