# Vizuara 贡献指南

欢迎来到 **Vizuara** 项目！我们很高兴您有兴趣为这个 Rust 科学可视化库做出贡献。

## 📖 快速开始

### 🛠️ 开发环境设置

#### 系统要求
- **Rust**: 稳定版本 (1.70+)
- **操作系统**: Linux, macOS, Windows
- **GPU**: 支持 Vulkan, DirectX 12, 或 Metal 的现代显卡

#### 环境配置
```bash
# 1. 克隆仓库
git clone https://github.com/HowLi0/Vizuara.git
cd Vizuara

# 2. 安装 Rust 工具链
rustup update stable
rustup component add rustfmt clippy

# 3. 验证环境
cargo check --workspace --all-targets
cargo test --workspace

# 4. 运行示例（验证 GPU 支持）
cargo run -p vizuara-examples --bin interactive_demo
```

#### 推荐开发工具
- **IDE**: VS Code + rust-analyzer
- **调试**: LLDB/GDB + Rust LLDB 扩展
- **性能分析**: `cargo flamegraph`, `perf`
- **GPU 调试**: RenderDoc, Nsight Graphics

---

## 🎯 贡献方式

### 🐛 报告 Bug
发现问题？请帮助我们改进！

**Bug 报告清单**:
- [ ] 搜索现有 Issues 确认未重复
- [ ] 使用 Bug 报告模板
- [ ] 提供完整的环境信息
- [ ] 包含最小可复现示例
- [ ] 附上错误日志和截图

**Bug 报告模板**:
```markdown
## Bug 描述
简短描述问题的现象

## 复现步骤
1. 执行操作 A
2. 执行操作 B
3. 观察到错误现象

## 期望行为
描述正确的行为应该是什么

## 环境信息
- OS: [e.g. Ubuntu 22.04]
- Rust: [e.g. 1.75.0]
- GPU: [e.g. NVIDIA RTX 3060]
- Vizuara: [e.g. 0.3.1-alpha]

## 日志输出
```
在此粘贴相关日志
```

## 截图
如果适用，请添加截图
```

### 💡 功能请求
有好想法？我们很想听到！

**功能请求指南**:
- [ ] 描述具体的使用场景
- [ ] 解释当前方案的不足
- [ ] 提供设计草图或伪代码
- [ ] 考虑向后兼容性
- [ ] 评估实现复杂度

### 🔧 代码贡献
准备提交代码？太棒了！

#### 贡献流程
1. **Fork 仓库** 并创建特性分支
2. **实现功能** 或修复 Bug
3. **编写测试** 确保代码质量
4. **更新文档** 保持文档同步
5. **提交 PR** 并描述变更内容
6. **代码审查** 响应反馈并改进
7. **合并完成** 庆祝贡献成功！

#### 分支命名规范
```bash
# 功能开发
feat/add-scatter-plot-animation

# Bug 修复
fix/resolve-memory-leak-in-renderer

# 文档改进
docs/update-api-examples

# 性能优化
perf/optimize-large-dataset-rendering

# 重构
refactor/simplify-theme-system
```

---

## 📝 代码规范

### 🎨 代码风格

#### Rust 代码规范
```rust
// ✅ 推荐：清晰的命名和文档
/// 创建新的散点图实例
/// 
/// # 参数
/// * `data` - 数据点集合
/// * `style` - 绘制样式配置
/// 
/// # 示例
/// ```rust
/// let plot = ScatterPlot::new(data, style)?;
/// ```
pub fn create_scatter_plot(
    data: &[DataPoint], 
    style: PlotStyle,
) -> Result<ScatterPlot, VizuaraError> {
    // 实现细节
}

// ❌ 避免：模糊的命名和缺失文档
pub fn create(d: &[DP], s: PS) -> Result<SP, VE> {
    // 无注释的实现
}
```

#### 错误处理
```rust
// ✅ 使用 thiserror 定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum VizuaraError {
    #[error("Invalid data format: {0}")]
    InvalidData(String),
    
    #[error("Rendering failed: {reason}")]
    RenderError { reason: String },
    
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

