# Vizuara 技术架构文档

> **版本**: v2.0  
> **最后更新**: 2025年8月28日  
> **架构状态**: 稳定，持续优化中

## 📖 架构概览

### 🎯 设计原则

**Vizuara** 的架构设计遵循以下核心原则：

1. **模块化至上**: 每个功能模块都是独立的 crate，职责清晰，依赖明确
2. **性能优先**: 利用 Rust 零成本抽象和现代 GPU 技术实现极致性能
3. **类型安全**: 在编译时捕获错误，确保运行时稳定性
4. **可扩展性**: 支持插件式扩展和自定义组件
5. **跨平台**: 统一的 API，多平台一致的用户体验

### 🏗️ 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                        用户接口层                             │
├─────────────────────────────────────────────────────────────┤
│  vizuara-window    │  Python绑定   │  Web接口   │  CLI工具   │
└─────────────────────────────────────────────────────────────┘
                                │
┌─────────────────────────────────────────────────────────────┐
│                        应用逻辑层                             │
├─────────────────────────────────────────────────────────────┤
│  vizuara-plots  │ vizuara-3d  │ vizuara-animation │ examples │
└─────────────────────────────────────────────────────────────┘
                                │
┌─────────────────────────────────────────────────────────────┐
│                        功能服务层                             │
├─────────────────────────────────────────────────────────────┤
│ interactivity │  themes  │  export  │  layout  │ components │
└─────────────────────────────────────────────────────────────┘
                                │
┌─────────────────────────────────────────────────────────────┐
│                        渲染引擎层                             │
├─────────────────────────────────────────────────────────────┤
│     vizuara-wgpu     │      vizuara-scene                    │
└─────────────────────────────────────────────────────────────┘
                                │
┌─────────────────────────────────────────────────────────────┐
│                        核心基础层                             │
├─────────────────────────────────────────────────────────────┤
│                     vizuara-core                             │
└─────────────────────────────────────────────────────────────┘
                                │
┌─────────────────────────────────────────────────────────────┐
│                        系统平台层                             │
├─────────────────────────────────────────────────────────────┤
│    wgpu    │   winit   │  nalgebra  │   系统图形驱动          │
└─────────────────────────────────────────────────────────────┘
```

---

## 🧱 模块架构详解

### 🔧 核心基础层

#### `vizuara-core`
**职责**: 提供基础数据结构、坐标系统、错误处理等核心功能

```rust
// 核心模块结构
vizuara-core/
├── src/
│   ├── lib.rs              // 公共 API 导出
│   ├── coords.rs           // 坐标系统和变换
│   ├── primitive.rs        // 基础几何图形
│   ├── scale.rs           // 数据缩放和映射
│   ├── style.rs           // 样式定义
│   └── error.rs           // 错误类型定义
└── Cargo.toml
```

**核心类型**:
```rust
// 基础坐标类型
pub struct Point2D { x: f64, y: f64 }
pub struct Point3D { x: f64, y: f64, z: f64 }

// 变换矩阵
pub struct Transform2D(Matrix3<f64>);
pub struct Transform3D(Matrix4<f64>);

// 数据缩放
pub trait Scale {
    fn scale(&self, value: f64) -> f64;
    fn inverse_scale(&self, value: f64) -> f64;
}

// 统一错误类型
#[derive(Debug, thiserror::Error)]
pub enum VizuaraError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Rendering error: {0}")]
    RenderError(String),
    // ... 其他错误类型
}
```

### 🎨 渲染引擎层

#### `vizuara-wgpu`
**职责**: GPU 加速渲染，着色器管理，缓冲区操作

```rust
// 渲染引擎架构
vizuara-wgpu/
├── src/
│   ├── lib.rs              // 渲染器接口
│   ├── renderer.rs         // 主渲染器
│   ├── renderer_3d.rs      // 3D 渲染器
│   ├── renderer_3d_lit.rs  // 3D 光照渲染器
│   ├── shader.rs           // 着色器管理
│   ├── vertex.rs           // 顶点数据结构
│   └── shaders/            // 内置着色器目录
└── shaders/
    ├── shader_3d.wgsl      // 3D 基础着色器
    └── shader_3d_lit.wgsl  // 3D 光照着色器
