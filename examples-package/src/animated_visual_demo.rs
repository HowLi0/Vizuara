use std::sync::Arc;
use std::time::{Duration, Instant};
use nalgebra::Point2;
use winit::event::{Event, WindowEvent, ElementState, KeyEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::keyboard::{KeyCode, PhysicalKey};

use vizuara_core::{Color, Style};
use vizuara_wgpu::WgpuRenderer;
use vizuara_animation::{
    AnimationConfig, AnimationState,
    transition::Transition,
    keyframe::KeyframeAnimation,
    easing::EasingFunction,
    timeline::Timeline,
};

/// 动画可视化演示：实时动画渲染
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 动画可视化演示启动");
    println!("控制键：");
    println!("  SPACE - 开始/暂停动画");
    println!("  R - 重置动画");
    println!("  1-3 - 切换动画类型");
    println!("  ESC - 退出");

    // 1) 初始窗口/渲染器
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Vizuara - 动画可视化演示")
            .with_inner_size(winit::dpi::LogicalSize::new(1000u32, 800u32))
            .with_min_inner_size(winit::dpi::LogicalSize::new(600u32, 480u32))
            .build(&event_loop)?
    );
    let size = window.inner_size();
    let (mut renderer, surface) = WgpuRenderer::new(&window, size).await?;

    // 2) 动画状态
    let mut animation_type = 1; // 当前动画类型
    let mut last_frame_time = Instant::now();
    
    // 动画对象们
    let mut simple_transition = create_simple_transition();
    let mut keyframe_anim = create_keyframe_animation();
    let mut timeline = create_timeline_animation();
    
    // 启动初始动画
    simple_transition.start();

    let window_id = window.id();
    let window_for_redraw = Arc::clone(&window);

    println!("✅ 动画系统初始化完成，开始渲染循环");

    // 3) 主事件循环
    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent { event, window_id: wid, .. } if wid == window_id => {
                    match event {
                        WindowEvent::CloseRequested => control_flow.exit(),

                        WindowEvent::Resized(physical_size) => {
                            if physical_size.width > 0 && physical_size.height > 0 {
                                renderer.resize(physical_size, &surface);
                                window_for_redraw.request_redraw();
                            }
                        }

                        WindowEvent::KeyboardInput { 
                            event: KeyEvent { physical_key, state: ElementState::Pressed, .. }, .. 
                        } => {
                            match physical_key {
                                PhysicalKey::Code(KeyCode::Escape) => control_flow.exit(),
                                PhysicalKey::Code(KeyCode::Space) => {
                                    // 开始/暂停当前动画
                                    match animation_type {
                                        1 => toggle_simple_transition(&mut simple_transition),
                                        2 => toggle_keyframe_animation(&mut keyframe_anim),
                                        3 => toggle_timeline(&mut timeline),
                                        _ => {}
                                    }
                                    window_for_redraw.request_redraw();
                                }
                                PhysicalKey::Code(KeyCode::KeyR) => {
                                    // 重置所有动画
                                    simple_transition = create_simple_transition();
                                    keyframe_anim = create_keyframe_animation();
                                    timeline = create_timeline_animation();
                                    
                                    // 自动开始当前选中的动画
                                    match animation_type {
                                        1 => {
                                            simple_transition.start();
                                            println!("🔄 重置并启动简单过渡动画");
                                        }
                                        2 => {
                                            keyframe_anim.start();
                                            println!("🔄 重置并启动关键帧动画");
                                        }
                                        3 => {
                                            timeline.start();
                                            println!("🔄 重置并启动时间轴动画");
                                        }
                                        _ => {
                                            println!("🔄 所有动画已重置");
                                        }
                                    }
                                    window_for_redraw.request_redraw();
                                }
                                PhysicalKey::Code(KeyCode::Digit1) => {
                                    animation_type = 1;
                                    // 重新创建并自动开始当前动画
                                    simple_transition = create_simple_transition();
                                    simple_transition.start();
                                    println!("📊 切换到动画类型1：简单过渡 (重新创建并自动开始)");
                                    println!("🔍 简单过渡状态: {:?}", simple_transition.state());
                                    window_for_redraw.request_redraw();
                                }
                                PhysicalKey::Code(KeyCode::Digit2) => {
                                    animation_type = 2;
                                    // 重新创建并自动开始当前动画
                                    keyframe_anim = create_keyframe_animation();
                                    keyframe_anim.start();
                                    println!("📊 切换到动画类型2：关键帧动画 (重新创建并自动开始)");
                                    println!("🔍 关键帧动画状态: {:?}", keyframe_anim.state());
                                    window_for_redraw.request_redraw();
                                }
                                PhysicalKey::Code(KeyCode::Digit3) => {
                                    animation_type = 3;
                                    // 重新创建并自动开始当前动画
                                    timeline = create_timeline_animation();
                                    timeline.start();
                                    println!("📊 切换到动画类型3：时间轴动画 (重新创建并自动开始)");
                                    println!("🔍 时间轴动画状态: {:?}", timeline.state());
                                    window_for_redraw.request_redraw();
                                }
                                _ => {}
                            }
                        }

                        WindowEvent::RedrawRequested => {
                            // 计算delta时间
                            let now = Instant::now();
                            let delta_time = now.duration_since(last_frame_time);
                            last_frame_time = now;

                            // 更新动画
                            update_animations(
                                delta_time,
                                &mut simple_transition,
                                &mut keyframe_anim,
                                &mut timeline,
                            );

                            // 根据当前动画类型生成数据和样式
                            let (primitives, styles) = generate_animation_visuals(
                                animation_type,
                                &simple_transition,
                                &keyframe_anim,
                                &timeline,
                            );

                            // 渲染 (注意参数顺序)
                            if let Err(e) = renderer.render(&surface, &primitives, &styles) {
                                eprintln!("❌ 渲染错误: {}", e);
                            }

                            // 如果动画在播放，继续请求重绘
                            if is_any_animation_playing(
                                &simple_transition,
                                &keyframe_anim,
                                &timeline,
                            ) {
                                window_for_redraw.request_redraw();
                            }
                        }

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    // 定期请求重绘以保持动画流畅
                    if is_any_animation_playing(
                        &simple_transition,
                        &keyframe_anim,
                        &timeline,
                    ) {
                        window_for_redraw.request_redraw();
                    }
                }

                _ => {}
            }
        })?;

    Ok(())
}

