//! 窗口管理模块
//! 
//! 提供跨平台窗口创建和事件处理

pub mod window;
pub mod figure_window;

pub use window::VizuaraWindow;
pub use figure_window::{FigureWindow, show_figure};