```

**渲染管线设计**:
```rust
pub trait Renderer {
    /// 开始新的渲染帧
    fn begin_frame(&mut self) -> Result<(), VizuaraError>;
    
    /// 渲染几何图形
    fn render_geometry(&mut self, geometry: &Geometry) -> Result<(), VizuaraError>;
    
    /// 结束当前帧并提交到 GPU
    fn end_frame(&mut self) -> Result<(), VizuaraError>;
}

// GPU 资源管理
pub struct GpuResources {
    device: wgpu::Device,
    queue: wgpu::Queue,
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffers: Vec<wgpu::Buffer>,
    uniform_buffers: Vec<wgpu::Buffer>,
}
```

#### `vizuara-scene`
**职责**: 场景图管理，图层系统，渲染调度

```rust
// 场景管理系统
pub struct Scene {
    layers: Vec<Layer>,
    camera: Camera,
    viewport: Viewport,
}

pub struct Layer {
    id: LayerId,
    objects: Vec<SceneObject>,
    visible: bool,
    z_order: i32,
}

pub trait SceneObject {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<(), VizuaraError>;
    fn update(&mut self, delta_time: f64);
    fn get_bounds(&self) -> Bounds;
}
```

### 📊 应用逻辑层

#### `vizuara-plots`
**职责**: 2D 图表类型实现

```rust
// 图表模块结构
vizuara-plots/
├── src/
│   ├── lib.rs          // 图表 API 导出
│   ├── scatter.rs      // 散点图
│   ├── line.rs         // 线图
│   ├── bar.rs          // 柱状图
│   ├── histogram.rs    // 直方图
│   ├── heatmap.rs      // 热力图
│   └── boxplot.rs      // 箱线图
└── Cargo.toml
```

**图表设计模式**:
```rust
// 统一的图表接口
pub trait Plot {
    type Data;
    type Style;
    
    fn new(data: Self::Data, style: Self::Style) -> Result<Self, VizuaraError>;
    fn update_data(&mut self, data: Self::Data) -> Result<(), VizuaraError>;
    fn update_style(&mut self, style: Self::Style);
}

// 渲染能力
pub trait Renderable {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<(), VizuaraError>;
}

// 主题应用
pub trait Themeable {
    fn apply_theme(&mut self, theme: &Theme);
}

// 示例：散点图实现
pub struct ScatterPlot {
    data: Vec<Point2D>,
    style: ScatterStyle,
    cached_geometry: Option<Geometry>,
}

impl Plot for ScatterPlot {
    type Data = Vec<Point2D>;
    type Style = ScatterStyle;
    
    fn new(data: Self::Data, style: Self::Style) -> Result<Self, VizuaraError> {
        Ok(Self {
            data,
            style,
            cached_geometry: None,
        })
    }
}
```

#### `vizuara-3d`
**职责**: 3D 可视化功能

```rust
// 3D 模块架构
vizuara-3d/
├── src/
│   ├── lib.rs          // 3D API 导出
│   ├── scatter3d.rs    // 3D 散点图
│   ├── surface.rs      // 表面图
│   ├── mesh.rs         // 网格图
│   ├── camera.rs       // 相机控制
│   └── lighting.rs     // 光照系统
└── Cargo.toml
```

**3D 特有功能**:
```rust
// 3D 相机系统
pub struct Camera3D {
    position: Point3D,
    target: Point3D,
    up: Vector3<f64>,
    fov: f64,
    aspect: f64,
    near: f64,
    far: f64,
}

impl Camera3D {
    pub fn view_matrix(&self) -> Matrix4<f64> { /* ... */ }
    pub fn projection_matrix(&self) -> Matrix4<f64> { /* ... */ }
}

// 光照系统
pub struct Light {
    position: Point3D,
    color: Color,
    intensity: f64,
    light_type: LightType,
}

