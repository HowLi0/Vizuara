//! 基础三角形渲染示例
//! 
//! 这是 Vizuara 的第一个示例，验证基础渲染功能

use tokio;
use vizuara_window::VizuaraWindow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 启动基本三角形示例...");
    
    // 创建并运行窗口，内置三个点的渲染示例
    VizuaraWindow::create_and_run().await?;
    
    Ok(())
}
