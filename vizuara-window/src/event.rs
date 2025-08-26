/// 应用程序事件类型
#[derive(Debug, Clone)]
pub enum VizuaraEvent {
    /// 窗口关闭请求
    CloseRequested,
    /// 窗口大小改变
    Resized { width: u32, height: u32 },
    /// 鼠标移动
    MouseMoved { x: f64, y: f64 },
    /// 鼠标按钮按下
    MousePressed { button: MouseButton, x: f64, y: f64 },
    /// 鼠标按钮释放
    MouseReleased { button: MouseButton, x: f64, y: f64 },
    /// 滚轮滚动
    Scroll { delta_x: f64, delta_y: f64 },
    /// 键盘按键
    KeyPressed { key: String },
}

/// 鼠标按钮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// 事件处理器trait
pub trait EventHandler {
    /// 处理事件
    fn handle_event(&mut self, event: VizuaraEvent);
}
