use serde::{Deserialize, Serialize};

/// 数据尺度映射抽象
pub trait Scale {
    /// 将数据值映射到 [0, 1] 范围
    fn normalize(&self, value: f32) -> f32;
    
    /// 将 [0, 1] 范围的值反向映射到数据值
    fn denormalize(&self, normalized: f32) -> f32;
    
    /// 获取合适的刻度值
    fn ticks(&self, count: usize) -> Vec<f32>;
    
    /// 获取刻度的标签
    fn tick_labels(&self, ticks: &[f32]) -> Vec<String>;
}

/// 线性比例尺
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinearScale {
    pub domain_min: f32,
    pub domain_max: f32,
}

impl LinearScale {
    /// 创建新的线性比例尺
    pub fn new(domain_min: f32, domain_max: f32) -> Self {
        Self { domain_min, domain_max }
    }
    
    /// 从数据自动创建比例尺
    pub fn from_data(data: &[f32]) -> Self {
        if data.is_empty() {
            return Self::new(0.0, 1.0);
        }
        
        let min = data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        // 添加一些边距
        let range = max - min;
        let margin = range * 0.05;
        
        Self::new(min - margin, max + margin)
    }
}

impl Scale for LinearScale {
    fn normalize(&self, value: f32) -> f32 {
        if self.domain_max == self.domain_min {
            0.5
        } else {
            (value - self.domain_min) / (self.domain_max - self.domain_min)
        }
    }
    
    fn denormalize(&self, normalized: f32) -> f32 {
        self.domain_min + normalized * (self.domain_max - self.domain_min)
    }
    
    fn ticks(&self, count: usize) -> Vec<f32> {
        if count == 0 {
            return vec![];
        }
        
        let range = self.domain_max - self.domain_min;
        let step = range / (count - 1) as f32;
        
        (0..count)
            .map(|i| self.domain_min + i as f32 * step)
            .collect()
    }
    
    fn tick_labels(&self, ticks: &[f32]) -> Vec<String> {
        ticks.iter().map(|&tick| format!("{:.2}", tick)).collect()
    }
}

/// 对数比例尺
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogScale {
    pub domain_min: f32,
    pub domain_max: f32,
    pub base: f32,
}

impl LogScale {
    /// 创建新的对数比例尺
    pub fn new(domain_min: f32, domain_max: f32, base: f32) -> Self {
        assert!(domain_min > 0.0 && domain_max > 0.0 && base > 0.0 && base != 1.0);
        Self { domain_min, domain_max, base }
    }
    
    /// 创建以10为底的对数比例尺
    pub fn base10(domain_min: f32, domain_max: f32) -> Self {
        Self::new(domain_min, domain_max, 10.0)
    }
}

impl Scale for LogScale {
    fn normalize(&self, value: f32) -> f32 {
        if value <= 0.0 {
            return 0.0;
        }
        
        let log_min = self.domain_min.log(self.base);
        let log_max = self.domain_max.log(self.base);
        let log_value = value.log(self.base);
        
        (log_value - log_min) / (log_max - log_min)
    }
    
    fn denormalize(&self, normalized: f32) -> f32 {
        let log_min = self.domain_min.log(self.base);
        let log_max = self.domain_max.log(self.base);
        let log_value = log_min + normalized * (log_max - log_min);
        
        self.base.powf(log_value)
    }
    
    fn ticks(&self, count: usize) -> Vec<f32> {
        if count == 0 {
            return vec![];
        }
        
        let log_min = self.domain_min.log(self.base);
        let log_max = self.domain_max.log(self.base);
        let log_step = (log_max - log_min) / (count - 1) as f32;
        
        (0..count)
            .map(|i| self.base.powf(log_min + i as f32 * log_step))
            .collect()
    }
    
    fn tick_labels(&self, ticks: &[f32]) -> Vec<String> {
        ticks.iter().map(|&tick| {
            if tick >= 1000.0 || tick < 0.01 {
                format!("{:.1e}", tick)
            } else {
                format!("{:.2}", tick)
            }
        }).collect()
    }
}
