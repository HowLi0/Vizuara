use crate::{AnimationState, Transition};
use std::time::{Duration, Instant};

/// 时间轴 - 管理多个动画的同步播放
#[derive(Debug)]
pub struct Timeline {
    /// 时间轴开始时间
    start_time: Option<Instant>,
    /// 时间轴状态
    state: AnimationState,
    /// 总时长
    duration: Duration,
    /// 暂停时间
    pause_time: Option<Duration>,
}

impl Timeline {
    /// 创建新的时间轴
    pub fn new(duration: Duration) -> Self {
        Self {
            start_time: None,
            state: AnimationState::NotStarted,
            duration,
            pause_time: None,
        }
    }

    /// 开始时间轴
    pub fn start(&mut self) {
        self.state = AnimationState::Playing;
        self.start_time = Some(Instant::now());
        self.pause_time = None;
    }

    /// 暂停时间轴
    pub fn pause(&mut self) {
        if self.state == AnimationState::Playing {
            self.state = AnimationState::Paused;
            if let Some(start) = self.start_time {
                self.pause_time = Some(start.elapsed());
            }
        }
    }

    /// 恢复时间轴
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Playing;
            if let Some(pause_duration) = self.pause_time {
                self.start_time = Some(Instant::now() - pause_duration);
                self.pause_time = None;
            }
        }
    }

    /// 停止时间轴
    pub fn stop(&mut self) {
        self.state = AnimationState::NotStarted;
        self.start_time = None;
        self.pause_time = None;
    }

    /// 获取当前状态
    pub fn state(&self) -> AnimationState {
        self.state
    }

    /// 获取当前时间轴进度 (0.0 到 1.0)
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

    /// 获取当前时间 (相对于时间轴开始)
    pub fn current_time(&self) -> Duration {
        let progress = self.progress();
        Duration::from_secs_f32(progress * self.duration.as_secs_f32())
    }

    /// 更新时间轴状态
    pub fn update(&mut self) {
        if self.state == AnimationState::Playing {
            let progress = self.progress();
            if progress >= 1.0 {
                self.state = AnimationState::Completed;
            }
        }
    }

    /// 设置时间轴到指定进度
    pub fn seek(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        let target_time = Duration::from_secs_f32(progress * self.duration.as_secs_f32());

        if self.state == AnimationState::Playing {
            self.start_time = Some(Instant::now() - target_time);
        } else {
            self.pause_time = Some(target_time);
        }
    }

    /// 获取时间轴总时长
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

/// 动画序列 - 按顺序播放多个动画
#[derive(Debug)]
pub struct AnimationSequence<T> {
    /// 动画列表
    animations: Vec<(Duration, Transition<T>)>, // (开始时间, 动画)
    /// 时间轴
    timeline: Timeline,
}

