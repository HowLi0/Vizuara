use crate::{AnimationConfig, AnimationState};
use std::time::{Duration, Instant};

/// 过渡动画
#[derive(Debug)]
pub struct Transition<T> {
    /// 起始值
    from: T,
    /// 目标值
    to: T,
    /// 动画配置
    config: AnimationConfig,
    /// 当前状态
    state: AnimationState,
    /// 开始时间
    start_time: Option<Instant>,
    /// 暂停时间
    pause_time: Option<Duration>,
    /// 当前循环次数
    current_loop: u32,
}

impl<T> Transition<T>
where
    T: Clone,
{
    /// 创建新的过渡动画
    pub fn new(from: T, to: T, config: AnimationConfig) -> Self {
        Self {
            from,
            to,
            config,
            state: AnimationState::NotStarted,
            start_time: None,
            pause_time: None,
            current_loop: 0,
        }
    }

    /// 创建简单的过渡动画
    pub fn simple(from: T, to: T, duration: Duration) -> Self {
        Self::new(from, to, AnimationConfig::new(duration))
    }

    /// 开始动画
    pub fn start(&mut self) {
        if self.config.delay.is_zero() {
            self.state = AnimationState::Playing;
            self.start_time = Some(Instant::now());
        } else {
            self.state = AnimationState::NotStarted;
            self.start_time = Some(Instant::now() + self.config.delay);
        }
        self.pause_time = None;
        self.current_loop = 0;
    }

    /// 暂停动画
    pub fn pause(&mut self) {
        if self.state == AnimationState::Playing {
            self.state = AnimationState::Paused;
            if let Some(start) = self.start_time {
                let now = Instant::now();
                if now >= start {
                    self.pause_time = Some(now.duration_since(start));
                }
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
        self.current_loop = 0;
    }

    /// 重置动画到目标值
    pub fn reset(&mut self, from: T, to: T) {
        self.from = from;
        self.to = to;
        self.stop();
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
                    (pause_duration.as_secs_f32() / self.config.duration.as_secs_f32())
                        .clamp(0.0, 1.0)
                } else {
                    0.0
                }
            }
            AnimationState::Playing => {
                if let Some(start) = self.start_time {
                    let now = Instant::now();
                    if now < start {
                        // 还在延迟期
                        return 0.0;
                    }

                    let elapsed = now.duration_since(start);
                    let progress = elapsed.as_secs_f32() / self.config.duration.as_secs_f32();
                    progress.clamp(0.0, 1.0)
                } else {
                    0.0
                }
            }
        }
    }

    /// 获取原始进度（未应用缓动）
    pub fn raw_progress(&self) -> f32 {
        self.progress()
    }

    /// 获取缓动后的进度
    pub fn eased_progress(&self) -> f32 {
        let raw = self.progress();
        self.config.easing.apply(raw)
    }

    /// 更新动画状态
    pub fn update(&mut self) {
        match self.state {
            AnimationState::NotStarted => {
                if let Some(start) = self.start_time {
                    if Instant::now() >= start {
                        self.state = AnimationState::Playing;
                    }
                }
            }
            AnimationState::Playing => {
                let progress = self.progress();
                if progress >= 1.0 {
                    if self.config.looping {
                        self.current_loop += 1;

                        // 检查是否达到循环次数限制
                        if let Some(max_loops) = self.config.loop_count {
                            if self.current_loop >= max_loops {
                                self.state = AnimationState::Completed;
                                return;
                            }
                        }

                        // 重启动画
                        self.start_time = Some(Instant::now());
                    } else {
                        self.state = AnimationState::Completed;
                    }
                }
            }
            _ => {}
        }
    }

    /// 获取当前插值结果
    pub fn current_value(&self, lerp_fn: impl Fn(&T, &T, f32) -> T) -> T {
        let t = self.eased_progress();
        lerp_fn(&self.from, &self.to, t)
    }

    /// 获取当前循环次数
    pub fn current_loop(&self) -> u32 {
        self.current_loop
    }

    /// 检查动画是否已完成
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// 检查动画是否正在播放
    pub fn is_playing(&self) -> bool {
        self.state == AnimationState::Playing
    }

    /// 获取剩余时间
    pub fn remaining_time(&self) -> Duration {
        if self.state != AnimationState::Playing {
            return Duration::ZERO;
        }

        let progress = self.progress();
        if progress >= 1.0 {
            return Duration::ZERO;
        }

        let remaining_progress = 1.0 - progress;
        Duration::from_secs_f32(remaining_progress * self.config.duration.as_secs_f32())
    }

    /// 获取动画配置
    pub fn config(&self) -> &AnimationConfig {
        &self.config
    }

    /// 获取动画时长
    pub fn duration(&self) -> Duration {
        self.config.duration
    }
}

