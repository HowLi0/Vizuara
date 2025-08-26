//! Vizuara 核心库
//! 
//! 提供可视化的基础数据结构和抽象接口

pub mod primitive;
pub mod style;
pub mod coords;
pub mod scale;
pub mod error;

pub use primitive::*;
pub use style::*;
pub use coords::*;
pub use scale::*;
pub use error::*;

/// 核心版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