impl<T> AnimationSequence<T>
where
    T: Clone,
{
    /// 创建新的动画序列
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            timeline: Timeline::new(Duration::ZERO),
        }
    }

    /// 添加动画
    pub fn add_animation(mut self, start_time: Duration, animation: Transition<T>) -> Self {
        self.animations.push((start_time, animation));

        // 更新总时长
        let total_duration = self
            .animations
            .iter()
            .map(|(start, anim)| *start + anim.duration())
            .max()
            .unwrap_or(Duration::ZERO);

        self.timeline = Timeline::new(total_duration);
        self
    }

    /// 在指定时间添加动画（便捷方法）
    pub fn at(self, start_time: Duration, from: T, to: T, duration: Duration) -> Self {
        let transition = Transition::simple(from, to, duration);
        self.add_animation(start_time, transition)
    }

    /// 开始序列
    pub fn start(&mut self) {
        self.timeline.start();

        // 初始化所有动画
        for (_, animation) in &mut self.animations {
            animation.stop();
        }
    }

    /// 暂停序列
    pub fn pause(&mut self) {
        self.timeline.pause();
        for (_, animation) in &mut self.animations {
            animation.pause();
        }
    }

    /// 恢复序列
    pub fn resume(&mut self) {
        self.timeline.resume();
        for (_, animation) in &mut self.animations {
            animation.resume();
        }
    }

    /// 停止序列
    pub fn stop(&mut self) {
        self.timeline.stop();
        for (_, animation) in &mut self.animations {
            animation.stop();
        }
    }

    /// 更新序列状态
    pub fn update(&mut self) {
        self.timeline.update();

        if self.timeline.state() == AnimationState::Playing {
            let current_time = self.timeline.current_time();

            for (start_time, animation) in &mut self.animations {
                if current_time >= *start_time {
                    // 应该开始这个动画
                    if animation.state() == AnimationState::NotStarted {
                        animation.start();
                    }
                    animation.update();
                }
            }
        }
    }

    /// 获取当前活跃的动画值
    pub fn current_values(&self, lerp_fn: impl Fn(&T, &T, f32) -> T + Copy) -> Vec<T> {
        let mut values = Vec::new();

        for (_, animation) in &self.animations {
            if animation.state() == AnimationState::Playing
                || animation.state() == AnimationState::Completed
            {
                values.push(animation.current_value(lerp_fn));
            }
        }

        values
    }

    /// 获取序列状态
    pub fn state(&self) -> AnimationState {
        self.timeline.state()
    }

    /// 获取序列进度
    pub fn progress(&self) -> f32 {
        self.timeline.progress()
    }

    /// 获取动画数量
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }
}

impl<T> Default for AnimationSequence<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// 并行动画组 - 同时播放多个动画
#[derive(Debug)]
pub struct ParallelAnimations<T> {
    /// 动画列表
    animations: Vec<Transition<T>>,
    /// 时间轴
    timeline: Timeline,
}

