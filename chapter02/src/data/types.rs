// data/types.rs - 数据类型定义

use std::fmt;

/// 统计数据结构
#[derive(Debug, Clone)]
pub struct Statistics {
    /// 数据点数量
    pub count: usize,
    /// 平均值
    pub mean: f64,
    /// 中位数
    pub median: f64,
    /// 众数（可能有多个）
    pub mode: Vec<f64>,
    /// 方差
    pub variance: f64,
    /// 标准差
    pub std_dev: f64,
    /// 最小值
    pub min: f64,
    /// 最大值
    pub max: f64,
    /// 总和
    pub sum: f64,
}

impl Statistics {
    /// 创建新的统计数据实例
    pub fn new(
        count: usize,
        mean: f64,
        median: f64,
        mode: Vec<f64>,
        variance: f64,
        std_dev: f64,
        min: f64,
        max: f64,
        sum: f64,
    ) -> Self {
        Self {
            count,
            mean,
            median,
            mode,
            variance,
            std_dev,
            min,
            max,
            sum,
        }
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Statistics {{ count: {}, mean: {:.2}, median: {:.2}, std_dev: {:.2}, min: {:.2}, max: {:.2} }}",
            self.count, self.mean, self.median, self.std_dev, self.min, self.max
        )
    }
}

/// 数据点
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

impl DataPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
