use std::collections::HashMap;
use winit::event::MouseButton;
use vizuara_core::{error::Result, coords::{LogicalPosition, WorldPosition}};
use crate::viewport::*;

/// 简化的鼠标事件（用于工具系统）
#[derive(Debug, Clone)]
pub enum SimpleMouseEvent {
    ButtonPress { button: MouseButton, position: LogicalPosition },
    ButtonRelease { button: MouseButton, position: LogicalPosition },
    Move { position: LogicalPosition },
    Scroll { delta: f64, position: LogicalPosition },
    DoubleClick { button: MouseButton, position: LogicalPosition },
}

/// 简化的键盘事件（用于工具系统）
#[derive(Debug, Clone)]
pub enum SimpleKeyboardEvent {
    KeyPress { key: String },
    KeyRelease { key: String },
}

/// 交互工具类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolType {
    /// 平移工具
    Pan,
    /// 缩放工具
    Zoom,
    /// 选择工具
    Select,
    /// 测量工具
    Measure,
    /// 重置视图工具
    Reset,
}

/// 工具状态
#[derive(Debug, Clone, PartialEq)]
pub enum ToolState {
    /// 空闲状态
    Idle,
    /// 激活状态（正在使用）
    Active { start_pos: LogicalPosition },
    /// 拖拽状态
    Dragging { 
        start_pos: LogicalPosition, 
        current_pos: LogicalPosition 
    },
}

/// 交互工具的核心trait
pub trait InteractiveTool: std::fmt::Debug {
    /// 处理鼠标事件
    fn handle_mouse_event(&mut self, event: &SimpleMouseEvent, viewport: &mut Viewport) -> Result<bool>;
    
    /// 处理键盘事件
    fn handle_keyboard_event(&mut self, event: &SimpleKeyboardEvent, viewport: &mut Viewport) -> Result<bool>;
    
    /// 获取工具类型
    fn tool_type(&self) -> ToolType;
    
    /// 获取当前状态
    fn state(&self) -> &ToolState;
    
    /// 重置工具状态
    fn reset(&mut self);
    
    /// 检查工具是否处于活动状态
    fn is_active(&self) -> bool {
        !matches!(self.state(), ToolState::Idle)
    }
}

/// 平移工具
#[derive(Debug, Clone)]
pub struct PanTool {
    state: ToolState,
    button: MouseButton,
    sensitivity: f64,
}

impl PanTool {
    /// 创建新的平移工具
    pub fn new() -> Self {
        Self {
            state: ToolState::Idle,
            button: MouseButton::Left,
            sensitivity: 1.0,
        }
    }
    
    /// 设置触发按钮
    pub fn with_button(mut self, button: MouseButton) -> Self {
        self.button = button;
        self
    }
    
    /// 设置灵敏度
    pub fn with_sensitivity(mut self, sensitivity: f64) -> Self {
        self.sensitivity = sensitivity;
        self
    }
}

impl Default for PanTool {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveTool for PanTool {
    fn handle_mouse_event(&mut self, event: &SimpleMouseEvent, viewport: &mut Viewport) -> Result<bool> {
        match event {
            SimpleMouseEvent::ButtonPress { button, position } if *button == self.button => {
                self.state = ToolState::Active { start_pos: *position };
                Ok(true)
            }
            
            SimpleMouseEvent::ButtonRelease { button, .. } if *button == self.button => {
                self.state = ToolState::Idle;
                Ok(true)
            }
            
            SimpleMouseEvent::Move { position } => {
                match &self.state {
                    ToolState::Active { start_pos } => {
                        self.state = ToolState::Dragging { 
                            start_pos: *start_pos, 
                            current_pos: *position 
                        };
                        Ok(true)
                    }
                    ToolState::Dragging { start_pos, current_pos } => {
                        // 计算拖拽偏移
                        let delta = nalgebra::Vector2::new(
                            (position.x - current_pos.x) * self.sensitivity,
                            (position.y - current_pos.y) * self.sensitivity,
                        );
                        
                        // 应用平移
                        viewport.pan(delta)?;
                        
                        // 更新状态
                        self.state = ToolState::Dragging { 
                            start_pos: *start_pos, 
                            current_pos: *position 
                        };
                        
                        Ok(true)
                    }
                    _ => Ok(false)
                }
            }
            
            _ => Ok(false)
        }
    }
    