// 创建各种动画对象
fn create_simple_transition() -> Transition<f32> {
    Transition::new(
        0.0,
        100.0,
        AnimationConfig::new(Duration::from_secs(3))
            .with_easing(EasingFunction::EaseInOut)
    )
}

fn create_keyframe_animation() -> KeyframeAnimation<f32> {
    KeyframeAnimation::new(Duration::from_secs(4))
        .at(0.0, 0.0)
        .at(0.33, 80.0)
        .at(0.66, 20.0)
        .at(1.0, 100.0)
}

fn create_timeline_animation() -> Timeline {
    Timeline::new(Duration::from_secs(5))
}

// 动画控制函数
fn toggle_simple_transition(transition: &mut Transition<f32>) {
    match transition.state() {
        AnimationState::NotStarted | AnimationState::Paused => {
            transition.start();
            println!("▶️ 简单过渡动画开始播放");
        }
        AnimationState::Playing => {
            transition.pause();
            println!("⏸️ 简单过渡动画暂停");
        }
        AnimationState::Completed => {
            // 重新开始动画
            *transition = create_simple_transition();
            transition.start();
            println!("🔄 简单过渡动画重新开始");
        }
    }
}

fn toggle_keyframe_animation(anim: &mut KeyframeAnimation<f32>) {
    match anim.state() {
        AnimationState::NotStarted | AnimationState::Paused => {
            anim.start();
            println!("▶️ 关键帧动画开始播放");
        }
        AnimationState::Playing => {
            anim.pause();
            println!("⏸️ 关键帧动画暂停");
        }
        AnimationState::Completed => {
            // 重新创建并开始关键帧动画
            *anim = create_keyframe_animation();
            anim.start();
            println!("🔄 关键帧动画重新开始");
        }
    }
}

