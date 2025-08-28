//! Vizuara 图表类型库
//!
//! 提供各种图表类型的实现

pub mod area;
pub mod bar;
pub mod boxplot;
pub mod contour;
pub mod density;
pub mod heatmap;
pub mod histogram;
pub mod line;
pub mod parallel;
pub mod pie;
pub mod radar;
pub mod sankey;
pub mod scatter;
pub mod treemap;
pub mod violin;

pub use area::*;
pub use bar::*;
pub use boxplot::*;
pub use contour::*;
pub use density::*;
pub use heatmap::*;
pub use histogram::*;
pub use line::*;
pub use parallel::*;
pub use pie::*;
pub use radar::*;
pub use sankey::*;
pub use scatter::*;
pub use treemap::*;
pub use violin::*;