    fn handle_keyboard_event(&mut self, _event: &SimpleKeyboardEvent, _viewport: &mut Viewport) -> Result<bool> {
        Ok(false)
    }
    
    fn tool_type(&self) -> ToolType {
        ToolType::Pan
    }
    
    fn state(&self) -> &ToolState {
        &self.state
    }
    
    fn reset(&mut self) {
        self.state = ToolState::Idle;
    }
}

/// 缩放工具
#[derive(Debug, Clone)]
pub struct ZoomTool {
    state: ToolState,
    scroll_sensitivity: f64,
    click_zoom_factor: f64,
    button: Option<MouseButton>,
}

impl ZoomTool {
    /// 创建新的缩放工具
    pub fn new() -> Self {
        Self {
            state: ToolState::Idle,
            scroll_sensitivity: 0.1,
            click_zoom_factor: 1.5,
            button: None,
        }
    }
    
    /// 设置滚轮灵敏度
    pub fn with_scroll_sensitivity(mut self, sensitivity: f64) -> Self {
        self.scroll_sensitivity = sensitivity;
        self
    }
    
    /// 设置点击缩放因子
    pub fn with_click_zoom_factor(mut self, factor: f64) -> Self {
        self.click_zoom_factor = factor;
        self
    }
    
    /// 设置点击缩放按钮
    pub fn with_button(mut self, button: MouseButton) -> Self {
        self.button = Some(button);
        self
    }
}

impl Default for ZoomTool {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveTool for ZoomTool {
    fn handle_mouse_event(&mut self, event: &SimpleMouseEvent, viewport: &mut Viewport) -> Result<bool> {
        match event {
            SimpleMouseEvent::Scroll { delta, position } => {
                let zoom_factor = if *delta > 0.0 {
                    1.0 + self.scroll_sensitivity
                } else {
                    1.0 - self.scroll_sensitivity
                };
                
                viewport.zoom_at_point(zoom_factor, *position)?;
                Ok(true)
            }
            
            SimpleMouseEvent::ButtonPress { button, position } 
                if self.button == Some(*button) => {
                viewport.zoom_at_point(self.click_zoom_factor, *position)?;
                Ok(true)
            }
            
            _ => Ok(false)
        }
    }
    
    fn handle_keyboard_event(&mut self, event: &SimpleKeyboardEvent, viewport: &mut Viewport) -> Result<bool> {
        match event {
            SimpleKeyboardEvent::KeyPress { key } => {
                match key.as_str() {
                    "+" | "=" => {
                        let center = LogicalPosition {
                            x: viewport.size().x as f64 / 2.0,
                            y: viewport.size().y as f64 / 2.0,
                        };
                        viewport.zoom_at_point(self.click_zoom_factor, center)?;
                        Ok(true)
                    }
                    "-" => {
                        let center = LogicalPosition {
                            x: viewport.size().x as f64 / 2.0,
                            y: viewport.size().y as f64 / 2.0,
                        };
                        viewport.zoom_at_point(1.0 / self.click_zoom_factor, center)?;
                        Ok(true)
                    }
                    _ => Ok(false)
                }
            }
            _ => Ok(false)
        }
    }
    
    fn tool_type(&self) -> ToolType {
        ToolType::Zoom
    }
    
    fn state(&self) -> &ToolState {
        &self.state
    }
    
    fn reset(&mut self) {
        self.state = ToolState::Idle;
    }
}

/// 选择工具
#[derive(Debug, Clone)]
pub struct SelectTool {
    state: ToolState,
    button: MouseButton,
    selection_rectangle: Option<(WorldPosition, WorldPosition)>,
    selection_threshold: f64,
}

impl SelectTool {
    /// 创建新的选择工具
    pub fn new() -> Self {
        Self {
            state: ToolState::Idle,
            button: MouseButton::Left,
            selection_rectangle: None,
            selection_threshold: 5.0, // 像素
        }
    }
    
