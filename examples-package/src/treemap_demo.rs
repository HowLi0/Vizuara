//! 树状图演示
//!
//! 展示层次数据可视化功能

use vizuara_plots::{ColorScheme, PlotArea, Treemap, TreemapItem};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌳 树状图演示 - Hierarchical Data Visualization");

    // 创建科技公司市值树状图
    let mut treemap = Treemap::new()
        .title("全球科技公司市值分布")
        .color_scheme(ColorScheme::Category);

    // 添加科技巨头
    let apple = TreemapItem::new("苹果".to_string(), 2800.0);
    let microsoft = TreemapItem::new("微软".to_string(), 2400.0);
    let google = TreemapItem::new("谷歌".to_string(), 1600.0);
    let amazon = TreemapItem::new("亚马逊".to_string(), 1400.0);
    let tesla = TreemapItem::new("特斯拉".to_string(), 800.0);
    let meta = TreemapItem::new("Meta".to_string(), 700.0);
    let nvidia = TreemapItem::new("英伟达".to_string(), 600.0);
    let netflix = TreemapItem::new("Netflix".to_string(), 200.0);
    let uber = TreemapItem::new("Uber".to_string(), 80.0);
    let twitter = TreemapItem::new("Twitter".to_string(), 40.0);

    treemap = treemap
        .add_item(apple)
        .add_item(microsoft)
        .add_item(google)
        .add_item(amazon)
        .add_item(tesla)
        .add_item(meta)
        .add_item(nvidia)
        .add_item(netflix)
        .add_item(uber)
        .add_item(twitter);

    // 设置样式
    treemap = treemap
        .border_width(2.0)
        .padding(4.0)
        .show_labels(true)
        .show_values(true);

    let plot_area = PlotArea::new(60.0, 80.0, 680.0, 440.0);
    let scene = Scene::new(plot_area).add_treemap(treemap);

    let figure = Figure::new(800.0, 600.0)
        .title("树状图演示")
        .add_scene(scene);

    println!("✅ 树状图演示图形已创建！");
    println!("这个图表展示了公司市值的层次结构，矩形大小代表相对市值。");

    show_figure(figure)?;
    Ok(())
}
