use crate::market::model::num::TipNum;
use chrono::{DateTime, Utc};
use std::collections::{linked_list::Iter, LinkedList};

pub struct PriceHistory {
    // 直近の1秒刻みの価格の推移履歴
    in_sec: LinkedList<PriceHistoryItem>,
    // 1秒刻みの価格レコードの最大保持数
    max_item_in_sec: usize,
    // 将来的に1分刻みの価格なども保存する
}

impl PriceHistory {
    fn new(max_item_in_sec: usize) -> PriceHistory {
        PriceHistory {
            in_sec: LinkedList::new(),
            max_item_in_sec,
        }
    }

    pub fn push_item(&mut self, time: &DateTime<Utc>, price: TipNum) {
        self.push_item_to_in_sec(time, price);
    }

    fn push_item_to_in_sec(&mut self, time: &DateTime<Utc>, price: TipNum) {
        if self.is_need_push(time) {
            self.in_sec.push_back(PriceHistoryItem::new(*time, price));
            if self.in_sec.len() > self.max_item_in_sec {
                self.in_sec.pop_front();
            }
        }
    }

    fn is_need_push(&self, time: &DateTime<Utc>) -> bool {
        if let Some(last_item) = self.in_sec.back() {
            seconds_of(&last_item.time) < seconds_of(time)
        } else {
            true
        }
    }

    pub fn iter_in_sec(&self) -> Iter<PriceHistoryItem> {
        self.in_sec.iter()
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

fn seconds_of(time: &DateTime<Utc>) -> i64 {
    time.timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outdated_item_should_pruned() {
        // 最大履歴保持数を1で作成
        let mut history = PriceHistory::new(1);

        history.push_item(&Utc::now(), TipNum(42));
        assert_eq!(history.in_sec.len(), 1);

        history.push_item(&Utc::now(), TipNum(42));
        assert_eq!(history.in_sec.len(), 1);
    }
}