    /// 设置选择阈值（像素）
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.selection_threshold = threshold;
        self
    }
    
    /// 获取当前选择矩形（世界坐标）
    pub fn selection_rectangle(&self) -> Option<(WorldPosition, WorldPosition)> {
        self.selection_rectangle
    }
    
    /// 清除选择
    pub fn clear_selection(&mut self) {
        self.selection_rectangle = None;
    }
    
    /// 检查点是否在选择区域内
    pub fn is_point_selected(&self, point: WorldPosition) -> bool {
        if let Some((min_point, max_point)) = self.selection_rectangle {
            point.x >= min_point.x.min(max_point.x)
                && point.x <= min_point.x.max(max_point.x)
                && point.y >= min_point.y.min(max_point.y)
                && point.y <= min_point.y.max(max_point.y)
        } else {
            false
        }
    }
}

impl Default for SelectTool {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveTool for SelectTool {
    fn handle_mouse_event(&mut self, event: &SimpleMouseEvent, viewport: &mut Viewport) -> Result<bool> {
        match event {
            SimpleMouseEvent::ButtonPress { button, position } if *button == self.button => {
                self.state = ToolState::Active { start_pos: *position };
                self.selection_rectangle = None;
                Ok(true)
            }
            
            SimpleMouseEvent::ButtonRelease { button, position } if *button == self.button => {
                if let ToolState::Dragging { start_pos, .. } = self.state {
                    // 完成选择矩形
                    let start_world = viewport.screen_to_world(start_pos);
                    let end_world = viewport.screen_to_world(*position);
                    self.selection_rectangle = Some((start_world, end_world));
                } else if let ToolState::Active { start_pos } = self.state {
                    // 点选择
                    let distance = ((position.x - start_pos.x).powi(2) + 
                                   (position.y - start_pos.y).powi(2)).sqrt();
                    
                    if distance < self.selection_threshold {
                        // 点击选择
                        let world_pos = viewport.screen_to_world(*position);
                        let threshold_world = self.selection_threshold / viewport.zoom_level();
                        self.selection_rectangle = Some((
                            WorldPosition { 
                                x: world_pos.x - threshold_world, 
                                y: world_pos.y - threshold_world 
                            },
                            WorldPosition { 
                                x: world_pos.x + threshold_world, 
                                y: world_pos.y + threshold_world 
                            },
                        ));
                    }
                }
                
                self.state = ToolState::Idle;
                Ok(true)
            }
            
            SimpleMouseEvent::Move { position } => {
                if let ToolState::Active { start_pos } = &self.state {
                    let distance = ((position.x - start_pos.x).powi(2) + 
                                   (position.y - start_pos.y).powi(2)).sqrt();
                    
                    if distance > self.selection_threshold {
                        self.state = ToolState::Dragging { 
                            start_pos: *start_pos, 
                            current_pos: *position 
                        };
                    }
                    Ok(true)
                } else if let ToolState::Dragging { start_pos, .. } = &self.state {
                    self.state = ToolState::Dragging { 
                        start_pos: *start_pos, 
                        current_pos: *position 
                    };
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            
            _ => Ok(false)
        }
    }
    
    fn handle_keyboard_event(&mut self, event: &SimpleKeyboardEvent, _viewport: &mut Viewport) -> Result<bool> {
        match event {
            SimpleKeyboardEvent::KeyPress { key } => {
                match key.as_str() {
                    "Escape" => {
                        self.clear_selection();
                        self.reset();
                        Ok(true)
                    }
                    _ => Ok(false)
                }
            }
            _ => Ok(false)
        }
    }
    
    fn tool_type(&self) -> ToolType {
        ToolType::Select
    }
    
    fn state(&self) -> &ToolState {
        &self.state
    }
    
    fn reset(&mut self) {
        self.state = ToolState::Idle;
    }
}

/// 工具管理器
#[derive(Debug)]
pub struct ToolManager {
    tools: HashMap<ToolType, Box<dyn InteractiveTool>>,
    active_tool: Option<ToolType>,
    default_viewport_bounds: Option<ViewBounds>,
}

impl ToolManager {
    /// 创建新的工具管理器
    pub fn new() -> Self {
        let mut tools: HashMap<ToolType, Box<dyn InteractiveTool>> = HashMap::new();
        
        // 添加默认工具
        tools.insert(ToolType::Pan, Box::new(PanTool::new()));
        tools.insert(ToolType::Zoom, Box::new(ZoomTool::new()));
        tools.insert(ToolType::Select, Box::new(SelectTool::new()));
        
        Self {
            tools,
            active_tool: Some(ToolType::Pan), // 默认激活平移工具
            default_viewport_bounds: None,
        }
    }
    
    /// 设置默认视口边界（用于重置）
    pub fn set_default_viewport_bounds(&mut self, bounds: ViewBounds) {
        self.default_viewport_bounds = Some(bounds);
    }
    
    /// 激活指定工具
    pub fn activate_tool(&mut self, tool_type: ToolType) -> Result<()> {
        if self.tools.contains_key(&tool_type) {
            // 重置当前活动工具
            if let Some(current_tool) = self.active_tool {
                if let Some(tool) = self.tools.get_mut(&current_tool) {
                    tool.reset();
                }
            }
            
            self.active_tool = Some(tool_type);
            Ok(())
        } else {
            Err(format!("工具类型 {:?} 不存在", tool_type).into())
        }
    }
    
    /// 获取当前活动工具
    pub fn active_tool(&self) -> Option<ToolType> {
        self.active_tool
    }
    
    /// 处理鼠标事件
    pub fn handle_mouse_event(&mut self, event: &SimpleMouseEvent, viewport: &mut Viewport) -> Result<bool> {
        // 处理重置工具（双击）
        if let SimpleMouseEvent::DoubleClick { .. } = event {
            if let Some(bounds) = &self.default_viewport_bounds {
                viewport.reset(bounds.clone());
                return Ok(true);
            }
        }
        
        // 处理活动工具事件
        if let Some(tool_type) = self.active_tool {
            if let Some(tool) = self.tools.get_mut(&tool_type) {
                return tool.handle_mouse_event(event, viewport);
            }
        }
        
        Ok(false)
    }
    
    /// 处理键盘事件
    pub fn handle_keyboard_event(&mut self, event: &SimpleKeyboardEvent, viewport: &mut Viewport) -> Result<bool> {
        // 处理工具切换快捷键
        if let SimpleKeyboardEvent::KeyPress { key } = event {
            match key.as_str() {
                "p" | "P" => {
                    self.activate_tool(ToolType::Pan)?;
                    return Ok(true);
                }
                "z" | "Z" => {
                    self.activate_tool(ToolType::Zoom)?;
                    return Ok(true);
                }
                "s" | "S" => {
                    self.activate_tool(ToolType::Select)?;
                    return Ok(true);
                }
                "r" | "R" => {
                    if let Some(bounds) = &self.default_viewport_bounds {
                        viewport.reset(bounds.clone());
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }
        
        // 处理活动工具键盘事件
        if let Some(tool_type) = self.active_tool {
            if let Some(tool) = self.tools.get_mut(&tool_type) {
                return tool.handle_keyboard_event(event, viewport);
            }
        }
        
        Ok(false)
    }
    
    /// 添加自定义工具
    pub fn add_tool(&mut self, tool: Box<dyn InteractiveTool>) {
        let tool_type = tool.tool_type();
        self.tools.insert(tool_type, tool);
    }
    
    /// 移除工具
    pub fn remove_tool(&mut self, tool_type: ToolType) -> Result<()> {
        if self.active_tool == Some(tool_type) {
            self.active_tool = None;
        }
        
        self.tools.remove(&tool_type);
        Ok(())
    }
    
    /// 获取工具状态
    pub fn tool_state(&self, tool_type: ToolType) -> Option<&ToolState> {
        self.tools.get(&tool_type).map(|tool| tool.state())
    }
    
    /// 重置所有工具
    pub fn reset_all_tools(&mut self) {
        for tool in self.tools.values_mut() {
            tool.reset();
        }
    }
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pan_tool() {
        let mut pan_tool = PanTool::new();
        let mut viewport = Viewport::new(800, 600, ViewBounds::new(0.0, 10.0, 0.0, 10.0));
        
        // 测试鼠标按下
        let press_event = SimpleMouseEvent::ButtonPress {
            button: MouseButton::Left,
            position: LogicalPosition { x: 400.0, y: 300.0 },
        };
        
        assert!(pan_tool.handle_mouse_event(&press_event, &mut viewport).unwrap());
        assert!(matches!(pan_tool.state(), ToolState::Active { .. }));
        
        // 测试鼠标移动
        let move_event = SimpleMouseEvent::Move {
            position: LogicalPosition { x: 450.0, y: 350.0 },
        };
        
        assert!(pan_tool.handle_mouse_event(&move_event, &mut viewport).unwrap());
        assert!(matches!(pan_tool.state(), ToolState::Dragging { .. }));
    }

    #[test]
    fn test_zoom_tool() {
        let mut zoom_tool = ZoomTool::new();
        let mut viewport = Viewport::new(800, 600, ViewBounds::new(0.0, 10.0, 0.0, 10.0));
        
        let original_width = viewport.bounds().width();
        
        // 测试滚轮缩放
        let scroll_event = SimpleMouseEvent::Scroll {
            delta: 1.0,
            position: LogicalPosition { x: 400.0, y: 300.0 },
        };
        
        assert!(zoom_tool.handle_mouse_event(&scroll_event, &mut viewport).unwrap());
        
        // 缩放后视图应该变小
        assert!(viewport.bounds().width() < original_width);
    }

    #[test]
    fn test_select_tool() {
        let mut select_tool = SelectTool::new();
        let mut viewport = Viewport::new(800, 600, ViewBounds::new(0.0, 10.0, 0.0, 10.0));
        
        // 测试点选择
        let press_event = SimpleMouseEvent::ButtonPress {
            button: MouseButton::Left,
            position: LogicalPosition { x: 400.0, y: 300.0 },
        };
        
        let release_event = SimpleMouseEvent::ButtonRelease {
            button: MouseButton::Left,
            position: LogicalPosition { x: 402.0, y: 302.0 },
        };
        
        assert!(select_tool.handle_mouse_event(&press_event, &mut viewport).unwrap());
        assert!(select_tool.handle_mouse_event(&release_event, &mut viewport).unwrap());
        
        // 应该有选择区域
        assert!(select_tool.selection_rectangle().is_some());
    }

    #[test]
    fn test_tool_manager() {
        let mut manager = ToolManager::new();
        let mut viewport = Viewport::new(800, 600, ViewBounds::new(0.0, 10.0, 0.0, 10.0));
        
        // 测试工具切换
        assert_eq!(manager.active_tool(), Some(ToolType::Pan));
        
        manager.activate_tool(ToolType::Zoom).unwrap();
        assert_eq!(manager.active_tool(), Some(ToolType::Zoom));
        
        // 测试键盘切换
        let key_event = SimpleKeyboardEvent::KeyPress {
            key: "s".to_string(),
        };
        
        assert!(manager.handle_keyboard_event(&key_event, &mut viewport).unwrap());
        assert_eq!(manager.active_tool(), Some(ToolType::Select));
    }

    #[test]
    fn test_tool_state_transitions() {
        let mut pan_tool = PanTool::new();
        let mut viewport = Viewport::new(800, 600, ViewBounds::new(0.0, 10.0, 0.0, 10.0));
        
        // 开始时应该是空闲状态
        assert!(!pan_tool.is_active());
        
        // 按下鼠标后应该是活动状态
        let press_event = SimpleMouseEvent::ButtonPress {
            button: MouseButton::Left,
            position: LogicalPosition { x: 100.0, y: 100.0 },
        };
        pan_tool.handle_mouse_event(&press_event, &mut viewport).unwrap();
        assert!(pan_tool.is_active());
        
        // 释放鼠标后应该回到空闲状态
        let release_event = SimpleMouseEvent::ButtonRelease {
            button: MouseButton::Left,
            position: LogicalPosition { x: 100.0, y: 100.0 },
        };
        pan_tool.handle_mouse_event(&release_event, &mut viewport).unwrap();
        assert!(!pan_tool.is_active());
    }
}