pub enum LightType {
    Directional(Vector3<f64>),
    Point,
    Spot { direction: Vector3<f64>, angle: f64 },
}
```

### 🎮 功能服务层

#### `vizuara-interactivity`
**职责**: 用户交互处理，事件系统

```rust
// 交互系统架构
pub struct InteractionManager {
    tools: Vec<Box<dyn InteractiveTool>>,
    active_tool: Option<usize>,
    event_queue: VecDeque<InputEvent>,
}

pub trait InteractiveTool {
    fn handle_event(&mut self, event: &InputEvent) -> ToolResponse;
    fn update(&mut self, delta_time: f64);
    fn is_active(&self) -> bool;
}

// 内置交互工具
pub struct PanTool { /* 平移工具 */ }
pub struct ZoomTool { /* 缩放工具 */ }
pub struct SelectTool { /* 选择工具 */ }

// 事件类型
#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseMove { pos: Point2D, delta: Vector2<f64> },
    MouseButton { button: MouseButton, state: ButtonState, pos: Point2D },
    MouseWheel { delta: f64, pos: Point2D },
    Keyboard { key: Key, state: ButtonState, modifiers: Modifiers },
}
```

#### `vizuara-animation`
**职责**: 动画系统，关键帧，缓动函数

```rust
// 动画系统核心
pub struct Timeline {
    animations: Vec<Animation>,
    current_time: f64,
    is_playing: bool,
}

pub struct Animation {
    keyframes: Vec<Keyframe>,
    duration: f64,
    easing: EasingFunction,
    target: AnimationTarget,
}

pub struct Keyframe {
    time: f64,
    value: AnimationValue,
}

// 缓动函数
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    Custom(fn(f64) -> f64),
}

// 动画目标
pub enum AnimationTarget {
    Position(Point2D),
    Color(Color),
    Scale(f64),
    Rotation(f64),
    Custom(String),
}
```

#### `vizuara-themes`
**职责**: 主题管理，颜色调色板

```rust
// 主题系统设计
pub struct Theme {
    name: String,
    colors: ColorPalette,
    typography: Typography,
    spacing: Spacing,
    effects: Effects,
}

pub struct ColorPalette {
    primary: Color,
    secondary: Color,
    background: Color,
    surface: Color,
    accent: Color,
    data_colors: Vec<Color>,
}

// 主题构建器
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    pub fn new() -> Self { /* ... */ }
    pub fn name(mut self, name: &str) -> Self { /* ... */ }
    pub fn primary_color(mut self, color: Color) -> Self { /* ... */ }
    pub fn build(self) -> Theme { /* ... */ }
}

// 预设主题
impl Theme {
    pub fn default() -> Self { /* ... */ }
    pub fn dark() -> Self { /* ... */ }
    pub fn light() -> Self { /* ... */ }
    pub fn colorful() -> Self { /* ... */ }
}
```

#### `vizuara-export`
**职责**: 图表导出功能

```rust
// 导出系统
pub trait Exporter {
    type Options;
    
    fn export<P: AsRef<Path>>(
        &self,
        scene: &Scene,
        path: P,
        options: Self::Options,
    ) -> Result<(), VizuaraError>;
}

// 具体导出器实现
pub struct PngExporter;
pub struct SvgExporter;
pub struct PdfExporter;

// 导出选项
pub struct ExportOptions {
    width: u32,
    height: u32,
    dpi: u32,
    background: Option<Color>,
    quality: f32,
}
```

### 🖼️ 用户接口层

#### `vizuara-window`
**职责**: 窗口管理，平台抽象

```rust
// 窗口系统
pub struct Window {
    inner: winit::window::Window,
    surface: wgpu::Surface,
    renderer: Box<dyn Renderer>,
    event_loop: Option<EventLoop<()>>,
}

