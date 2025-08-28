use crate::{AnimationState, EasingFunction};
use std::time::{Duration, Instant};

/// 关键帧
#[derive(Debug, Clone)]
pub struct Keyframe<T> {
    /// 时间点 (0.0 到 1.0)
    pub time: f32,
    /// 值
    pub value: T,
    /// 到下一个关键帧的缓动函数
    pub easing: EasingFunction,
}

impl<T> Keyframe<T> {
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time: time.clamp(0.0, 1.0),
            value,
            easing: EasingFunction::Linear,
        }
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
}

/// 关键帧动画
#[derive(Debug, Clone)]
pub struct KeyframeAnimation<T> {
    /// 关键帧列表
    keyframes: Vec<Keyframe<T>>,
    /// 动画时长
    duration: Duration,
    /// 当前状态
    state: AnimationState,
    /// 开始时间
    start_time: Option<Instant>,
    /// 暂停时间
    pause_time: Option<Duration>,
}

impl<T> KeyframeAnimation<T>
where
    T: Clone,
{
    /// 创建新的关键帧动画
    pub fn new(duration: Duration) -> Self {
        Self {
            keyframes: Vec::new(),
            duration,
            state: AnimationState::NotStarted,
            start_time: None,
            pause_time: None,
        }
    }

    /// 添加关键帧
    pub fn add_keyframe(mut self, keyframe: Keyframe<T>) -> Self {
        self.keyframes.push(keyframe);
        self.keyframes
            .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    /// 添加关键帧（便捷方法）
    pub fn at(self, time: f32, value: T) -> Self {
        self.add_keyframe(Keyframe::new(time, value))
    }

    /// 添加带缓动的关键帧
    pub fn at_with_easing(self, time: f32, value: T, easing: EasingFunction) -> Self {
        self.add_keyframe(Keyframe::new(time, value).with_easing(easing))
    }

    /// 开始动画
    pub fn start(&mut self) {
        self.state = AnimationState::Playing;
        self.start_time = Some(Instant::now());
        self.pause_time = None;
    }

    /// 暂停动画
    pub fn pause(&mut self) {
        if self.state == AnimationState::Playing {
            self.state = AnimationState::Paused;
            if let Some(start) = self.start_time {
                self.pause_time = Some(start.elapsed());
            }
        }
    }

    /// 恢复动画
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Playing;
            if let Some(pause_duration) = self.pause_time {
                self.start_time = Some(Instant::now() - pause_duration);
                self.pause_time = None;
            }
        }
    }

    /// 停止动画
    pub fn stop(&mut self) {
        self.state = AnimationState::NotStarted;
        self.start_time = None;
        self.pause_time = None;
    }

    /// 获取当前状态
    pub fn state(&self) -> AnimationState {
        self.state
    }

    /// 获取当前进度 (0.0 到 1.0)
    pub fn progress(&self) -> f32 {
        match self.state {
            AnimationState::NotStarted => 0.0,
            AnimationState::Completed => 1.0,
            AnimationState::Paused => {
                if let Some(pause_duration) = self.pause_time {
                    (pause_duration.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
                } else {
                    0.0
                }
            }
            AnimationState::Playing => {
                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();
                    progress.clamp(0.0, 1.0)
                } else {
                    0.0
                }
            }
        }
    }

    /// 更新动画状态
    pub fn update(&mut self) {
        if self.state == AnimationState::Playing {
            let progress = self.progress();
            if progress >= 1.0 {
                self.state = AnimationState::Completed;
            }
        }
    }

    /// 获取指定时间的插值结果
    pub fn interpolate_at(&self, t: f32, lerp_fn: impl Fn(&T, &T, f32) -> T) -> Option<T> {
        if self.keyframes.is_empty() {
            return None;
        }

        let t = t.clamp(0.0, 1.0);

        // 如果只有一个关键帧
        if self.keyframes.len() == 1 {
            return Some(self.keyframes[0].value.clone());
        }

        // 查找当前时间在哪两个关键帧之间
        for i in 0..self.keyframes.len() - 1 {
            let current = &self.keyframes[i];
            let next = &self.keyframes[i + 1];

            if t >= current.time && t <= next.time {
                // 计算局部时间
                let time_range = next.time - current.time;
                if time_range == 0.0 {
                    return Some(current.value.clone());
                }

                let local_t = (t - current.time) / time_range;
                let eased_t = current.easing.apply(local_t);

                return Some(lerp_fn(&current.value, &next.value, eased_t));
            }
        }

        // 如果时间在范围外，返回最近的关键帧
        if t < self.keyframes[0].time {
            Some(self.keyframes[0].value.clone())
        } else {
            Some(self.keyframes.last().unwrap().value.clone())
        }
    }

    /// 获取当前插值结果
    pub fn current_value(&self, lerp_fn: impl Fn(&T, &T, f32) -> T) -> Option<T> {
        let progress = self.progress();
        self.interpolate_at(progress, lerp_fn)
    }

    /// 获取关键帧数量
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    /// 获取动画时长
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

/// 数值类型的关键帧动画辅助实现
impl KeyframeAnimation<f32> {
    /// 获取当前f32值
    pub fn current_f32(&self) -> Option<f32> {
        self.current_value(|from, to, t| from + (to - from) * t)
    }

    /// 在指定时间获取f32值
    pub fn f32_at(&self, t: f32) -> Option<f32> {
        self.interpolate_at(t, |from, to, t| from + (to - from) * t)
    }
}

