//! 桑基图演示
//!
//! 展示数据流向可视化功能

use vizuara_plots::{SankeyDiagram, SankeyNode, SankeyLink, PlotArea};
use vizuara_scene::{Figure, Scene};
use vizuara_window::show_figure;
use vizuara_core::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 桑基图演示 - Data Flow Visualization");

    // 创建能源流向桑基图
    let mut sankey = SankeyDiagram::new()
        .title("能源消耗流向图");

    // 添加节点
    let coal = SankeyNode::new("煤炭".to_string(), Color::rgb(0.4, 0.2, 0.1));
    let oil = SankeyNode::new("石油".to_string(), Color::rgb(0.2, 0.2, 0.2));
    let gas = SankeyNode::new("天然气".to_string(), Color::rgb(0.3, 0.4, 0.8));
    let power = SankeyNode::new("发电".to_string(), Color::rgb(0.6, 0.6, 0.6));
    let industry = SankeyNode::new("工业".to_string(), Color::rgb(0.8, 0.4, 0.2));
    let residential = SankeyNode::new("民用".to_string(), Color::rgb(0.2, 0.8, 0.2));

    sankey = sankey
        .add_node(coal)
        .add_node(oil)
        .add_node(gas)
        .add_node(power)
        .add_node(industry)
        .add_node(residential);

    // 添加连接
    let link1 = SankeyLink::new("煤炭".to_string(), "发电".to_string(), 45.0, Color::rgba(0.4, 0.2, 0.1, 0.6));
    let link2 = SankeyLink::new("石油".to_string(), "发电".to_string(), 15.0, Color::rgba(0.2, 0.2, 0.2, 0.6));
    let link3 = SankeyLink::new("天然气".to_string(), "发电".to_string(), 25.0, Color::rgba(0.3, 0.4, 0.8, 0.6));
    let link4 = SankeyLink::new("发电".to_string(), "工业".to_string(), 40.0, Color::rgba(0.8, 0.4, 0.2, 0.6));
    let link5 = SankeyLink::new("发电".to_string(), "民用".to_string(), 45.0, Color::rgba(0.2, 0.8, 0.2, 0.6));

    sankey = sankey
        .add_link(link1)
        .add_link(link2)
        .add_link(link3)
        .add_link(link4)
        .add_link(link5);

    // 设置样式
    sankey = sankey
        .node_width(25.0)
        .node_padding(50.0)
        .link_opacity(0.7)
        .show_values(true);

    let plot_area = PlotArea::new(60.0, 80.0, 680.0, 440.0);
    let scene = Scene::new(plot_area)
        .add_sankey_diagram(sankey);
    
    let figure = Figure::new(800.0, 600.0)
        .title("桑基图演示")
        .add_scene(scene);

    println!("✅ 桑基图演示图形已创建！");
    println!("这个图表展示了能源的流向分配，线条粗细代表流量大小。");
    
    show_figure(figure)?;
    Ok(())
}