impl Window {
    pub fn new(config: WindowConfig) -> Result<Self, VizuaraError> { /* ... */ }
    pub fn run(self, mut app: impl App + 'static) -> Result<(), VizuaraError> { /* ... */ }
}

// 应用接口
pub trait App {
    fn update(&mut self, delta_time: f64);
    fn render(&mut self, renderer: &mut dyn Renderer) -> Result<(), VizuaraError>;
    fn handle_event(&mut self, event: &WindowEvent) -> EventResponse;
}
```

---

## 🔄 数据流架构

### 📊 数据处理管线

```
原始数据 → 数据验证 → 坐标变换 → 几何生成 → GPU缓冲 → 渲染
    ↓         ↓         ↓         ↓         ↓        ↓
[Vec<f64>] → [Valid] → [Scaled] → [Vertex] → [Buffer] → [Frame]
```

**详细数据流**:

1. **数据输入**: 用户提供原始数据（Vec、Array、DataFrame 等）
2. **数据验证**: 检查数据有效性，处理缺失值和异常值
3. **坐标变换**: 将数据坐标映射到屏幕坐标系统
4. **几何生成**: 根据图表类型生成对应的几何图形
5. **GPU 传输**: 将几何数据上传到 GPU 缓冲区
6. **渲染输出**: GPU 渲染并输出到屏幕或文件

### 🎯 事件处理流程

```
硬件事件 → 系统事件 → 窗口事件 → 应用事件 → 工具处理 → 场景更新
    ↓         ↓         ↓         ↓         ↓        ↓
[Raw Input] → [WinIt] → [Window] → [App] → [Tool] → [Scene]
```

**事件处理步骤**:

1. **硬件输入**: 鼠标、键盘、触摸等原始输入
2. **系统处理**: 操作系统处理原始输入并生成事件
3. **窗口分发**: winit 库接收系统事件并分发给窗口
4. **应用处理**: 应用层接收事件并进行初步处理
5. **工具响应**: 交互工具处理特定类型的事件
6. **场景更新**: 根据事件结果更新场景状态

---

## 🧮 内存管理策略

### 📦 资源生命周期

**CPU 端资源**:
```rust
// 智能指针使用策略
pub struct PlotData {
    raw_data: Arc<Vec<f64>>,          // 共享原始数据
    processed: Rc<RefCell<Geometry>>, // 局部几何缓存
    style: Box<dyn PlotStyle>,        // 堆分配样式
}

// 写时复制优化
pub enum DataRef<'a> {
    Borrowed(&'a [f64]),
    Owned(Vec<f64>),
}
```

**GPU 端资源**:
```rust
// GPU 缓冲区管理
pub struct BufferPool {
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffers: Vec<wgpu::Buffer>,
    uniform_buffers: Vec<wgpu::Buffer>,
    free_list: Vec<usize>,
}

impl BufferPool {
    pub fn acquire(&mut self, size: u64) -> BufferHandle { /* ... */ }
    pub fn release(&mut self, handle: BufferHandle) { /* ... */ }
}
```

### 🔄 缓存策略

**多级缓存系统**:
1. **L1 缓存**: 频繁访问的几何数据 (CPU 内存)
2. **L2 缓存**: GPU 缓冲区数据 (GPU 内存)
3. **L3 缓存**: 预计算的渲染结果 (纹理缓存)

```rust
pub struct RenderCache {
    geometry_cache: LruCache<GeometryKey, Geometry>,
    texture_cache: LruCache<TextureKey, wgpu::Texture>,
    pipeline_cache: HashMap<PipelineKey, wgpu::RenderPipeline>,
}
```

---

## 🚀 性能优化架构

### ⚡ 并发处理

**多线程架构**:
```rust
// 工作线程池
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// 异步数据处理
pub async fn process_large_dataset(
    data: Vec<f64>,
    chunk_size: usize,
) -> Result<Vec<ProcessedChunk>, VizuaraError> {
    let chunks = data.chunks(chunk_size);
    let futures = chunks.map(|chunk| {
        tokio::spawn(async move {
            process_chunk(chunk).await
        })
    });
    
    try_join_all(futures).await
}
```

**GPU 并行计算**:
```rust
// 计算着色器用于数据预处理
#[repr(C)]
struct ComputeUniforms {
    data_size: u32,
    scale_factor: f32,
    offset: [f32; 2],
}

// 并行几何生成
fn generate_geometry_parallel(
    data: &[Point2D],
    style: &PlotStyle,
) -> Geometry {
    // 使用 compute shader 并行生成顶点
}
```

### 🎯 渲染优化

**批处理渲染**:
```rust
pub struct RenderBatch {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    instance_data: Vec<InstanceData>,
    material: MaterialId,
}

impl RenderBatch {
    pub fn add_geometry(&mut self, geometry: &Geometry) { /* ... */ }
    pub fn render(&self, renderer: &mut dyn Renderer) { /* ... */ }
}
```

**层次细节 (LOD)**:
```rust
pub struct LodGeometry {
    high_detail: Geometry,    // < 1K 顶点
    medium_detail: Geometry,  // < 10K 顶点
    low_detail: Geometry,     // < 100K 顶点
}

impl LodGeometry {
    pub fn select_lod(&self, distance: f64, screen_size: f64) -> &Geometry {
        if distance < 10.0 && screen_size > 0.1 {
            &self.high_detail
        } else if distance < 100.0 && screen_size > 0.01 {
            &self.medium_detail
        } else {
            &self.low_detail
        }
    }
}
```

---

## 🔌 扩展性设计

### 🧩 插件系统

```rust
// 插件接口定义
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn init(&mut self, context: &mut PluginContext) -> Result<(), PluginError>;
    fn update(&mut self, delta_time: f64);
    fn cleanup(&mut self);
}

