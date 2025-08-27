//! 
//! Vizuara 动画系统
//! 
//! 提供高性能的动画功能，支持：
//! - 缓动函数 (Easing functions)
//! - 关键帧动画 (Keyframe animations)  
//! - 平滑过渡 (Smooth transitions)
//! - 动画组合 (Animation composition)
//! - 时间轴管理 (Timeline management)
//!

pub mod timeline;
pub mod transition;
pub mod easing;
pub mod keyframe;

pub use timeline::*;
pub use transition::*;
pub use easing::*;
pub use keyframe::*;

use std::time::Duration;

/// 动画状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    /// 未开始
    NotStarted,
    /// 播放中
    Playing,
    /// 暂停
    Paused,
    /// 已完成
    Completed,
}

/// 动画系统的基础trait
pub trait Animatable {
    /// 动画数据类型
    type Value;
    
    /// 获取当前值
    fn current_value(&self) -> Self::Value;
    
    /// 设置当前值
    fn set_value(&mut self, value: Self::Value);
    
    /// 线性插值
    fn lerp(&self, from: &Self::Value, to: &Self::Value, t: f32) -> Self::Value;
}

/// 动画配置
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// 动画时长
    pub duration: Duration,
    /// 缓动函数
    pub easing: EasingFunction,
    /// 是否循环
    pub looping: bool,
    /// 循环次数 (None表示无限循环)
    pub loop_count: Option<u32>,
    /// 延迟时间
    pub delay: Duration,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(1000),
            easing: EasingFunction::EaseInOut,
            looping: false,
            loop_count: None,
            delay: Duration::ZERO,
        }
    }
}

impl AnimationConfig {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            ..Default::default()
        }
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn looping(mut self, count: Option<u32>) -> Self {
        self.looping = true;
        self.loop_count = count;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_config_creation() {
        let config = AnimationConfig::new(Duration::from_millis(500));
        assert_eq!(config.duration, Duration::from_millis(500));
        assert_eq!(config.easing, EasingFunction::EaseInOut);
        assert!(!config.looping);
    }

    #[test]
    fn test_animation_config_builder() {
        let config = AnimationConfig::new(Duration::from_millis(1000))
            .with_easing(EasingFunction::EaseIn)
            .with_delay(Duration::from_millis(200))
            .looping(Some(3));

        assert_eq!(config.duration, Duration::from_millis(1000));
        assert_eq!(config.easing, EasingFunction::EaseIn);
        assert_eq!(config.delay, Duration::from_millis(200));
        assert!(config.looping);
        assert_eq!(config.loop_count, Some(3));
    }

    #[test]
    fn test_animation_state() {
        let state = AnimationState::NotStarted;
        assert_eq!(state, AnimationState::NotStarted);
        
        let state = AnimationState::Playing;
        assert_eq!(state, AnimationState::Playing);
    }
}
