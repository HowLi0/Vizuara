use crate::{Theme, ThemeError, ThemePresets, ThemeResult};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// 全局主题管理器
///
/// 单例模式的主题管理器，负责主题的注册、加载、切换和持久化
pub struct ThemeManager {
    /// 当前活跃的主题
    current_theme: Arc<RwLock<Theme>>,
    /// 已注册的主题集合
    themes: Arc<RwLock<HashMap<String, Theme>>>,
    /// 主题文件存储路径
    theme_dir: Option<PathBuf>,
}

/// 全局主题管理器实例
static THEME_MANAGER: Lazy<ThemeManager> = Lazy::new(ThemeManager::new);

impl ThemeManager {
    /// 创建新的主题管理器
    fn new() -> Self {
        let mut manager = Self {
            current_theme: Arc::new(RwLock::new(ThemePresets::default_theme())),
            themes: Arc::new(RwLock::new(HashMap::new())),
            theme_dir: None,
        };

        // 注册所有预设主题
        manager.register_preset_themes();
        manager
    }

    /// 获取全局主题管理器实例
    pub fn instance() -> &'static ThemeManager {
        &THEME_MANAGER
    }

    /// 设置主题文件存储目录
    pub fn set_theme_directory(&mut self, dir: impl AsRef<Path>) -> ThemeResult<()> {
        let path = dir.as_ref().to_path_buf();

        // 创建目录（如果不存在）
        if !path.exists() {
            fs::create_dir_all(&path)
                .map_err(|e| ThemeError::IoError(format!("无法创建主题目录: {}", e)))?;
        }

        self.theme_dir = Some(path);
        Ok(())
    }

    /// 注册预设主题
    fn register_preset_themes(&mut self) {
        let mut themes = self.themes.write().unwrap();

        for name in ThemePresets::list_preset_names() {
            if let Some(theme) = ThemePresets::get_preset(name) {
                themes.insert(name.to_string(), theme);
            }
        }
    }

    /// 获取当前主题
    pub fn current_theme(&self) -> Theme {
        self.current_theme.read().unwrap().clone()
    }

    /// 切换到指定主题
    pub fn switch_theme(&self, theme_name: &str) -> ThemeResult<()> {
        let themes = self.themes.read().unwrap();

        if let Some(theme) = themes.get(theme_name) {
            let mut current = self.current_theme.write().unwrap();
            *current = theme.clone();

            // 保存当前主题设置到配置文件
            if let Err(e) = self.save_current_theme_config() {
                eprintln!("保存主题配置失败: {}", e);
            }

            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound(theme_name.to_string()))
        }
    }

    /// 注册自定义主题
    pub fn register_theme(&self, theme: Theme) -> ThemeResult<()> {
        // 验证主题
        theme.validate()?;

        let mut themes = self.themes.write().unwrap();
        themes.insert(theme.name.clone(), theme);

        Ok(())
    }

    /// 获取已注册的主题名称列表
    pub fn list_themes(&self) -> Vec<String> {
        let themes = self.themes.read().unwrap();
        themes.keys().cloned().collect()
    }

    /// 获取指定主题
    pub fn get_theme(&self, name: &str) -> Option<Theme> {
        let themes = self.themes.read().unwrap();
        themes.get(name).cloned()
    }

    /// 从文件加载主题
    pub fn load_theme_from_file(&self, file_path: impl AsRef<Path>) -> ThemeResult<Theme> {
        let content = fs::read_to_string(file_path.as_ref())
            .map_err(|e| ThemeError::IoError(format!("读取主题文件失败: {}", e)))?;

        let theme: Theme = toml::from_str(&content)
            .map_err(|e| ThemeError::ParseError(format!("解析主题文件失败: {}", e)))?;

        // 验证主题
        theme.validate()?;

        Ok(theme)
    }

    /// 将主题保存到文件
    pub fn save_theme_to_file(
        &self,
        theme: &Theme,
        file_path: impl AsRef<Path>,
    ) -> ThemeResult<()> {
        // 验证主题
        theme.validate()?;

        let content = toml::to_string_pretty(theme)
            .map_err(|e| ThemeError::ParseError(format!("序列化主题失败: {}", e)))?;

        fs::write(file_path.as_ref(), content)
            .map_err(|e| ThemeError::IoError(format!("写入主题文件失败: {}", e)))?;

        Ok(())
    }

    /// 加载指定目录中的所有主题文件
    pub fn load_themes_from_directory(&self, dir: impl AsRef<Path>) -> ThemeResult<usize> {
        let dir_path = dir.as_ref();

        if !dir_path.exists() {
            return Err(ThemeError::IoError(format!(
                "主题目录不存在: {:?}",
                dir_path
            )));
        }

        let mut loaded_count = 0;

        let entries = fs::read_dir(dir_path)
            .map_err(|e| ThemeError::IoError(format!("读取主题目录失败: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| ThemeError::IoError(format!("读取目录项失败: {}", e)))?;

            let path = entry.path();

            // 只处理 .toml 文件
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match self.load_theme_from_file(&path) {
                    Ok(theme) => {
                        if let Err(e) = self.register_theme(theme) {
                            eprintln!("注册主题失败 {:?}: {}", path, e);
                        } else {
                            loaded_count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("加载主题失败 {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(loaded_count)
    }

    /// 保存当前主题配置
    fn save_current_theme_config(&self) -> ThemeResult<()> {
        if let Some(ref theme_dir) = self.theme_dir {
            let config_path = theme_dir.join("current_theme.toml");
            let current = self.current_theme.read().unwrap();

            let config = CurrentThemeConfig {
                theme_name: current.name.clone(),
                last_updated: chrono::Utc::now().to_rfc3339(),
            };

            let content = toml::to_string_pretty(&config)
                .map_err(|e| ThemeError::ParseError(format!("序列化配置失败: {}", e)))?;

            fs::write(config_path, content)
                .map_err(|e| ThemeError::IoError(format!("保存配置失败: {}", e)))?;
        }

        Ok(())
    }

    /// 加载上次使用的主题
    pub fn load_last_theme(&self) -> ThemeResult<()> {
        if let Some(ref theme_dir) = self.theme_dir {
            let config_path = theme_dir.join("current_theme.toml");

            if config_path.exists() {
                let content = fs::read_to_string(config_path)
                    .map_err(|e| ThemeError::IoError(format!("读取配置失败: {}", e)))?;

                let config: CurrentThemeConfig = toml::from_str(&content)
                    .map_err(|e| ThemeError::ParseError(format!("解析配置失败: {}", e)))?;

                self.switch_theme(&config.theme_name)?;
            }
        }

        Ok(())
    }

    /// 创建主题的副本（用于自定义）
    pub fn duplicate_theme(&self, source_name: &str, new_name: &str) -> ThemeResult<Theme> {
        let themes = self.themes.read().unwrap();

        if let Some(source_theme) = themes.get(source_name) {
            let mut new_theme = source_theme.clone();
            new_theme.name = new_name.to_string();
            new_theme.parent = Some(source_name.to_string());
            new_theme.version = "1.0.0".to_string();

            Ok(new_theme)
        } else {
            Err(ThemeError::ThemeNotFound(source_name.to_string()))
        }
    }

    /// 删除自定义主题
    pub fn remove_theme(&self, theme_name: &str) -> ThemeResult<()> {
        // 不能删除预设主题
        if ThemePresets::list_preset_names().contains(&theme_name) {
            return Err(ThemeError::InvalidTheme("不能删除预设主题".to_string()));
        }

        let mut themes = self.themes.write().unwrap();

        if themes.remove(theme_name).is_some() {
            // 如果删除的是当前主题，切换到默认主题
            let current_name = self.current_theme.read().unwrap().name.clone();
            if current_name == theme_name {
                drop(themes); // 释放锁
                self.switch_theme("default")?;
            }

            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound(theme_name.to_string()))
        }
    }

    /// 重置到默认主题
    pub fn reset_to_default(&self) -> ThemeResult<()> {
        self.switch_theme("default")
    }

    /// 获取主题统计信息
    pub fn get_theme_stats(&self) -> ThemeStats {
        let themes = self.themes.read().unwrap();
        let current = self.current_theme.read().unwrap();

        let preset_count = ThemePresets::list_preset_names().len();
        let custom_count = themes.len() - preset_count;

        ThemeStats {
            total_themes: themes.len(),
            preset_themes: preset_count,
            custom_themes: custom_count,
            current_theme: current.name.clone(),
        }
    }
}

/// 当前主题配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CurrentThemeConfig {
    theme_name: String,
    last_updated: String,
}

/// 主题统计信息
#[derive(Debug, Clone)]
pub struct ThemeStats {
    /// 总主题数
    pub total_themes: usize,
    /// 预设主题数
    pub preset_themes: usize,
    /// 自定义主题数
    pub custom_themes: usize,
    /// 当前主题名称
    pub current_theme: String,
}

/// 主题管理器的便捷函数
impl ThemeManager {
    /// 应用主题到样式（便捷方法）
    pub fn apply_current_theme(
        &self,
        component_type: &crate::ComponentType,
        base_style: vizuara_core::Style,
    ) -> vizuara_core::Style {
        let current = self.current_theme.read().unwrap();
        current.apply_to_style(component_type, base_style)
    }

    /// 获取当前主题的组件颜色
    pub fn get_current_primary_color(
        &self,
        component_type: &crate::ComponentType,
    ) -> vizuara_core::Color {
        let current = self.current_theme.read().unwrap();
        current.get_primary_color(component_type)
    }

    /// 获取当前背景颜色
    pub fn get_current_background_color(&self) -> vizuara_core::Color {
        let current = self.current_theme.read().unwrap();
        current.get_background_color()
    }

    /// 获取当前文本颜色
    pub fn get_current_text_color(&self) -> vizuara_core::Color {
        let current = self.current_theme.read().unwrap();
        current.get_text_color()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_theme_manager_singleton() {
        let manager1 = ThemeManager::instance();
        let manager2 = ThemeManager::instance();

        // 应该是同一个实例
        assert!(std::ptr::eq(manager1, manager2));
    }

    #[test]
    fn test_switch_theme() {
        let manager = ThemeManager::instance();

        // 切换到深色主题
        assert!(manager.switch_theme("dark").is_ok());
        assert_eq!(manager.current_theme().name, "dark");

        // 切换到不存在的主题应该失败
        assert!(manager.switch_theme("nonexistent").is_err());
    }

    #[test]
    fn test_register_custom_theme() {
        let manager = ThemeManager::instance();

        let custom_theme = crate::Theme::new("test_custom", "Test custom theme");
        assert!(manager.register_theme(custom_theme).is_err()); // 应该失败，因为缺少必需属性

        let mut valid_theme = crate::Theme::new("test_valid", "Test valid theme");
        valid_theme.set_global(
            crate::ThemeProperty::PrimaryColor,
            crate::ThemeValue::Color(vizuara_core::Color::rgb(1.0, 0.0, 0.0)),
        );
        valid_theme.set_global(
            crate::ThemeProperty::BackgroundColor,
            crate::ThemeValue::Color(vizuara_core::Color::rgb(1.0, 1.0, 1.0)),
        );
        valid_theme.set_global(
            crate::ThemeProperty::TextColor,
            crate::ThemeValue::Color(vizuara_core::Color::rgb(0.0, 0.0, 0.0)),
        );

        assert!(manager.register_theme(valid_theme).is_ok());
        assert!(manager.list_themes().contains(&"test_valid".to_string()));
    }

    #[test]
    fn test_theme_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ThemeManager::instance();

        // 设置主题目录
        let mut manager_copy = ThemeManager::new();
        assert!(manager_copy.set_theme_directory(temp_dir.path()).is_ok());

        // 创建有效主题
        let mut theme = crate::Theme::new("file_test", "Test file theme");
        theme.set_global(
            crate::ThemeProperty::PrimaryColor,
            crate::ThemeValue::Color(vizuara_core::Color::rgb(1.0, 0.0, 0.0)),
        );
        theme.set_global(
            crate::ThemeProperty::BackgroundColor,
            crate::ThemeValue::Color(vizuara_core::Color::rgb(1.0, 1.0, 1.0)),
        );
        theme.set_global(
            crate::ThemeProperty::TextColor,
            crate::ThemeValue::Color(vizuara_core::Color::rgb(0.0, 0.0, 0.0)),
        );

        // 保存主题到文件
        let theme_path = temp_dir.path().join("test_theme.toml");
        assert!(manager.save_theme_to_file(&theme, &theme_path).is_ok());

        // 从文件加载主题
        let loaded_theme = manager.load_theme_from_file(&theme_path).unwrap();
        assert_eq!(loaded_theme.name, "file_test");
    }

    #[test]
    fn test_duplicate_theme() {
        let manager = ThemeManager::instance();

        let duplicated = manager.duplicate_theme("default", "my_default").unwrap();
        assert_eq!(duplicated.name, "my_default");
        assert_eq!(duplicated.parent, Some("default".to_string()));
    }

    #[test]
    fn test_list_themes() {
        let manager = ThemeManager::instance();
        let themes = manager.list_themes();

        // 应该包含所有预设主题
        for preset_name in ThemePresets::list_preset_names() {
            assert!(themes.contains(&preset_name.to_string()));
        }
    }

    #[test]
    fn test_theme_stats() {
        let manager = ThemeManager::instance();
        let stats = manager.get_theme_stats();

        assert!(stats.total_themes > 0);
        assert!(stats.preset_themes > 0);
        assert_eq!(
            stats.total_themes,
            stats.preset_themes + stats.custom_themes
        );
        assert!(!stats.current_theme.is_empty());
    }

    #[test]
    fn test_convenience_methods() {
        let manager = ThemeManager::instance();

        // 测试便捷方法
        let bg_color = manager.get_current_background_color();
        let text_color = manager.get_current_text_color();
        let primary_color = manager.get_current_primary_color(&crate::ComponentType::ScatterPlot);

        // 颜色应该是有效的
        assert!(bg_color.r >= 0.0 && bg_color.r <= 1.0);
        assert!(text_color.g >= 0.0 && text_color.g <= 1.0);
        assert!(primary_color.b >= 0.0 && primary_color.b <= 1.0);
    }
}
