use vizuara_core::Color;
use vizuara_themes::{
    ThemeBuilder, ComponentThemeBuilder, PaletteBuilder, 
    ThemeManager, ThemePresets, ComponentType
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Vizuara 主题系统示例");
    println!("═══════════════════════════════\n");

    // 1. 使用预设主题
    println!("1. 📋 加载预设主题");
    println!("─────────────────────");
    
    let dark_theme = ThemePresets::get_preset("dark").unwrap();
    println!("✓ 加载暗黑主题: {}", dark_theme.name);
    println!("  描述: {}", dark_theme.description);
    println!("  版本: {}", dark_theme.version);
    
    let scientific_theme = ThemePresets::get_preset("scientific").unwrap();
    println!("✓ 加载科学主题: {}", scientific_theme.name);
    
    // 列出所有预设主题
    let preset_names = ThemePresets::list_preset_names();
    println!("  可用预设主题: {:?}\n", preset_names);

    // 2. 创建自定义调色板
    println!("2. 🎨 创建自定义调色板");
    println!("─────────────────────");
    
    let ocean_palette = PaletteBuilder::new("ocean")
        .description("海洋风格调色板")
        .primary(Color::rgb(0.0, 0.4, 0.8))
        .secondary(Color::rgb(0.2, 0.6, 0.9))
        .accent(Color::rgb(0.0, 0.8, 0.6))
        .background(Color::rgb(0.95, 0.98, 1.0))
        .surface(Color::rgb(0.9, 0.95, 0.98))
        .text(Color::rgb(0.1, 0.2, 0.3))
        .generate_series_hsv(8, 0.7, 0.8)
        .build();
    
    println!("✓ 创建海洋调色板: {}", ocean_palette.name);
    println!("  主色: RGB({:.2}, {:.2}, {:.2})", 
             ocean_palette.primary.r, ocean_palette.primary.g, ocean_palette.primary.b);
    println!("  系列颜色数量: {}\n", ocean_palette.series.len());

    // 3. 使用构建器创建复杂主题
    println!("3. 🏗️ 使用构建器创建自定义主题");
    println!("──────────────────────────────");
    
    let custom_theme = ThemeBuilder::new("ocean_analytics")
        .description("基于海洋色彩的数据分析主题")
        .version("1.0.0")
        .author("Vizuara 开发团队")
        .palette(ocean_palette)
        .font_size(12.0)
        .line_width(1.5)
        .opacity(0.9)
        // 配置散点图
        .scatter_plot(|builder| {
            builder
                .primary_color(Color::rgb(0.2, 0.5, 0.8))
                .point_size(4.0)
                .hover_color(Color::rgb(0.4, 0.7, 1.0))
                .active_color(Color::rgb(0.1, 0.3, 0.6))
                .opacity(0.8)
        })
        // 配置折线图
        .line_plot(|builder| {
            builder
                .primary_color(Color::rgb(0.0, 0.6, 0.4))
                .line_width(2.5)
                .border_color(Color::rgb(0.0, 0.4, 0.3))
        })
        // 配置坐标轴
        .axis(|builder| {
            builder
                .primary_color(Color::rgb(0.3, 0.3, 0.3))
                .font_size(10.0)
                .line_width(1.0)
        })
        // 配置网格
        .grid(|builder| {
            builder
                .primary_color(Color::rgb(0.8, 0.9, 0.95))
                .line_width(0.5)
                .opacity(0.6)
        })
        .custom_property("animation_duration", vizuara_themes::ThemeValue::Number(500.0))
        .custom_property("show_tooltips", vizuara_themes::ThemeValue::Boolean(true))
        .build()?;
    
    println!("✓ 创建自定义主题: {}", custom_theme.name);
    println!("  组件数量: {}", custom_theme.components.len());
    println!("  自定义属性数量: {}", custom_theme.custom.len());

    // 4. 注册并管理主题
    println!("\n4. 📝 主题管理");
    println!("─────────────────");
    
    let manager = ThemeManager::instance();
    
    // 注册预设主题
    manager.register_theme(dark_theme)?;
    manager.register_theme(scientific_theme)?;
    manager.register_theme(custom_theme)?;
    
    println!("✓ 注册了 {} 个主题", manager.list_themes().len());
    
    // 切换主题
    manager.switch_theme("ocean_analytics")?;
    let current_theme = manager.current_theme();
    println!("✓ 切换到主题: {}", current_theme.name);
    
    // 获取当前主题的一些属性
    println!("  当前主题描述: {}", current_theme.description);
        
        if let Some(scatter_component) = current_theme.get_component(&ComponentType::ScatterPlot) {
            if let Some(point_size) = scatter_component.get_number(&vizuara_themes::ThemeProperty::PointSize) {
                println!("  散点图点大小: {}", point_size);
            }
        }

    // 5. 主题导出和导入
    println!("\n5. 💾 主题持久化");
    println!("─────────────────");
    
    // 保存主题到文件
    let theme_file = "/tmp/ocean_analytics.toml";
    let ocean_theme = manager.get_theme("ocean_analytics").unwrap();
    manager.save_theme_to_file(&ocean_theme, theme_file)?;
    println!("✓ 主题已保存到: {}", theme_file);
    
    // 从文件加载主题
    let loaded_theme = manager.load_theme_from_file(theme_file)?;
    println!("✓ 从文件加载主题: {}", loaded_theme.name);
    
    // 验证加载的主题
    assert_eq!(loaded_theme.name, "ocean_analytics");
    assert_eq!(loaded_theme.description, "基于海洋色彩的数据分析主题");

    // 6. 主题修改和继承
    println!("\n6. 🔄 主题修改和继承");
    println!("──────────────────────");
    
    let modified_theme = ThemeBuilder::from_preset("dark")
        .unwrap()
        .description("修改后的暗黑主题")
        .primary_color(Color::rgb(0.8, 0.2, 0.4))
        .parent("dark".to_string())
        .scatter_plot(|builder| {
            builder
                .primary_color(Color::rgb(1.0, 0.3, 0.5))
                .point_size(5.0)
        })
        .build()?;
    
    manager.register_theme(modified_theme)?;
    println!("✓ 创建并注册了修改版暗黑主题");

    // 7. 统计信息
    println!("\n7. 📊 主题统计");
    println!("─────────────────");
    
    let theme_count = manager.list_themes().len();
    let current_theme = manager.current_theme();
    println!("✓ 总主题数: {}", theme_count);
    println!("✓ 当前主题: {}", current_theme.name);

    // 8. 应用主题到组件
    println!("\n8. 🎭 主题应用示例");
    println!("─────────────────────");
    
    manager.switch_theme("ocean_analytics")?;
    let current = manager.current_theme();
    
    // 模拟应用主题到散点图组件
    println!("应用主题到散点图:");
    
    // 获取全局样式
    if let Some(primary) = current.get_global(&vizuara_themes::ThemeProperty::PrimaryColor) {
        if let vizuara_themes::ThemeValue::Color(color) = primary {
            println!("  全局主色: RGB({:.2}, {:.2}, {:.2})", color.r, color.g, color.b);
        }
    }
    
    // 获取散点图特定样式
    if let Some(scatter_style) = current.get_component(&ComponentType::ScatterPlot) {
        if let Some(point_color) = scatter_style.get_color(&vizuara_themes::ThemeProperty::PrimaryColor) {
            println!("  散点颜色: RGB({:.2}, {:.2}, {:.2})", point_color.r, point_color.g, point_color.b);
        }
        if let Some(point_size) = scatter_style.get_number(&vizuara_themes::ThemeProperty::PointSize) {
            println!("  点大小: {}", point_size);
        }
    }

    println!("\n🎉 主题系统示例完成!");
    println!("════════════════════════");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_system_integration() {
        // 测试完整的主题系统集成
        let manager = ThemeManager::instance();
        
        // 创建测试主题
        let test_theme = ThemeBuilder::new("integration_test")
            .description("集成测试主题")
            .primary_color(Color::rgb(1.0, 0.0, 0.0))
            .scatter_plot(|builder| {
                builder.point_size(3.0)
            })
            .build()
            .unwrap();
        
        // 注册主题
        manager.register_theme(test_theme).unwrap();
        
        // 切换主题
        manager.switch_theme("integration_test").unwrap();
        
        // 验证主题
        let current = manager.current_theme();
        assert_eq!(current.name, "integration_test");
        
        let current = manager.current_theme();
        assert_eq!(
            current.get_global(&vizuara_themes::ThemeProperty::PrimaryColor),
            Some(&vizuara_themes::ThemeValue::Color(Color::rgb(1.0, 0.0, 0.0)))
        );
        
        let scatter = current.get_component(&ComponentType::ScatterPlot).unwrap();
        assert_eq!(
            scatter.get_number(&vizuara_themes::ThemeProperty::PointSize),
            Some(3.0)
        );
    }
}
