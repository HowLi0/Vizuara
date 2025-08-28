use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use winit::event::{ElementState, MouseButton};

/// 鼠标事件类型
#[derive(Debug, Clone, PartialEq)]
pub enum MouseEventType {
    Click,
    DoubleClick,
    Press,
    Release,
    Move,
    Drag,
    Wheel,
    Enter,
    Leave,
}

/// 鼠标事件
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub button: Option<MouseButton>,
    pub position: Point2<f32>,              // 屏幕坐标
    pub data_position: Option<Point2<f32>>, // 数据坐标（如果在绘图区域内）
    pub delta: Option<Point2<f32>>,         // 移动或滚轮增量
    pub modifiers: KeyModifiers,
}

/// 键盘修饰键状态
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool, // Windows 键或 Mac Cmd 键
}

/// 交互事件
#[derive(Debug, Clone)]
pub enum InteractionEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Touch(TouchEvent),
}

/// 键盘事件
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: String,
    pub state: ElementState,
    pub modifiers: KeyModifiers,
}

/// 触摸事件（为移动设备预留）
#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub id: u64,
    pub position: Point2<f32>,
    pub pressure: f32,
    pub phase: TouchPhase,
}

/// 触摸阶段
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl MouseEvent {
    /// 创建新的鼠标事件
    pub fn new(
        event_type: MouseEventType,
        position: Point2<f32>,
        button: Option<MouseButton>,
    ) -> Self {
        Self {
            event_type,
            button,
            position,
            data_position: None,
            delta: None,
            modifiers: KeyModifiers::default(),
        }
    }

    /// 设置数据坐标
    pub fn with_data_position(mut self, data_pos: Point2<f32>) -> Self {
        self.data_position = Some(data_pos);
        self
    }

    /// 设置增量
    pub fn with_delta(mut self, delta: Point2<f32>) -> Self {
        self.delta = Some(delta);
        self
    }

    /// 设置修饰键
    pub fn with_modifiers(mut self, modifiers: KeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    /// 是否是左键点击
    pub fn is_left_click(&self) -> bool {
        matches!(self.event_type, MouseEventType::Click)
            && matches!(self.button, Some(MouseButton::Left))
    }

    /// 是否是右键点击
    pub fn is_right_click(&self) -> bool {
        matches!(self.event_type, MouseEventType::Click)
            && matches!(self.button, Some(MouseButton::Right))
    }

    /// 是否是拖拽事件
    pub fn is_drag(&self) -> bool {
        matches!(self.event_type, MouseEventType::Drag)
    }

    /// 是否在指定区域内
    pub fn is_in_bounds(&self, min: Point2<f32>, max: Point2<f32>) -> bool {
        self.position.x >= min.x
            && self.position.x <= max.x
            && self.position.y >= min.y
            && self.position.y <= max.y
    }
}

impl KeyModifiers {
    /// 创建新的修饰键状态
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 Shift 键状态
    pub fn with_shift(mut self, pressed: bool) -> Self {
        self.shift = pressed;
        self
    }

    /// 设置 Ctrl 键状态
    pub fn with_ctrl(mut self, pressed: bool) -> Self {
        self.ctrl = pressed;
        self
    }

    /// 设置 Alt 键状态
    pub fn with_alt(mut self, pressed: bool) -> Self {
        self.alt = pressed;
        self
    }

    /// 是否没有修饰键被按下
    pub fn is_empty(&self) -> bool {
        !self.shift && !self.ctrl && !self.alt && !self.meta
    }

    /// 是否只按下了指定的修饰键
    pub fn only_shift(&self) -> bool {
        self.shift && !self.ctrl && !self.alt && !self.meta
    }

    pub fn only_ctrl(&self) -> bool {
        !self.shift && self.ctrl && !self.alt && !self.meta
    }

    pub fn only_alt(&self) -> bool {
        !self.shift && !self.ctrl && self.alt && !self.meta
    }
}

/// 事件处理器 trait
pub trait EventHandler {
    /// 处理鼠标事件
    fn handle_mouse_event(&mut self, event: &MouseEvent) -> bool;

    /// 处理键盘事件
    fn handle_keyboard_event(&mut self, event: &KeyboardEvent) -> bool;

    /// 处理交互事件
    fn handle_interaction_event(&mut self, event: &InteractionEvent) -> bool {
        match event {
            InteractionEvent::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            InteractionEvent::Keyboard(keyboard_event) => {
                self.handle_keyboard_event(keyboard_event)
            }
            InteractionEvent::Touch(_) => false, // 暂不处理触摸事件
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_event_creation() {
        let event = MouseEvent::new(
            MouseEventType::Click,
            Point2::new(100.0, 200.0),
            Some(MouseButton::Left),
        );

        assert_eq!(event.event_type, MouseEventType::Click);
        assert_eq!(event.position, Point2::new(100.0, 200.0));
        assert!(event.is_left_click());
        assert!(!event.is_right_click());
    }

    #[test]
    fn test_mouse_event_bounds_check() {
        let event = MouseEvent::new(
            MouseEventType::Click,
            Point2::new(150.0, 250.0),
            Some(MouseButton::Left),
        );

        assert!(event.is_in_bounds(Point2::new(100.0, 200.0), Point2::new(200.0, 300.0)));
        assert!(!event.is_in_bounds(Point2::new(0.0, 0.0), Point2::new(100.0, 100.0)));
    }

    #[test]
    fn test_key_modifiers() {
        let modifiers = KeyModifiers::new().with_shift(true).with_ctrl(false);

        assert!(modifiers.shift);
        assert!(!modifiers.ctrl);
        assert!(modifiers.only_shift());
        assert!(!modifiers.is_empty());
    }

    #[test]
    fn test_drag_event() {
        let event = MouseEvent::new(
            MouseEventType::Drag,
            Point2::new(100.0, 100.0),
            Some(MouseButton::Left),
        )
        .with_delta(Point2::new(10.0, -5.0));

        assert!(event.is_drag());
        assert_eq!(event.delta, Some(Point2::new(10.0, -5.0)));
    }
}
