//! Vizuara 核心库
//!
//! 提供可视化的基础数据结构和抽象接口

pub mod coords;
pub mod error;
pub mod primitive;
pub mod scale;
pub mod style;

pub use coords::*;
pub use error::*;
pub use primitive::*;
pub use scale::*;
pub use style::*;

/// 核心版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
