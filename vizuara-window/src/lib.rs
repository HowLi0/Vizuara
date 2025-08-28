//! 窗口管理模块
//!
//! 提供跨平台窗口创建和事件处理

pub mod event;
pub mod figure_window;
pub mod window;
pub mod window_3d;

pub use event::*;
pub use figure_window::FigureWindow;
pub use window::VizuaraWindow;
pub use window_3d::Window3D;

use vizuara_scene::Figure;

/// 显示图形的便捷函数
pub fn show_figure(figure: Figure) -> Result<(), Box<dyn std::error::Error>> {
    let window = FigureWindow::new("Vizuara".to_string(), 800, 600)?;
    window.show_figure(figure)?;
    Ok(())
}