fn toggle_timeline(timeline: &mut Timeline) {
    match timeline.state() {
        AnimationState::NotStarted | AnimationState::Paused => {
            timeline.start();
            println!("▶️ 时间轴动画开始播放");
        }
        AnimationState::Playing => {
            timeline.pause();
            println!("⏸️ 时间轴动画暂停");
        }
        AnimationState::Completed => {
            // 重新开始时间轴
            *timeline = create_timeline_animation();
            timeline.start();
            println!("🔄 时间轴动画重新开始");
        }
    }
}

// 更新所有动画
fn update_animations(
    _delta_time: Duration,
    simple_transition: &mut Transition<f32>,
    keyframe_anim: &mut KeyframeAnimation<f32>,
    timeline: &mut Timeline,
) {
    simple_transition.update();
    keyframe_anim.update();
    timeline.update();
}

// 检查是否有动画在播放
fn is_any_animation_playing(
    simple_transition: &Transition<f32>,
    keyframe_anim: &KeyframeAnimation<f32>,
    timeline: &Timeline,
) -> bool {
    matches!(simple_transition.state(), AnimationState::Playing) ||
    matches!(keyframe_anim.state(), AnimationState::Playing) ||
    matches!(timeline.state(), AnimationState::Playing)
}

// 根据动画状态生成可视化内容
fn generate_animation_visuals(
    animation_type: i32,
    simple_transition: &Transition<f32>,
    keyframe_anim: &KeyframeAnimation<f32>,
    timeline: &Timeline,
) -> (Vec<vizuara_core::Primitive>, Vec<Style>) {
    let mut primitives = Vec::new();
    let mut styles = Vec::new();

    // 添加标题文本
    let title_text = format!("动画类型 {}: {}", animation_type, get_animation_name(animation_type));
    primitives.push(vizuara_core::Primitive::Text {
        position: Point2::new(10.0, 30.0),
        content: title_text,
        size: 20.0,
        color: Color::rgb(1.0, 1.0, 1.0),
        h_align: vizuara_core::HorizontalAlign::Left,
        v_align: vizuara_core::VerticalAlign::Top,
    });
    styles.push(Style::new().fill_color(Color::rgb(1.0, 1.0, 1.0)));

    // 添加控制说明
    let controls = "SPACE: 播放/暂停  R: 重置  1-3: 切换动画  ESC: 退出";
    primitives.push(vizuara_core::Primitive::Text {
        position: Point2::new(10.0, 60.0),
        content: controls.to_string(),
        size: 14.0,
        color: Color::rgb(0.8, 0.8, 0.8),
        h_align: vizuara_core::HorizontalAlign::Left,
        v_align: vizuara_core::VerticalAlign::Top,
    });
    styles.push(Style::new().fill_color(Color::rgb(0.8, 0.8, 0.8)));

    // 根据动画类型生成特定的可视化
    match animation_type {
        1 => generate_simple_transition_visual(simple_transition, &mut primitives, &mut styles),
        2 => generate_keyframe_visual(keyframe_anim, &mut primitives, &mut styles),
        3 => generate_timeline_visual(timeline, &mut primitives, &mut styles),
        _ => {}
    }

    (primitives, styles)
}

fn get_animation_name(animation_type: i32) -> &'static str {
    match animation_type {
        1 => "简单过渡动画",
        2 => "关键帧动画",
        3 => "时间轴动画",
        _ => "未知动画",
    }
}

