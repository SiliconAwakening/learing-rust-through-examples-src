// history/mod.rs - 历史记录管理模块

use std::fmt;

/// 历史记录条目
#[derive(Debug, Clone)]
pub struct HistoryRecord {
    /// 表达式字符串
    pub expression: String,
    /// 计算结果
    pub result: f64,
    /// 记录索引
    pub index: usize,
}

impl HistoryRecord {
    /// 创建新的历史记录
    pub fn new(expression: String, result: f64, index: usize) -> Self {
        Self {
            expression,
            result,
            index,
        }
    }
}

impl fmt::Display for HistoryRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} = {}", self.index, self.expression, self.result)
    }
}

/// 历史记录管理器
pub struct HistoryManager {
    records: Vec<HistoryRecord>,
    max_records: usize,
    next_index: usize,
}

impl HistoryManager {
    /// 创建新的历史记录管理器
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// 创建指定容量的历史记录管理器
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            records: Vec::new(),
            max_records: capacity,
            next_index: 1,
        }
    }

    /// 添加新的历史记录
    pub fn add_record(&mut self, expression: &str, result: f64) {
        let record = HistoryRecord::new(expression.to_string(), result, self.next_index);

        self.records.push(record);
        self.next_index += 1;

        // 保持记录数量限制
        if self.records.len() > self.max_records {
            self.records.remove(0);
        }
    }

    /// 获取最近的 n 条记录
    pub fn get_recent_records(&self, count: usize) -> &[HistoryRecord] {
        let start = if self.records.len() > count {
            self.records.len() - count
        } else {
            0
        };
        &self.records[start..]
    }

    /// 获取所有记录
    pub fn get_all_records(&self) -> &[HistoryRecord] {
        &self.records
    }

    /// 搜索包含指定查询字符串的记录
    pub fn search_records(&self, query: &str) -> Vec<&HistoryRecord> {
        self.records
            .iter()
            .filter(|record| {
                record.expression.contains(query) || record.result.to_string().contains(query)
            })
            .collect()
    }

    /// 获取指定索引的记录
    pub fn get_record_by_index(&self, index: usize) -> Option<&HistoryRecord> {
        self.records.iter().find(|r| r.index == index)
    }

    /// 清除所有历史记录
    pub fn clear(&mut self) {
        self.records.clear();
        self.next_index = 1;
    }

    /// 获取记录总数
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// 显示历史记录
    pub fn display(&self) {
        if self.records.is_empty() {
            println!("暂无计算历史");
            return;
        }

        println!("\n=== 计算历史 ===");
        for record in &self.records {
            println!("{}", record);
        }
        println!("===============\n");
    }

    /// 显示最近的 n 条记录
    pub fn display_recent(&self, count: usize) {
        let records = self.get_recent_records(count);

        if records.is_empty() {
            println!("暂无计算历史");
            return;
        }

        println!("\n=== 最近 {} 条记录 ===", records.len());
        for record in records {
            println!("{}", record);
        }
        println!("=====================\n");
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_record() {
        let mut manager = HistoryManager::new();
        manager.add_record("1 + 1", 2.0);
        manager.add_record("2 * 3", 6.0);

        assert_eq!(manager.count(), 2);
    }

    #[test]
    fn test_max_capacity() {
        let mut manager = HistoryManager::with_capacity(3);

        manager.add_record("1 + 1", 2.0);
        manager.add_record("2 + 2", 4.0);
        manager.add_record("3 + 3", 6.0);
        manager.add_record("4 + 4", 8.0);

        // 应该只保留最后 3 条
        assert_eq!(manager.count(), 3);

        let records = manager.get_all_records();
        assert_eq!(records[0].result, 4.0);
        assert_eq!(records[2].result, 8.0);
    }

    #[test]
    fn test_search() {
        let mut manager = HistoryManager::new();
        manager.add_record("1 + 1", 2.0);
        manager.add_record("2 * 3", 6.0);
        manager.add_record("10 + 5", 15.0);

        let results = manager.search_records("+");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut manager = HistoryManager::new();
        manager.add_record("1 + 1", 2.0);
        manager.add_record("2 + 2", 4.0);

        assert_eq!(manager.count(), 2);

        manager.clear();
        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_get_recent() {
        let mut manager = HistoryManager::new();
        for i in 1..=10 {
            manager.add_record(&format!("{} + {}", i, i), (i * 2) as f64);
        }

        let recent = manager.get_recent_records(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].result, 16.0); // 8 + 8
        assert_eq!(recent[2].result, 20.0); // 10 + 10
    }
}