// ✅ 返回具体的错误信息
fn validate_data(data: &[f64]) -> Result<(), VizuaraError> {
    if data.is_empty() {
        return Err(VizuaraError::InvalidData(
            "Data cannot be empty".to_string()
        ));
    }
    Ok(())
}
```

#### 性能优化指南
```rust
// ✅ 避免不必要的克隆
fn process_data(data: &[Point]) -> Vec<ProcessedPoint> {
    data.iter()
        .map(|point| process_point(point))  // 借用而非克隆
        .collect()
}

// ✅ 使用迭代器链式操作
fn filter_and_transform(data: &[DataPoint]) -> Vec<DisplayPoint> {
    data.iter()
        .filter(|point| point.is_valid())
        .map(|point| point.to_display())
        .collect()
}

// ✅ 预分配容器大小
fn collect_results(size_hint: usize) -> Vec<Result> {
    let mut results = Vec::with_capacity(size_hint);
    // 填充数据...
    results
}
```

### 🧪 测试规范

#### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scatter_plot_creation() {
        let data = vec![
            DataPoint::new(1.0, 2.0),
            DataPoint::new(3.0, 4.0),
        ];
        let style = PlotStyle::default();
        
        let plot = ScatterPlot::new(&data, style);
        
        assert!(plot.is_ok());
        assert_eq!(plot.unwrap().point_count(), 2);
    }
    
    #[test]
    fn test_empty_data_error() {
        let data = vec![];
        let style = PlotStyle::default();
        
        let result = ScatterPlot::new(&data, style);
        
        assert!(result.is_err());
        assert!(matches!(result, Err(VizuaraError::InvalidData(_))));
    }
}
```

#### 集成测试
```rust
// tests/integration_test.rs
use vizuara_plots::*;
use vizuara_themes::*;

#[test]
fn test_plot_with_theme_integration() {
    let data = generate_test_data(100);
    let theme = Theme::dark();
    
    let mut plot = ScatterPlot::new(&data, PlotStyle::default())
        .expect("Failed to create plot");
    
    plot.apply_theme(&theme);
    
    // 验证主题应用效果
    assert_eq!(plot.background_color(), theme.background);
}
```

#### 性能测试
```rust
// benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vizuara_plots::ScatterPlot;

fn benchmark_large_dataset(c: &mut Criterion) {
    let data = generate_large_dataset(1_000_000);
    
    c.bench_function("scatter_plot_1m_points", |b| {
        b.iter(|| {
            let plot = ScatterPlot::new(black_box(&data), PlotStyle::default());
            black_box(plot)
        })
    });
}

criterion_group!(benches, benchmark_large_dataset);
criterion_main!(benches);
```

---

## 📚 文档规范

### 📖 API 文档
每个公开的函数、结构体和枚举都必须有文档注释：

```rust
/// 表示二维坐标点的结构体
/// 
/// `Point2D` 是所有 2D 可视化的基础数据类型，支持
/// 各种数学运算和坐标变换。
/// 
/// # 示例
/// 
/// ```rust
/// use vizuara_core::Point2D;
/// 
/// let point = Point2D::new(3.14, 2.71);
/// assert_eq!(point.x(), 3.14);
/// assert_eq!(point.y(), 2.71);
/// 
/// let moved = point.translate(1.0, 1.0);
/// assert_eq!(moved.x(), 4.14);
/// ```
/// 
/// # 坐标系统
/// 
/// Vizuara 使用标准的笛卡尔坐标系统：
/// - X 轴：从左到右递增
/// - Y 轴：从下到上递增
/// - 原点：(0, 0) 位于左下角
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    /// 创建新的 2D 点
    /// 
    /// # 参数
    /// 
    /// * `x` - X 坐标值
    /// * `y` - Y 坐标值
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// let origin = Point2D::new(0.0, 0.0);
    /// let point = Point2D::new(-1.5, 3.7);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