fn generate_simple_transition_visual(
    transition: &Transition<f32>,
    primitives: &mut Vec<vizuara_core::Primitive>,
    styles: &mut Vec<Style>,
) {
    // 使用lerp函数获取当前值
    let current_value = transition.current_value(|from, to, t| from + (to - from) * t);
    let progress = transition.progress();

    // 状态信息
    let status_text = format!(
        "状态: {:?} | 值: {:.1} | 进度: {:.1}%", 
        transition.state(), 
        current_value, 
        progress * 100.0
    );
    primitives.push(vizuara_core::Primitive::Text {
        position: Point2::new(10.0, 100.0),
        content: status_text,
        size: 16.0,
        color: Color::rgb(1.0, 1.0, 0.0),
        h_align: vizuara_core::HorizontalAlign::Left,
        v_align: vizuara_core::VerticalAlign::Top,
    });
    styles.push(Style::new().fill_color(Color::rgb(1.0, 1.0, 0.0)));

    // 动画圆球：从左到右移动
    let ball_x = 100.0 + (current_value / 100.0) * 600.0; // 映射到屏幕坐标
    let ball_y = 200.0;
    
    primitives.push(vizuara_core::Primitive::Circle {
        center: Point2::new(ball_x, ball_y),
        radius: 15.0,
    });
    styles.push(Style::new()
        .fill_color(Color::rgb(0.2, 0.8, 0.2))
        .stroke(Color::rgb(0.0, 0.6, 0.0), 2.0));

    // 进度条背景
    primitives.push(vizuara_core::Primitive::Rectangle {
        min: Point2::new(100.0, 250.0),
        max: Point2::new(700.0, 270.0),
    });
    styles.push(Style::new()
        .fill_color(Color::rgb(0.3, 0.3, 0.3))
        .stroke(Color::rgb(0.6, 0.6, 0.6), 1.0));

    // 进度条填充
    if progress > 0.0 {
        primitives.push(vizuara_core::Primitive::Rectangle {
            min: Point2::new(100.0, 250.0),
            max: Point2::new(100.0 + 600.0 * progress, 270.0),
        });
        styles.push(Style::new().fill_color(Color::rgb(0.2, 0.8, 0.2)));
    }
}

fn generate_keyframe_visual(
    keyframe_anim: &KeyframeAnimation<f32>,
    primitives: &mut Vec<vizuara_core::Primitive>,
    styles: &mut Vec<Style>,
) {
    // 使用lerp函数获取当前值
    let current_value = keyframe_anim.current_value(|from, to, t| from + (to - from) * t).unwrap_or(0.0);
    let progress = keyframe_anim.progress();

    // 调试信息
    println!("🔍 关键帧动画调试: 状态={:?}, 进度={:.3}, 值={:.1}", 
        keyframe_anim.state(), progress, current_value);

    // 状态信息
    let status_text = format!(
        "状态: {:?} | 值: {:.1} | 进度: {:.1}%", 
        keyframe_anim.state(), 
        current_value, 
        progress * 100.0
    );
    primitives.push(vizuara_core::Primitive::Text {
        position: Point2::new(10.0, 100.0),
        content: status_text,
        size: 16.0,
        color: Color::rgb(1.0, 1.0, 0.0),
        h_align: vizuara_core::HorizontalAlign::Left,
        v_align: vizuara_core::VerticalAlign::Top,
    });
    styles.push(Style::new().fill_color(Color::rgb(1.0, 1.0, 0.0)));

    // 简化测试：只显示一个大的动态圆圈
    let ball_x = 100.0 + progress * 600.0; // 水平位置基于进度
    let ball_y = 300.0; // 固定垂直位置
    let ball_radius = 20.0 + (current_value / 100.0) * 15.0; // 半径基于值变化
    
    primitives.push(vizuara_core::Primitive::Circle {
        center: Point2::new(ball_x, ball_y),
        radius: ball_radius,
    });
    styles.push(Style::new()
        .fill_color(Color::rgb(1.0, 0.2, 0.2))
        .stroke(Color::rgb(0.8, 0.0, 0.0), 3.0));

    // 添加进度条
    primitives.push(vizuara_core::Primitive::Rectangle {
        min: Point2::new(100.0, 400.0),
        max: Point2::new(700.0, 420.0),
    });
    styles.push(Style::new()
        .fill_color(Color::rgb(0.3, 0.3, 0.3))
        .stroke(Color::rgb(0.6, 0.6, 0.6), 1.0));

    // 进度条填充
    if progress > 0.0 {
        primitives.push(vizuara_core::Primitive::Rectangle {
            min: Point2::new(100.0, 400.0),
            max: Point2::new(100.0 + 600.0 * progress, 420.0),
        });
        styles.push(Style::new().fill_color(Color::rgb(1.0, 0.5, 0.0)));
    }

    // 测试添加一些简单的装饰圆圈
    for i in 0..5 {
        let x = 150.0 + i as f32 * 120.0;
        let y = 500.0;
        let radius = 5.0 + (progress * 10.0 + i as f32).sin().abs() * 5.0;
        
        primitives.push(vizuara_core::Primitive::Circle {
            center: Point2::new(x, y),
            radius: radius,
        });
        styles.push(Style::new()
            .fill_color(Color::rgb(0.5, 0.8, 0.2))
            .stroke(Color::rgb(0.3, 0.6, 0.0), 2.0));
    }
}

