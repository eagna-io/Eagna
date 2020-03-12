use crate::market::model::num::TipNum;
use chrono::{DateTime, Utc};
use std::collections::{linked_list::Iter, LinkedList};

pub struct PriceHistory {
    in_sec: Series,
    // 将来的に1分刻みの価格なども保存する
}

impl PriceHistory {
    fn new(max_item_in_sec: usize) -> PriceHistory {
        PriceHistory {
            in_sec: Series::second_step(max_item_in_sec),
        }
    }

    pub fn push_item(&mut self, time: &DateTime<Utc>, price: TipNum) {
        self.in_sec.push_item(time, price);
    }
}

impl Default for PriceHistory {
    fn default() -> PriceHistory {
        // 最低でも1時間分は保存しておく
        // 更新頻度が1秒より長くなった場合、履歴は
        // 1時間ぶんよりも長くなることがある。
        PriceHistory::new(60 * 60)
    }
}

pub struct Series {
    list: LinkedList<PriceHistoryItem>,
    max_item: usize,
    // DateTimeを秒や分の整数にroundする関数
    step_time_of: for<'a> fn(&'a DateTime<Utc>) -> i64,
}

impl Series {
    // 1秒刻みのデータシリーズ
    fn second_step(max_item: usize) -> Series {
        Series {
            list: LinkedList::new(),
            max_item,
            step_time_of: |time| time.timestamp(),
        }
    }

    fn push_item(&mut self, time: &DateTime<Utc>, price: TipNum) {
        if self.is_need_push(time) {
            self.list.push_back(PriceHistoryItem::new(*time, price));
            if self.list.len() > self.max_item {
                self.list.pop_front();
            }
        }
    }

    fn is_need_push(&self, time: &DateTime<Utc>) -> bool {
        if let Some(last_item) = self.list.back() {
            (self.step_time_of)(&last_item.time) < (self.step_time_of)(time)
        } else {
            true
        }
    }

    pub fn iter(&self) -> Iter<PriceHistoryItem> {
        self.list.iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PriceHistoryItem {
    time: DateTime<Utc>,
    price: TipNum,
}

impl PriceHistoryItem {
    fn new(time: DateTime<Utc>, price: TipNum) -> PriceHistoryItem {
        PriceHistoryItem { time, price }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outdated_item_should_pruned() {
        // 最大履歴保持数を1で作成
        let mut history = PriceHistory::new(1);

        history.push_item(&Utc::now(), TipNum(42));
        assert_eq!(history.in_sec.list.len(), 1);

        history.push_item(&Utc::now(), TipNum(42));
        assert_eq!(history.in_sec.list.len(), 1);
    }
}