// 插件管理器
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    context: PluginContext,
}

impl PluginManager {
    pub fn load_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> Result<(), PluginError> {
        let mut plugin = Box::new(plugin);
        plugin.init(&mut self.context)?;
        self.plugins.push(plugin);
        Ok(())
    }
}
```

### 🎨 自定义渲染器

```rust
// 渲染器扩展接口
pub trait CustomRenderer: Renderer {
    fn supports_feature(&self, feature: RenderFeature) -> bool;
    fn create_custom_pipeline(&mut self, desc: &PipelineDescriptor) -> Result<PipelineId, VizuaraError>;
}

// 自定义图表类型
pub trait CustomPlot: Plot + Renderable + Themeable {
    fn plot_type(&self) -> &'static str;
    fn register_with_engine(engine: &mut PlotEngine);
}
```

---

## 🔐 安全性考虑

### 🛡️ 内存安全

**Rust 语言级别保证**:
- 所有权系统防止悬垂指针
- 借用检查器防止数据竞争
- 类型系统防止空指针解引用

**额外安全措施**:
```rust
// 输入验证
pub fn validate_plot_data(data: &[f64]) -> Result<(), ValidationError> {
    if data.is_empty() {
        return Err(ValidationError::EmptyData);
    }
    
    for &value in data {
        if !value.is_finite() {
            return Err(ValidationError::InvalidValue(value));
        }
    }
    
    Ok(())
}

// 资源限制
pub struct ResourceLimits {
    max_vertices: usize,
    max_textures: usize,
    max_memory: usize,
}

impl ResourceLimits {
    pub fn check_vertex_count(&self, count: usize) -> Result<(), ResourceError> {
        if count > self.max_vertices {
            Err(ResourceError::TooManyVertices { count, max: self.max_vertices })
        } else {
            Ok(())
        }
    }
}
```

### 🔒 GPU 资源安全

```rust
// GPU 资源RAII管理
pub struct GpuBuffer {
    buffer: wgpu::Buffer,
    size: u64,
    usage: wgpu::BufferUsages,
}

impl Drop for GpuBuffer {
    fn drop(&mut self) {
        // 自动清理 GPU 资源
        self.buffer.destroy();
    }
}

// 安全的着色器加载
pub fn load_shader_safe(
    device: &wgpu::Device,
    source: &str,
) -> Result<wgpu::ShaderModule, ShaderError> {
    // 验证着色器代码
    validate_shader_source(source)?;
    
    // 创建着色器模块
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Safe Shader"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    
    Ok(module)
}
```

---

## 📊 监控与诊断

### 📈 性能监控

```rust
// 性能计数器
pub struct PerformanceCounters {
    frame_time: MovingAverage<f64>,
    render_time: MovingAverage<f64>,
    gpu_memory_usage: AtomicU64,
    cpu_memory_usage: AtomicU64,
    draw_calls: AtomicU32,
}

impl PerformanceCounters {
    pub fn record_frame_time(&self, time: f64) {
        self.frame_time.add_sample(time);
    }
    
