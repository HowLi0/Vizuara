//! 基础三角形渲染示例
//! 
//! 这是 Vizuara 的第一个示例，验证基础渲染功能

use vizuara_window::VizuaraWindow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 启动 Vizuara 基础示例...");
    
    // 创建窗口和渲染器
    let window = VizuaraWindow::new().await?;
    
    println!("✅ 窗口创建成功，开始事件循环");
    
    // 运行事件循环
    window.run().await?;
    
    Ok(())
}
