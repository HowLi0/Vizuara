use nalgebra::Point2;
use vizuara_core::{Color, Primitive, Style};
use vizuara_export::{ExportManager, ExportOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vizuara 导出功能演示");

    // 创建一些示例图形数据
    let primitives = vec![
        // 一个圆形
        Primitive::Circle {
            center: Point2::new(50.0, 50.0),
            radius: 20.0,
        },
        // 一个矩形
        Primitive::Rectangle {
            min: Point2::new(80.0, 30.0),
            max: Point2::new(120.0, 70.0),
        },
        // 一条线
        Primitive::Line {
            start: Point2::new(10.0, 10.0),
            end: Point2::new(40.0, 40.0),
        },
        // 一个点
        Primitive::Point(Point2::new(150.0, 50.0)),
    ];

    // 为每个图形设置样式
    let styles = vec![
        Style::new()
            .fill_color(Color::rgb(1.0, 0.0, 0.0))  // 红色圆形
            .stroke(Color::rgb(0.0, 0.0, 0.0), 2.0), // 黑色边框
        Style::new()
            .fill_color(Color::rgb(0.0, 1.0, 0.0))  // 绿色矩形
            .stroke(Color::rgb(0.0, 0.0, 1.0), 1.5), // 蓝色边框
        Style::new().stroke(Color::rgb(1.0, 0.5, 0.0), 3.0), // 橙色线条
        Style::new().fill_color(Color::rgb(0.5, 0.0, 1.0)),  // 紫色点
    ];

    // 设置导出选项（一次性初始化，避免默认后再赋值）
    let mut options = ExportOptions {
        background_color: Some(Color::rgb(1.0, 1.0, 1.0)),
        include_metadata: true,
        ..ExportOptions::default()
    };

    // 添加自定义属性
    options
        .custom_attributes
        .insert("title".to_string(), "Vizuara导出示例".to_string());
    options
        .custom_attributes
        .insert("author".to_string(), "Vizuara示例程序".to_string());

    // 导出为SVG格式
    println!("导出SVG文件...");
    ExportManager::export_svg(
        &primitives,
        &styles,
        200,
        100,
        "/tmp/vizuara_demo.svg",
        Some(options.clone()),
    )?;
    println!("SVG文件已保存到: /tmp/vizuara_demo.svg");

    // 导出为PNG格式
    println!("导出PNG文件...");
    ExportManager::export_png(
        &primitives,
        &styles,
        200,
        100,
        "/tmp/vizuara_demo.png",
        Some(options.clone()),
    )?;
    println!("PNG文件已保存到: /tmp/vizuara_demo.png");

    // 自动检测格式导出
    println!("自动检测格式导出SVG...");
    ExportManager::export_auto(
        &primitives,
        &styles,
        200,
        100,
        "/tmp/vizuara_auto.svg",
        Some(options.clone()),
    )?;
    println!("自动导出SVG完成: /tmp/vizuara_auto.svg");

    println!("自动检测格式导出PNG...");
    ExportManager::export_auto(
        &primitives,
        &styles,
        200,
        100,
        "/tmp/vizuara_auto.png",
        Some(options.clone()),
    )?;
    println!("自动导出PNG完成: /tmp/vizuara_auto.png");

    println!("\n导出演示完成！检查 /tmp/ 目录查看生成的文件。");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_demo_data_creation() {
        // 测试能够创建示例数据
        let primitives = [Primitive::Circle {
            center: Point2::new(50.0, 50.0),
            radius: 20.0,
        }];

        let styles = [Style::new().fill_color(Color::rgb(1.0, 0.0, 0.0))];

        assert_eq!(primitives.len(), 1);
        assert_eq!(styles.len(), 1);
    }

    #[test]
    fn test_export_operations() -> Result<(), Box<dyn std::error::Error>> {
        let primitives = [Primitive::Circle {
            center: Point2::new(25.0, 25.0),
            radius: 10.0,
        }];

        let styles = [Style::new().fill_color(Color::rgb(0.0, 1.0, 0.0))];

        // 创建临时文件
        let temp_svg = "/tmp/test_export.svg";
        let temp_png = "/tmp/test_export.png";

        // 测试SVG导出
        ExportManager::export_svg(&primitives, &styles, 50, 50, temp_svg, None)?;

        // 验证文件被创建
        assert!(std::path::Path::new(temp_svg).exists());

        // 测试PNG导出
        ExportManager::export_png(&primitives, &styles, 50, 50, temp_png, None)?;

        // 验证文件被创建
        assert!(std::path::Path::new(temp_png).exists());

        // 清理临时文件
        let _ = std::fs::remove_file(temp_svg);
        let _ = std::fs::remove_file(temp_png);

        Ok(())
    }
}