fn generate_timeline_visual(
    timeline: &Timeline,
    primitives: &mut Vec<vizuara_core::Primitive>,
    styles: &mut Vec<Style>,
) {
    let progress = timeline.progress();
    let current_time = timeline.current_time();

    // 调试信息
    println!("🔍 时间轴动画调试: 状态={:?}, 进度={:.3}, 时间={:.2}s", 
        timeline.state(), progress, current_time.as_secs_f32());

    // 状态信息
    let status_text = format!(
        "状态: {:?} | 时间: {:.2}s | 进度: {:.1}%", 
        timeline.state(), 
        current_time.as_secs_f32(),
        progress * 100.0
    );
    primitives.push(vizuara_core::Primitive::Text {
        position: Point2::new(10.0, 100.0),
        content: status_text,
        size: 16.0,
        color: Color::rgb(1.0, 1.0, 0.0),
        h_align: vizuara_core::HorizontalAlign::Left,
        v_align: vizuara_core::VerticalAlign::Top,
    });
    styles.push(Style::new().fill_color(Color::rgb(1.0, 1.0, 0.0)));

    // 简化的时间轴可视化
    let timeline_width = 600.0;
    let timeline_x = 100.0;
    let timeline_y = 250.0;

    // 背景进度条
    primitives.push(vizuara_core::Primitive::Rectangle {
        min: Point2::new(timeline_x, timeline_y),
        max: Point2::new(timeline_x + timeline_width, timeline_y + 20.0),
    });
    styles.push(Style::new()
        .fill_color(Color::rgb(0.3, 0.3, 0.3))
        .stroke(Color::rgb(0.6, 0.6, 0.6), 2.0));

    // 当前进度填充
    if progress > 0.0 {
        primitives.push(vizuara_core::Primitive::Rectangle {
            min: Point2::new(timeline_x, timeline_y),
            max: Point2::new(timeline_x + timeline_width * progress, timeline_y + 20.0),
        });
        styles.push(Style::new().fill_color(Color::rgb(0.2, 0.8, 1.0)));
    }

    // 当前位置指示器 - 大圆圈
    let current_x = timeline_x + timeline_width * progress;
    primitives.push(vizuara_core::Primitive::Circle {
        center: Point2::new(current_x, timeline_y + 10.0),
        radius: 15.0,
    });
    styles.push(Style::new()
        .fill_color(Color::rgb(1.0, 0.5, 0.0))
        .stroke(Color::rgb(0.8, 0.3, 0.0), 3.0));

    // 添加一些简单的装饰元素
    for i in 0..8 {
        let x = timeline_x + i as f32 * (timeline_width / 7.0);
        let y = timeline_y + 60.0 + (progress * 6.28 + i as f32).sin() * 30.0;
        let radius = 8.0 + (progress * 4.0 + i as f32).cos().abs() * 5.0;
        
        let color_t = (i as f32 / 7.0 + progress).fract();
        let color = Color::rgb(color_t, 0.5, 1.0 - color_t);
        
        primitives.push(vizuara_core::Primitive::Circle {
            center: Point2::new(x, y),
            radius: radius,
        });
        styles.push(Style::new()
            .fill_color(color)
            .stroke(Color::rgb(color_t * 0.5, 0.3, (1.0 - color_t) * 0.5), 2.0));
    }
}