```

### 📋 代码示例指南
- **完整性**: 示例应该可以直接运行
- **实用性**: 展示常见的使用场景
- **简洁性**: 专注于核心功能，避免不相关的复杂性
- **测试性**: 所有文档示例都会在 CI 中测试

---

## 🔍 代码审查指南

### 👀 审查者指南
作为代码审查者，请关注：

#### 功能正确性
- [ ] 代码逻辑是否正确？
- [ ] 边界条件是否处理？
- [ ] 错误处理是否合适？
- [ ] 测试覆盖是否充分？

#### 代码质量
- [ ] 命名是否清晰？
- [ ] 结构是否合理？
- [ ] 注释是否有用？
- [ ] 性能是否考虑？

#### 架构一致性
- [ ] 是否符合项目架构？
- [ ] API 设计是否一致？
- [ ] 依赖关系是否合理？
- [ ] 向后兼容性如何？

### 📝 提交者指南
提交 PR 时，请确保：

#### PR 描述模板
```markdown
## 变更概述
简短描述这个 PR 的主要变更

## 变更类型
- [ ] Bug 修复
- [ ] 新功能
- [ ] 性能优化
- [ ] 重构
- [ ] 文档更新

## 测试说明
- [ ] 添加了新的测试
- [ ] 所有现有测试通过
- [ ] 手动测试已完成

## 影响评估
- [ ] API 无破坏性变更
- [ ] 性能无显著影响
- [ ] 文档已更新

## 相关 Issue
解决 #123
相关 #456

## 截图/演示
如果适用，请添加截图或 GIF
```

#### 提交信息规范
```bash
# 格式：<类型>(<范围>): <描述>

# 示例
feat(plots): add animation support for scatter plots
fix(renderer): resolve memory leak in GPU buffer management
docs(api): update ScatterPlot documentation with examples
perf(core): optimize coordinate transformation for large datasets
refactor(themes): simplify theme application logic

# 类型说明
feat:     新功能
fix:      Bug 修复
docs:     文档变更
style:    代码格式化（不影响逻辑）
refactor: 重构
perf:     性能优化
test:     测试相关
chore:    构建过程或辅助工具变更
```

---

## 🎯 特定领域贡献指南

### 🎨 图表系统贡献

#### 添加新图表类型
```rust
// 1. 定义数据结构
pub struct RadarChart {
    data: Vec<RadarData>,
    style: RadarStyle,
}

// 2. 实现核心 trait
impl Renderable for RadarChart {
    fn render(&self, renderer: &mut dyn Renderer) -> Result<(), VizuaraError> {
        // 渲染实现
    }
}

impl Themeable for RadarChart {
    fn apply_theme(&mut self, theme: &Theme) {
        // 主题应用
    }
}

// 3. 添加构建器模式
impl RadarChart {
    pub fn builder() -> RadarChartBuilder {
        RadarChartBuilder::new()
    }
}

// 4. 编写完整测试
#[cfg(test)]
mod tests {
    // 测试实现
}
```

### 🎮 交互系统贡献

#### 添加新交互工具
```rust
// 1. 定义工具结构
pub struct SelectionTool {
    mode: SelectionMode,
    selection: Vec<DataIndex>,
}

// 2. 实现工具 trait
impl InteractiveTool for SelectionTool {
    fn handle_event(&mut self, event: &InputEvent) -> ToolResponse {
        match event {
            InputEvent::MouseDown(pos) => {
                // 处理鼠标按下
            }
            // 其他事件处理
        }
    }
}

// 3. 集成到工具系统
// 在 ToolManager 中注册新工具
```

### 🎭 主题系统贡献

#### 创建新主题
```rust
// 1. 定义主题配置
pub fn create_neon_theme() -> Theme {
    Theme::builder()
        .name("Neon")
        .background(Color::BLACK)
        .primary(Color::CYAN)
        .secondary(Color::MAGENTA)
        .accent(Color::YELLOW)
        .build()
}

// 2. 添加预设
impl ThemePresets {
    pub fn neon() -> Theme {
        create_neon_theme()
    }
}

