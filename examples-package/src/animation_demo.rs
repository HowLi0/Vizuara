use vizuara_animation::{
    Timeline, AnimationSequence, ParallelAnimations, Transition, KeyframeAnimation,
    EasingFunction, AnimationState
};
use vizuara_core::Color;
use std::time::Duration;
use std::thread;

/// 动画演示程序
/// 展示各种动画效果的使用
fn main() {
    println!("🎬 Vizuara 动画系统演示");
    println!("========================");

    // 1. 基础时间轴演示
    basic_timeline_demo();
    
    // 2. 简单过渡动画演示
    simple_transition_demo();
    
    // 3. 关键帧动画演示  
    keyframe_animation_demo();
    
    // 4. 缓动函数演示
    easing_functions_demo();
    
    // 5. 动画序列演示
    animation_sequence_demo();
    
    // 6. 并行动画演示
    parallel_animations_demo();
    
    // 7. 颜色动画演示
    color_animation_demo();

    println!("\n✅ 所有动画演示完成!");
}

/// 基础时间轴演示
fn basic_timeline_demo() {
    println!("\n🕐 基础时间轴演示");
    println!("创建一个2秒的时间轴...");
    
    let mut timeline = Timeline::new(Duration::from_millis(2000));
    
    println!("初始状态: {:?}", timeline.state());
    println!("初始进度: {:.1}%", timeline.progress() * 100.0);
    
    timeline.start();
    println!("开始播放...");
    
    // 模拟动画循环
    for _i in 0..21 {
        timeline.update();
        let progress = timeline.progress();
        let state = timeline.state();
        
        print!("\r进度: {:.1}% | 状态: {:?}", progress * 100.0, state);
        
        if state == AnimationState::Completed {
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("\n✅ 时间轴演示完成");
}

/// 简单过渡动画演示
fn simple_transition_demo() {
    println!("\n➡️ 简单过渡动画演示");
    println!("创建从0.0到100.0的过渡动画...");
    
    let mut transition = Transition::simple(0.0f32, 100.0f32, Duration::from_millis(1000));
    transition.start();
    
    println!("开始值: {:.1}", transition.current_value(|from, to, t| from + (to - from) * t));
    
    // 模拟动画更新
    for i in 0..11 {
        transition.update();
        let value = transition.current_value(|from, to, t| from + (to - from) * t);
        let progress = transition.progress();
        
        println!("步骤 {}: 值={:.1}, 进度={:.1}%", i, value, progress * 100.0);
        
        if transition.is_completed() {
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("✅ 过渡动画演示完成");
}

/// 关键帧动画演示
fn keyframe_animation_demo() {
    println!("\n🎯 关键帧动画演示");
    println!("创建多段关键帧动画...");
    
    // 创建关键帧动画：0 -> 50 -> 100 -> 25
    let mut keyframe_anim = KeyframeAnimation::new(Duration::from_millis(1500))
        .at(0.0, 0.0f32)
        .at(0.33, 50.0f32)
        .at(0.66, 100.0f32)
        .at(1.0, 25.0f32);
    keyframe_anim.start();
    
    println!("关键帧序列: 0 -> 50 -> 100 -> 25");
    
    // 模拟动画更新
    for i in 0..16 {
        keyframe_anim.update();
        let value = keyframe_anim.current_value(|from, to, t| from + (to - from) * t).unwrap_or(0.0);
        let progress = keyframe_anim.progress();
        
        println!("步骤 {}: 值={:.1}, 进度={:.1}%", i, value, progress * 100.0);
        
        if keyframe_anim.state() == AnimationState::Completed {
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("✅ 关键帧动画演示完成");
}

/// 缓动函数演示
fn easing_functions_demo() {
    println!("\n🌊 缓动函数演示");
    println!("比较不同缓动函数效果...");
    
    let easings = vec![
        ("Linear", EasingFunction::Linear),
        ("EaseIn", EasingFunction::EaseIn),
        ("EaseOut", EasingFunction::EaseOut),
        ("EaseInOut", EasingFunction::EaseInOut),
        ("ElasticIn", EasingFunction::ElasticIn),
        ("BounceOut", EasingFunction::BounceOut),
    ];
    
    for (name, easing) in easings {
        println!("\n缓动函数: {}", name);
        print!("进度:  ");
        
        // 显示不同时间点的缓动值
        for i in 0..11 {
            let t = i as f32 / 10.0;
            let eased = easing.apply(t);
            print!("{:.2} ", eased);
        }
        println!();
    }
    
    println!("\n✅ 缓动函数演示完成");
}

/// 动画序列演示
fn animation_sequence_demo() {
    println!("\n📊 动画序列演示");
    println!("创建按时间顺序播放的动画序列...");
    
    let mut sequence = AnimationSequence::new()
        .at(Duration::ZERO, 0.0f32, 30.0f32, Duration::from_millis(500))           // 0-500ms: 0->30
        .at(Duration::from_millis(500), 30.0f32, 80.0f32, Duration::from_millis(300))  // 500-800ms: 30->80
        .at(Duration::from_millis(800), 80.0f32, 100.0f32, Duration::from_millis(200)); // 800-1000ms: 80->100
    
    sequence.start();
    
    println!("序列: [0-500ms] 0->30, [500-800ms] 30->80, [800-1000ms] 80->100");
    
    // 模拟动画更新
    for i in 0..11 {
        sequence.update();
        let values = sequence.current_values(|from, to, t| from + (to - from) * t);
        let progress = sequence.progress();
        
        print!("步骤 {}: 进度={:.1}%, 活跃值=[", i, progress * 100.0);
        for (j, value) in values.iter().enumerate() {
            if j > 0 { print!(", "); }
            print!("{:.1}", value);
        }
        println!("]");
        
        if sequence.state() == AnimationState::Completed {
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("✅ 动画序列演示完成");
}

/// 并行动画演示
fn parallel_animations_demo() {
    println!("\n⚡ 并行动画演示");
    println!("创建同时播放的多个动画...");
    
    let mut parallel = ParallelAnimations::new(Duration::from_millis(1000))
        .add_simple(0.0f32, 100.0f32, Duration::from_millis(1000))      // 动画1: 0->100
        .add_simple(200.0f32, 50.0f32, Duration::from_millis(1000))     // 动画2: 200->50
        .add_simple(75.0f32, 150.0f32, Duration::from_millis(1000));    // 动画3: 75->150
    
    parallel.start();
    
    println!("并行动画: 0->100, 200->50, 75->150");
    
    // 模拟动画更新
    for i in 0..11 {
        parallel.update();
        let values = parallel.current_values(|from, to, t| from + (to - from) * t);
        let progress = parallel.progress();
        
        print!("步骤 {}: 进度={:.1}%, 值=[", i, progress * 100.0);
        for (j, value) in values.iter().enumerate() {
            if j > 0 { print!(", "); }
            print!("{:.1}", value);
        }
        println!("]");
        
        if parallel.all_completed() {
            println!("所有动画已完成");
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("✅ 并行动画演示完成");
}

/// 颜色动画演示
fn color_animation_demo() {
    println!("\n🌈 颜色动画演示");
    println!("创建颜色过渡动画...");
    
    let red = Color::rgb(1.0, 0.0, 0.0);
    let blue = Color::rgb(0.0, 0.0, 1.0);
    
    let mut color_transition = Transition::simple(red, blue, Duration::from_millis(1000));
    color_transition.start();
    
    println!("颜色过渡: 红色 -> 蓝色");
    
    // 模拟颜色动画更新
    for i in 0..11 {
        color_transition.update();
        let color = color_transition.current_value(|from, to, t| {
            Color::rgb(
                from.r + (to.r - from.r) * t,
                from.g + (to.g - from.g) * t,
                from.b + (to.b - from.b) * t,
            )
        });
        let progress = color_transition.progress();
        
        println!("步骤 {}: 进度={:.1}%, RGB=({:.2}, {:.2}, {:.2})", 
                i, progress * 100.0, color.r, color.g, color.b);
        
        if color_transition.is_completed() {
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("✅ 颜色动画演示完成");
}