/// f32类型的过渡动画特化实现
impl Transition<f32> {
    /// 获取当前f32值
    pub fn current_f32(&self) -> f32 {
        self.current_value(|from, to, t| from + (to - from) * t)
    }

    /// 设置新的目标值（平滑过渡）
    pub fn transition_to(&mut self, new_to: f32) {
        let current = self.current_f32();
        self.reset(current, new_to);
        self.start();
    }
}

/// 颜色过渡动画
impl Transition<vizuara_core::Color> {
    /// 获取当前颜色值
    pub fn current_color(&self) -> vizuara_core::Color {
        self.current_value(|from, to, t| {
            vizuara_core::Color::new(
                from.r + (to.r - from.r) * t,
                from.g + (to.g - from.g) * t,
                from.b + (to.b - from.b) * t,
                from.a + (to.a - from.a) * t,
            )
        })
    }
}

/// Point2过渡动画
impl Transition<nalgebra::Point2<f32>> {
    /// 获取当前Point2值
    pub fn current_point2(&self) -> nalgebra::Point2<f32> {
        self.current_value(|from, to, t| {
            nalgebra::Point2::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EasingFunction;
    use std::thread;

    #[test]
    fn test_transition_creation() {
        let transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(1000));
        assert_eq!(transition.state(), AnimationState::NotStarted);
        assert_eq!(transition.progress(), 0.0);
    }

    #[test]
    fn test_transition_with_config() {
        let config = AnimationConfig::new(Duration::from_millis(500))
            .with_easing(EasingFunction::EaseIn)
            .with_delay(Duration::from_millis(100));

        let transition = Transition::new(0.0f32, 100.0f32, config);
        assert_eq!(transition.state(), AnimationState::NotStarted);
    }

    #[test]
    fn test_transition_state_management() {
        let mut transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(100));

        // 初始状态
        assert_eq!(transition.state(), AnimationState::NotStarted);
        assert!(!transition.is_playing());
        assert!(!transition.is_completed());

        // 开始动画
        transition.start();
        assert_eq!(transition.state(), AnimationState::Playing);
        assert!(transition.is_playing());

        // 暂停动画
        transition.pause();
        assert_eq!(transition.state(), AnimationState::Paused);
        assert!(!transition.is_playing());

        // 恢复动画
        transition.resume();
        assert_eq!(transition.state(), AnimationState::Playing);
        assert!(transition.is_playing());

        // 停止动画
        transition.stop();
        assert_eq!(transition.state(), AnimationState::NotStarted);
        assert!(!transition.is_playing());
    }

    #[test]
    fn test_transition_progress() {
        let mut transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(100));
        transition.start();

        // 初始进度 - 使用近似比较
        let initial_progress = transition.progress();
        assert!(initial_progress < 0.1); // 允许小量的误差

        // 等待一段时间
        thread::sleep(Duration::from_millis(50));

        let progress = transition.progress();
        assert!(progress > 0.0 && progress < 1.0);

        // 更新状态
        transition.update();

        // 等待动画完成
        thread::sleep(Duration::from_millis(60));
        transition.update();

        assert!(transition.progress() >= 1.0); // 使用 >= 而不是 ==
        assert!(transition.is_completed());
    }

    #[test]
    fn test_transition_interpolation() {
        let transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(1000));

        // 模拟50%进度
        let mut test_transition = transition;
        test_transition.start();
        test_transition.start_time = Some(Instant::now() - Duration::from_millis(500));