// 3. 测试主题效果
#[test]
fn test_neon_theme() {
    let theme = ThemePresets::neon();
    assert_eq!(theme.background(), Color::BLACK);
    // 更多测试...
}
```

---

## 🚀 高级贡献主题

### 🔧 性能优化
- **GPU 优化**: 着色器优化、批处理渲染
- **内存管理**: 缓冲池、智能缓存
- **并发处理**: 多线程数据处理
- **算法优化**: 空间索引、层次细节

### 🌐 平台扩展
- **Web 支持**: WebAssembly 移植
- **移动端**: iOS/Android 适配
- **嵌入式**: 资源受限环境优化

### 📊 数据科学集成
- **统计图表**: 高级统计可视化
- **机器学习**: 模型可视化工具
- **大数据**: 流式数据处理
- **数据源**: 新的数据格式支持

---

## 🤝 社区参与

### 💬 沟通渠道
- **GitHub Discussions**: 技术讨论和问答
- **GitHub Issues**: Bug 报告和功能请求
- **Discord/Telegram**: 实时交流（筹备中）
- **邮件列表**: 重要公告（筹备中）

### 📅 社区活动
- **每月技术分享**: 展示最新进展
- **季度路线图审查**: 社区意见收集
- **年度开发者大会**: 深度技术交流

### 🎓 学习资源
- **官方文档**: 完整的 API 和教程
- **示例仓库**: 实际应用案例
- **技术博客**: 深度技术文章
- **视频教程**: 视觉化学习资源

---

## 🏆 贡献者认可

### 📈 贡献等级
- **新手贡献者**: 首次 PR 合并
- **活跃贡献者**: 10+ PR 合并
- **核心贡献者**: 重要功能贡献
- **项目维护者**: 长期稳定贡献

### 🎖️ 特殊认可
- **文档英雄**: 文档贡献突出
- **性能专家**: 性能优化贡献
- **设计大师**: UI/UX 设计贡献
- **测试卫士**: 测试和质量贡献

### 🎁 贡献奖励
- **开源徽章**: GitHub 个人资料展示
- **项目感谢**: README 和官网致谢
- **会议机会**: 技术会议演讲邀请
- **职业推荐**: 技能认证和推荐

---

## ❓ 常见问题

### 🤔 技术问题

**Q: 如何调试 GPU 渲染问题？**
A: 推荐使用 RenderDoc 捕获帧，配合 `RUST_LOG=debug` 环境变量查看详细日志。

**Q: 如何测试跨平台兼容性？**
A: 我们的 CI 会自动测试多平台，本地可以使用 Docker 或虚拟机测试。

**Q: 如何优化大数据集的性能？**
A: 考虑数据分片、LOD 技术和 GPU 实例化渲染。参考 `examples/large_dataset_demo.rs`。

### 🛠️ 开发流程

**Q: PR 审查需要多长时间？**
A: 通常 1-3 个工作日，复杂 PR 可能需要更长时间。

**Q: 如何同步最新的 master 分支？**
```bash
git checkout master
git pull upstream master
git checkout your-feature-branch
git rebase master
```

**Q: 测试失败怎么办？**
A: 首先本地运行 `cargo test --workspace`，查看具体错误信息。如果是 CI 环境问题，请在 PR 中说明。

---

## 📞 联系方式

### 👨‍💻 项目维护者
- **主要维护者**: @HowLi0
- **响应时间**: 24-48 小时
- **联系方式**: GitHub Issues 或 Discussions

### 🆘 获取帮助
1. **查阅文档**: 先查看官方文档和 FAQ
2. **搜索历史**: 在 Issues 中搜索相似问题
3. **提问格式**: 使用问题模板，提供完整信息
4. **耐心等待**: 维护者会尽快回复

---

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。通过贡献代码，您同意您的贡献将在相同许可证下发布。

---

**感谢您的贡献！每一个贡献都让 Vizuara 变得更好。** 🚀

*最后更新: 2025年8月28日*