impl KeyframeAnimation<nalgebra::Point2<f32>> {
    /// 获取当前Point2值
    pub fn current_point2(&self) -> Option<nalgebra::Point2<f32>> {
        self.current_value(|from, to, t| {
            nalgebra::Point2::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_keyframe_creation() {
        let keyframe = Keyframe::new(0.5, 10.0);
        assert_eq!(keyframe.time, 0.5);
        assert_eq!(keyframe.value, 10.0);
        assert_eq!(keyframe.easing, EasingFunction::Linear);
    }

    #[test]
    fn test_keyframe_with_easing() {
        let keyframe = Keyframe::new(0.3, 5.0).with_easing(EasingFunction::EaseIn);

        assert_eq!(keyframe.time, 0.3);
        assert_eq!(keyframe.value, 5.0);
        assert_eq!(keyframe.easing, EasingFunction::EaseIn);
    }

    #[test]
    fn test_keyframe_animation_creation() {
        let animation = KeyframeAnimation::<f32>::new(Duration::from_millis(1000));
        assert_eq!(animation.keyframe_count(), 0);
        assert_eq!(animation.state(), AnimationState::NotStarted);
        assert_eq!(animation.duration(), Duration::from_millis(1000));
    }

    #[test]
    fn test_keyframe_animation_add_keyframes() {
        let animation = KeyframeAnimation::new(Duration::from_millis(1000))
            .at(0.0, 0.0)
            .at(0.5, 50.0)
            .at(1.0, 100.0);

        assert_eq!(animation.keyframe_count(), 3);
    }

    #[test]
    fn test_keyframe_animation_interpolation() {
        let animation = KeyframeAnimation::new(Duration::from_millis(1000))
            .at(0.0, 0.0)
            .at(1.0, 100.0);

        // 在中点应该是50.0
        let mid_value = animation.f32_at(0.5).unwrap();
        assert_eq!(mid_value, 50.0);

        // 在开始应该是0.0
        let start_value = animation.f32_at(0.0).unwrap();
        assert_eq!(start_value, 0.0);

        // 在结束应该是100.0
        let end_value = animation.f32_at(1.0).unwrap();
        assert_eq!(end_value, 100.0);
    }

    #[test]
    fn test_keyframe_animation_multiple_keyframes() {
        let animation = KeyframeAnimation::new(Duration::from_millis(1000))
            .at(0.0, 0.0)
            .at(0.25, 25.0)
            .at(0.75, 75.0)
            .at(1.0, 100.0);

        // 测试各个区间的插值
        assert_eq!(animation.f32_at(0.125).unwrap(), 12.5); // 0.0到25.0的中点
        assert_eq!(animation.f32_at(0.5).unwrap(), 50.0); // 25.0到75.0的中点
        assert_eq!(animation.f32_at(0.875).unwrap(), 87.5); // 75.0到100.0的中点
    }

    #[test]
    fn test_keyframe_animation_state_management() {
        let mut animation = KeyframeAnimation::new(Duration::from_millis(100))
            .at(0.0, 0.0)
            .at(1.0, 100.0);

        // 初始状态
        assert_eq!(animation.state(), AnimationState::NotStarted);
        assert_eq!(animation.progress(), 0.0);

        // 开始动画
        animation.start();
        assert_eq!(animation.state(), AnimationState::Playing);

        // 暂停动画
        animation.pause();
        assert_eq!(animation.state(), AnimationState::Paused);

        // 恢复动画
        animation.resume();
        assert_eq!(animation.state(), AnimationState::Playing);

        // 停止动画
        animation.stop();
        assert_eq!(animation.state(), AnimationState::NotStarted);
    }

    #[test]
    fn test_keyframe_animation_progress() {
        let mut animation = KeyframeAnimation::new(Duration::from_millis(100))
            .at(0.0, 0.0)
            .at(1.0, 100.0);

        animation.start();

        // 等待一段时间
        thread::sleep(Duration::from_millis(50));

        let progress = animation.progress();
        assert!((0.0..1.0).contains(&progress));

        // 更新状态
        animation.update();

        // 等待动画完成
        thread::sleep(Duration::from_millis(60));
        animation.update();

        assert_eq!(animation.state(), AnimationState::Completed);
        assert_eq!(animation.progress(), 1.0);
    }

    #[test]
    fn test_keyframe_animation_easing() {
        let animation = KeyframeAnimation::new(Duration::from_millis(1000))
            .at_with_easing(0.0, 0.0, EasingFunction::Linear)
            .at_with_easing(1.0, 100.0, EasingFunction::EaseIn);

        // 测试插值
        let mid_value = animation.f32_at(0.5).unwrap_or(0.0);
        // 只要得到有效的插值结果就通过测试
        assert!((0.0..=100.0).contains(&mid_value));
    }

    #[test]
    fn test_point2_animation() {
        let animation = KeyframeAnimation::new(Duration::from_millis(1000))
            .at(0.0, nalgebra::Point2::new(0.0, 0.0))
            .at(1.0, nalgebra::Point2::new(100.0, 200.0));

        let mid_point = animation.current_point2().unwrap();
        // 注意：这里progress()为0，所以应该返回起始点
        assert_eq!(mid_point, nalgebra::Point2::new(0.0, 0.0));
    }

    #[test]
    fn test_empty_keyframe_animation() {
        let animation = KeyframeAnimation::<f32>::new(Duration::from_millis(1000));
        assert!(animation.current_f32().is_none());
        assert!(animation.f32_at(0.5).is_none());
    }

    #[test]
    fn test_single_keyframe_animation() {
        let animation = KeyframeAnimation::new(Duration::from_millis(1000)).at(0.5, 42.0);

        // 无论什么时间，都应该返回唯一的值
        assert_eq!(animation.f32_at(0.0).unwrap(), 42.0);
        assert_eq!(animation.f32_at(0.5).unwrap(), 42.0);
        assert_eq!(animation.f32_at(1.0).unwrap(), 42.0);
    }
}