    pub fn get_fps(&self) -> f64 {
        1.0 / self.frame_time.average()
    }
}

// 性能分析器
pub struct Profiler {
    sections: HashMap<String, ProfileSection>,
    current_frame: u64,
}

impl Profiler {
    pub fn begin_section(&mut self, name: &str) {
        let section = self.sections.entry(name.to_string())
            .or_insert_with(ProfileSection::new);
        section.begin();
    }
    
    pub fn end_section(&mut self, name: &str) {
        if let Some(section) = self.sections.get_mut(name) {
            section.end();
        }
    }
}
```

### 🐛 调试支持

```rust
// 调试渲染器
#[cfg(debug_assertions)]
pub struct DebugRenderer {
    base_renderer: Box<dyn Renderer>,
    debug_lines: Vec<DebugLine>,
    debug_texts: Vec<DebugText>,
}

impl DebugRenderer {
    pub fn draw_debug_line(&mut self, start: Point2D, end: Point2D, color: Color) {
        self.debug_lines.push(DebugLine { start, end, color });
    }
    
    pub fn draw_debug_text(&mut self, text: &str, pos: Point2D, color: Color) {
        self.debug_texts.push(DebugText {
            text: text.to_string(),
            position: pos,
            color,
        });
    }
}

// 错误报告
pub struct ErrorReporter {
    errors: Vec<ErrorReport>,
    max_errors: usize,
}

pub struct ErrorReport {
    error: VizuaraError,
    timestamp: std::time::Instant,
    context: String,
    stack_trace: Option<String>,
}
```

---

## 🔮 未来架构演进

### 🌐 分布式架构

**云端渲染支持**:
```rust
// 分布式渲染接口
pub trait DistributedRenderer {
    async fn render_remote(
        &self,
        scene: &Scene,
        target: RenderTarget,
    ) -> Result<RenderResult, DistributedError>;
    
    fn get_available_nodes(&self) -> Vec<RenderNode>;
}

// 渲染节点抽象
pub struct RenderNode {
    id: NodeId,
    capabilities: RenderCapabilities,
    load: f32,
    latency: Duration,
}
```

### 🤖 AI 集成

**智能可视化**:
```rust
// AI 辅助的可视化推荐
pub trait VisualizationAI {
    fn suggest_chart_type(&self, data: &DataSet) -> Vec<ChartSuggestion>;
    fn optimize_layout(&self, scene: &Scene) -> LayoutOptimization;
    fn detect_patterns(&self, data: &DataSet) -> Vec<Pattern>;
}

// 自动化图表生成
pub struct AutoChartGenerator {
    ai_engine: Box<dyn VisualizationAI>,
    template_library: TemplateLibrary,
}
```

### 📱 多端统一

**跨端架构**:
```rust
// 平台抽象层
pub trait Platform {
    type Window: PlatformWindow;
    type Renderer: PlatformRenderer;
    type Input: PlatformInput;
    
    fn create_window(&self, config: WindowConfig) -> Result<Self::Window, PlatformError>;
    fn create_renderer(&self, window: &Self::Window) -> Result<Self::Renderer, PlatformError>;
}

// 具体平台实现
pub struct DesktopPlatform;
pub struct WebPlatform;
pub struct MobilePlatform;
```

---

## 📚 架构文档维护

### 🔄 版本控制

**架构决策记录 (ADR)**:
- 每个重要架构决策都有对应的 ADR 文档
- 记录决策背景、考虑的选项、最终决策和后果
- 定期回顾和更新过时的决策

**API 稳定性保证**:
- 公共 API 使用语义化版本控制
- 破坏性变更需要详细的迁移指南
- 提供弃用周期来平滑过渡

### 📖 文档同步

**自动化文档生成**:
- 代码注释自动生成 API 文档
- 架构图表与代码同步更新
- 示例代码自动测试确保有效性

---

*本架构文档是活文档，随着项目发展持续更新。*

**维护者**: Vizuara 开发团队  
**更新频率**: 重大架构变更时  
**反馈渠道**: GitHub Issues / 架构讨论区