impl<T> ParallelAnimations<T>
where
    T: Clone,
{
    /// 创建新的并行动画组
    pub fn new(duration: Duration) -> Self {
        Self {
            animations: Vec::new(),
            timeline: Timeline::new(duration),
        }
    }

    /// 添加动画
    pub fn add_animation(mut self, animation: Transition<T>) -> Self {
        self.animations.push(animation);
        self
    }

    /// 添加简单动画（便捷方法）
    pub fn add_simple(self, from: T, to: T, duration: Duration) -> Self {
        let transition = Transition::simple(from, to, duration);
        self.add_animation(transition)
    }

    /// 开始所有动画
    pub fn start(&mut self) {
        self.timeline.start();
        for animation in &mut self.animations {
            animation.start();
        }
    }

    /// 暂停所有动画
    pub fn pause(&mut self) {
        self.timeline.pause();
        for animation in &mut self.animations {
            animation.pause();
        }
    }

    /// 恢复所有动画
    pub fn resume(&mut self) {
        self.timeline.resume();
        for animation in &mut self.animations {
            animation.resume();
        }
    }

    /// 停止所有动画
    pub fn stop(&mut self) {
        self.timeline.stop();
        for animation in &mut self.animations {
            animation.stop();
        }
    }

    /// 更新所有动画
    pub fn update(&mut self) {
        self.timeline.update();
        for animation in &mut self.animations {
            animation.update();
        }
    }

    /// 获取所有当前值
    pub fn current_values(&self, lerp_fn: impl Fn(&T, &T, f32) -> T + Copy) -> Vec<T> {
        self.animations
            .iter()
            .map(|anim| anim.current_value(lerp_fn))
            .collect()
    }

    /// 获取组状态
    pub fn state(&self) -> AnimationState {
        self.timeline.state()
    }

    /// 获取组进度
    pub fn progress(&self) -> f32 {
        self.timeline.progress()
    }

    /// 检查是否所有动画都完成
    pub fn all_completed(&self) -> bool {
        self.animations.iter().all(|anim| anim.is_completed())
    }

    /// 获取动画数量
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new(Duration::from_millis(1000));
        assert_eq!(timeline.state(), AnimationState::NotStarted);
        assert_eq!(timeline.progress(), 0.0);
        assert_eq!(timeline.duration(), Duration::from_millis(1000));
    }

    #[test]
    fn test_timeline_progress() {
        let mut timeline = Timeline::new(Duration::from_millis(100));
        timeline.start();

        assert_eq!(timeline.state(), AnimationState::Playing);

        // 等待一段时间
        thread::sleep(Duration::from_millis(50));

        let progress = timeline.progress();
        assert!(progress > 0.0 && progress < 1.0);

        // 更新状态
        timeline.update();

        // 等待完成
        thread::sleep(Duration::from_millis(60));
        timeline.update();

        assert_eq!(timeline.state(), AnimationState::Completed);
        assert_eq!(timeline.progress(), 1.0);
    }

    #[test]
    fn test_timeline_seek() {
        let mut timeline = Timeline::new(Duration::from_millis(1000));
        timeline.start();

        // 跳转到50%
        timeline.seek(0.5);

        let progress = timeline.progress();
        assert!((progress - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_animation_sequence() {
        let mut sequence = AnimationSequence::new()
            .at(Duration::ZERO, 0.0f32, 50.0f32, Duration::from_millis(100))
            .at(
                Duration::from_millis(100),
                50.0f32,
                100.0f32,
                Duration::from_millis(100),
            );

        assert_eq!(sequence.animation_count(), 2);
        assert_eq!(sequence.state(), AnimationState::NotStarted);

        sequence.start();
        assert_eq!(sequence.state(), AnimationState::Playing);
    }

    #[test]
    fn test_parallel_animations() {
        let mut parallel = ParallelAnimations::new(Duration::from_millis(200))
            .add_simple(0.0f32, 100.0f32, Duration::from_millis(200))
            .add_simple(200.0f32, 300.0f32, Duration::from_millis(200));

        assert_eq!(parallel.animation_count(), 2);
        assert_eq!(parallel.state(), AnimationState::NotStarted);

        parallel.start();
        assert_eq!(parallel.state(), AnimationState::Playing);

        // 获取当前值
        let values = parallel.current_values(|from, to, t| from + (to - from) * t);
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_timeline_state_management() {
        let mut timeline = Timeline::new(Duration::from_millis(1000));

        // 开始
        timeline.start();
        assert_eq!(timeline.state(), AnimationState::Playing);

        // 暂停
        timeline.pause();
        assert_eq!(timeline.state(), AnimationState::Paused);

        // 恢复
        timeline.resume();
        assert_eq!(timeline.state(), AnimationState::Playing);

        // 停止
        timeline.stop();
        assert_eq!(timeline.state(), AnimationState::NotStarted);
    }

    #[test]
    fn test_sequence_timing() {
        let mut sequence = AnimationSequence::new()
            .at(Duration::ZERO, 0.0f32, 10.0f32, Duration::from_millis(50))
            .at(
                Duration::from_millis(50),
                10.0f32,
                20.0f32,
                Duration::from_millis(50),
            );

        sequence.start();

        // 在开始时，应该只有第一个动画活跃
        sequence.update();
        let _values = sequence.current_values(|from, to, t| from + (to - from) * t);
        // 注意：可能没有值，因为动画刚开始

        // 等待第一个动画完成
        thread::sleep(Duration::from_millis(60));
        sequence.update();

        let values = sequence.current_values(|from, to, t| from + (to - from) * t);
        // 现在应该有值了
        assert!(!values.is_empty());
    }

    #[test]
    fn test_parallel_all_completed() {
        let mut parallel = ParallelAnimations::new(Duration::from_millis(50))
            .add_simple(0.0f32, 100.0f32, Duration::from_millis(50))
            .add_simple(200.0f32, 300.0f32, Duration::from_millis(50));

        parallel.start();

        // 初始时不应该全部完成
        assert!(!parallel.all_completed());

        // 等待完成
        thread::sleep(Duration::from_millis(60));
        parallel.update();

        // 现在应该全部完成
        assert!(parallel.all_completed());
    }

    #[test]
    fn test_timeline_current_time() {
        let mut timeline = Timeline::new(Duration::from_millis(1000));
        timeline.start();

        // 模拟50%进度
        timeline.seek(0.5);

        let current_time = timeline.current_time();
        assert!((current_time.as_millis() as f32 - 500.0).abs() < 50.0);
    }
}