        let current_value = test_transition.current_f32();
        assert!((current_value - 50.0).abs() < 1.0); // 允许小误差
    }

    #[test]
    fn test_transition_easing() {
        let config =
            AnimationConfig::new(Duration::from_millis(1000)).with_easing(EasingFunction::EaseIn);

        let mut transition = Transition::new(0.0f32, 100.0f32, config);
        transition.start();

        // 模拟50%进度
        transition.start_time = Some(Instant::now() - Duration::from_millis(500));

        let raw_progress = transition.raw_progress();
        let eased_progress = transition.eased_progress();

        // EaseIn应该使缓动进度小于原始进度
        assert!(eased_progress < raw_progress);
    }

    #[test]
    fn test_transition_looping() {
        let config = AnimationConfig::new(Duration::from_millis(100)).looping(Some(2));

        let mut transition = Transition::new(0.0f32, 100.0f32, config);
        transition.start();

        // 等待一个周期完成
        thread::sleep(Duration::from_millis(120));
        transition.update();

        // 应该仍在播放（第二轮循环）
        assert!(transition.is_playing() || transition.current_loop() > 0);

        // 等待第二个周期完成
        thread::sleep(Duration::from_millis(120));
        transition.update();

        // 现在应该完成了
        assert!(transition.is_completed() || transition.current_loop() >= 2);
    }

    #[test]
    fn test_transition_reset() {
        let mut transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(1000));
        transition.start();

        // 重置到新值
        transition.reset(50.0f32, 150.0f32);

        assert_eq!(transition.state(), AnimationState::NotStarted);
        assert_eq!(transition.current_loop(), 0);
    }

    #[test]
    fn test_transition_f32_transition_to() {
        let mut transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(100));
        transition.start();

        // 等待一段时间
        thread::sleep(Duration::from_millis(50));

        // 平滑过渡到新目标
        transition.transition_to(200.0f32);

        // 应该重新开始播放
        assert!(transition.is_playing());
    }

    #[test]
    fn test_color_transition() {
        let from = vizuara_core::Color::rgb(1.0, 0.0, 0.0); // 红色
        let to = vizuara_core::Color::rgb(0.0, 1.0, 0.0); // 绿色

        let mut transition = Transition::simple(from, to, Duration::from_millis(1000));
        transition.start();

        // 模拟50%进度
        transition.start_time = Some(Instant::now() - Duration::from_millis(500));

        let current_color = transition.current_color();

        // 中间颜色应该是(0.5, 0.5, 0.0)
        assert!((current_color.r - 0.5).abs() < 0.1);
        assert!((current_color.g - 0.5).abs() < 0.1);
        assert!((current_color.b - 0.0).abs() < 0.1);
    }

    #[test]
    fn test_point2_transition() {
        let from = nalgebra::Point2::new(0.0, 0.0);
        let to = nalgebra::Point2::new(100.0, 200.0);

        let mut transition = Transition::simple(from, to, Duration::from_millis(1000));
        transition.start();

        // 模拟50%进度
        transition.start_time = Some(Instant::now() - Duration::from_millis(500));

        let current_point = transition.current_point2();

        // 中间点应该是(50, 100)
        assert!((current_point.x - 50.0).abs() < 1.0);
        assert!((current_point.y - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_remaining_time() {
        let mut transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(1000));
        transition.start();

        // 模拟50%进度
        transition.start_time = Some(Instant::now() - Duration::from_millis(500));

        let remaining = transition.remaining_time();

        // 剩余时间应该大约是500ms
        assert!(remaining.as_millis() > 400 && remaining.as_millis() < 600);
    }

    #[test]
    fn test_transition_with_delay() {
        let config =
            AnimationConfig::new(Duration::from_millis(100)).with_delay(Duration::from_millis(50));

        let mut transition = Transition::new(0.0f32, 100.0f32, config);
        transition.start();

        // 在延迟期间，进度应该是0
        assert_eq!(transition.progress(), 0.0);
        assert_eq!(transition.state(), AnimationState::NotStarted);

        // 更新状态
        transition.update();

        // 等待延迟完成
        thread::sleep(Duration::from_millis(60));
        transition.update();

        // 现在应该开始播放
        assert_eq!(transition.state(), AnimationState::Playing);
    }
}
