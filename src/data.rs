use crate::decks::EMPTY_DECK;
use std::{
    fmt::{self, Display, Formatter},
    ops::AddAssign,
};

#[derive(Default, Debug)]
pub struct Data {
    // 牌数
    decks: u8,
    samples: usize,
    // 缺牌
    has_miss: usize,
    count_miss: usize,
    // 单张
    has_solo: usize,
    count_solo: usize,
    // 对子
    has_pair: usize,
    count_pair: usize,
    // 飞机
    has_trio: usize,
    count_trio: usize,
    // 炸弹
    has_bomb: usize,
    count_bomb: usize,
    // 王炸
    has_nuke: usize,
}

impl Data {
    pub fn new(decks: u8, samples: usize) -> Data {
        Data { decks, samples, ..Data::default() }
    }
    pub fn check(&mut self, cards: &[u8]) {
        // println!("{:?}", cards);
        let mut sorted = cards.to_vec();
        sorted.sort_unstable();
        let mut count = DataCount::default();
        count.check_nuke(&mut sorted, self.decks);
        let mut bin = EMPTY_DECK.clone();
        for n in sorted {
            bin[n as usize - 1] += 1
        }
        // println!("{:?}", bin)
        for n in bin.iter() {
            match n {
                0 => count.count_miss += 1,
                1 => count.count_solo += 1,
                2 => count.count_pair += 1,
                3 => count.count_trio += 1,
                _ => count.count_bomb += 1,
            }
        }
        self.add_assign(count);
    }
}

#[derive(Default, Debug)]
struct DataCount {
    count_miss: isize,
    count_solo: usize,
    count_pair: usize,
    count_trio: usize,
    count_bomb: usize,
    count_nuke: usize,
}

impl DataCount {
    fn check_nuke(&mut self, sorted: &mut Vec<u8>, decks: u8) {
        let len = sorted.len();
        match decks {
            1 => {
                let end = sorted[len - 2..len].iter().map(|u| *u as usize).sum::<usize>();
                if end == 29 {
                    self.count_nuke += 1;
                    sorted.pop();
                    sorted.pop();
                }
            }
            2 => {
                let end = sorted[len - 4..len].iter().map(|u| *u as usize).sum::<usize>();
                if end == 58 {
                    self.count_nuke += 1;
                    sorted.pop();
                    sorted.pop();
                    sorted.pop();
                    sorted.pop();
                }
            }
            3 => (),
            _ => unreachable!(),
        }
    }
}

impl AddAssign<DataCount> for Data {
    fn add_assign(&mut self, rhs: DataCount) {
        if rhs.count_nuke == 0 {
            if rhs.count_miss != 0 {
                self.has_miss += 1;
                self.count_miss += rhs.count_miss as usize
            }
        }
        // 如果有王炸, 缺牌数 -2
        else {
            self.has_nuke += 1;
            if rhs.count_miss - 2 != 0 {
                self.count_miss += (rhs.count_miss - 2) as usize;
                /* 理论上这里不应该溢出
                self.has_miss += 1;
                if rhs.count_miss - 2 > 0 {
                    self.count_miss += (rhs.count_miss - 2) as usize
                }
                */
            }
        }
        if rhs.count_solo != 0 {
            self.has_solo += 1;
            self.count_solo += rhs.count_solo
        }
        if rhs.count_pair != 0 {
            self.has_pair += 1;
            self.count_pair += rhs.count_pair
        }
        if rhs.count_trio != 0 {
            self.has_trio += 1;
            self.count_trio += rhs.count_trio
        }
        if rhs.count_bomb != 0 {
            self.has_bomb += 1;
            self.count_bomb += rhs.count_bomb
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "- 总实验数: {}\n", self.samples)?;
        writeln!(f, "缺牌概率: {:.3} %\n", self.has_miss as f64 / self.samples as f64 * 100.0)?;
        writeln!(f, "缺牌期望: {:.4}\n", self.count_miss as f64 / self.samples as f64)?;
        writeln!(f, "单张概率: {:.3} %\n", self.has_solo as f64 / self.samples as f64 * 100.0)?;
        writeln!(f, "单张期望: {:.4}\n", self.count_solo as f64 / self.samples as f64)?;
        writeln!(f, "对子概率: {:.3} %\n", self.has_pair as f64 / self.samples as f64 * 100.0)?;
        writeln!(f, "对子期望: {:.4}\n", self.count_pair as f64 / self.samples as f64)?;
        writeln!(f, "飞机概率: {:.3} %\n", self.has_trio as f64 / self.samples as f64 * 100.0)?;
        writeln!(f, "飞机期望: {:.4}\n", self.count_trio as f64 / self.samples as f64)?;
        writeln!(f, "炸弹概率: {:.3} %\n", self.has_bomb as f64 / self.samples as f64 * 100.0)?;
        writeln!(f, "炸弹期望: {:.4}\n", self.count_bomb as f64 / self.samples as f64)?;
        writeln!(f, "王炸概率: {:.3} %\n", self.has_nuke as f64 / self.samples as f64 * 100.0)?;
        writeln!(f, "王炸期望: {:.4}\n", self.has_nuke as f64 / self.samples as f64)?;
        Ok(())
    }
}
