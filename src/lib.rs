const NUMS: &[char] = &['零', '壹', '贰', '叁', '肆', '伍', '陆', '柒', '捌', '玖'];

const POS_001: char = '分';
const POS_01: char = '角';
const POS_0: char = '圆';
const POS_10: char = '拾';
const POS_100: char = '佰';
const POS_1000: char = '仟';

const POS_10K: char = '万';
const POS_1M: char = '亿';

const NUM_EXACTLY: char = '整';

const MINUS: char = '负';

pub trait ChineseCurrency {
    fn to_chinese_currency(self) -> String;
}

impl ChineseCurrency for isize {
    fn to_chinese_currency(self) -> String {
        let (value, is_minus) = if self < 0 {
            (-self as usize, true)
        } else {
            (self as usize, false)
        };

        Builder::new(value, is_minus).build()
    }
}

struct Builder {
    position_groups: PositionGroups,
    pos_01: usize,
    pos_001: usize,
    is_minus: bool,
}

struct PositionGroups {
    position_groups: Vec<PositionGroup>,
}

struct PositionGroup {
    pos_1: usize,
    pos_10: usize,
    pos_100: usize,
    pos_1000: usize,
    rank: Rank,
}

struct Rank {
    rank: u8,
}

impl Builder {
    fn new(mut i: usize, is_minus: bool) -> Builder {
        let pos_001 = i % 10;
        i /= 10;

        let pos_01 = i % 10;
        i /= 10;

        let position_groups = PositionGroups::new(i);

        Builder {
            position_groups,
            pos_01,
            pos_001,
            is_minus,
        }
    }

    fn build(self) -> String {
        let mut buf = String::default();

        if self.is_minus {
            buf.push(MINUS);
        }

        let is_empty = self.position_groups.build(&mut buf);

        let mut exactly = if self.pos_01 > 0 {
            buf.push(NUMS[self.pos_01]);
            buf.push(POS_01);
            false
        } else {
            true
        };

        if self.pos_001 > 0 {
            if exactly {
                buf.push(NUMS[self.pos_01]);
            }

            buf.push(NUMS[self.pos_001]);
            buf.push(POS_001);
            exactly = false;
        }

        if exactly {
            if is_empty {
                buf.push(NUMS[0]);
                buf.push(POS_0);
            }
            buf.push(NUM_EXACTLY);
        }

        buf
    }
}

impl PositionGroups {
    fn new(mut i: usize) -> PositionGroups {
        let mut position_groups = Vec::new();

        let mut rank = 0;
        while i > 0 {
            let pos_1 = i % 10;
            i /= 10;

            let pos_10 = i % 10;
            i /= 10;

            let pos_100 = i % 10;
            i /= 10;

            let pos_1000 = i % 10;
            i /= 10;

            position_groups.push(PositionGroup {
                pos_1,
                pos_10,
                pos_100,
                pos_1000,
                rank: Rank { rank },
            });
            rank += 1;
        }

        PositionGroups { position_groups }
    }

    fn build(&self, buf: &mut String) -> bool {
        if self.position_groups.is_empty() {
            true
        } else {
            let mut group_iter = self.position_groups.iter().rev();
            if let Some(position_group) = group_iter.next() {
                position_group.build(buf, false);
            }

            for position_group in group_iter {
                position_group.build(buf, true);
            }
            buf.push(POS_0);
            false
        }
    }
}

impl PositionGroup {
    fn build(&self, buf: &mut String, show_zero: bool) {
        let mut is_rendered = false;

        if self.pos_1000 > 0 {
            buf.push(NUMS[self.pos_1000]);
            buf.push(POS_1000);
            is_rendered = true;
        } else if show_zero {
            buf.push(NUMS[self.pos_1000]);
        }

        if self.pos_100 > 0 {
            buf.push(NUMS[self.pos_100]);
            buf.push(POS_100);
            is_rendered = true;
        }

        if self.pos_10 > 0 {
            if is_rendered && self.pos_100 == 0 {
                buf.push(NUMS[self.pos_100]);
            }
            buf.push(NUMS[self.pos_10]);
            buf.push(POS_10);
            is_rendered = true;
        }

        if self.pos_1 > 0 {
            if is_rendered && self.pos_10 == 0 {
                buf.push(NUMS[self.pos_10]);
            }
            buf.push(NUMS[self.pos_1]);
            is_rendered = true;
        }

        if is_rendered {
            buf.push_str(&self.rank.to_string());
        } else {
            buf.pop();
        }
    }
}

impl Rank {
    fn to_string(&self) -> String {
        Rank::rank_to_string(self.rank)
    }

    fn rank_to_string(rank: u8) -> String {
        match rank {
            0 => "".into(),
            1 => POS_10K.to_string(),
            _ => {
                let m = rank % 2;
                let d = rank / 2;
                let mut s = String::with_capacity(d as usize + 1);
                if m == 1 {
                    s.push(POS_10K);
                }
                for _ in 0..d {
                    s.push(POS_1M);
                }
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChineseCurrency, Rank};

    #[test]
    fn smoke_rank() {
        assert_eq!(Rank::rank_to_string(0), "");
        assert_eq!(Rank::rank_to_string(1), "万");
        assert_eq!(Rank::rank_to_string(2), "亿");
        assert_eq!(Rank::rank_to_string(3), "万亿");
        assert_eq!(Rank::rank_to_string(4), "亿亿");
        assert_eq!(Rank::rank_to_string(5), "万亿亿");
        assert_eq!(Rank::rank_to_string(6), "亿亿亿");
    }

    #[test]
    fn smoke_to_chinese_currency() {
        assert_eq!(0.to_chinese_currency(), "零圆整");
        assert_eq!(1.to_chinese_currency(), "零壹分");
        assert_eq!(21.to_chinese_currency(), "贰角壹分");
        assert_eq!(20.to_chinese_currency(), "贰角");
        assert_eq!(321.to_chinese_currency(), "叁圆贰角壹分");
        assert_eq!(300.to_chinese_currency(), "叁圆整");
        assert_eq!(4321.to_chinese_currency(), "肆拾叁圆贰角壹分");
        assert_eq!(54321.to_chinese_currency(), "伍佰肆拾叁圆贰角壹分");
        assert_eq!(654321.to_chinese_currency(), "陆仟伍佰肆拾叁圆贰角壹分");
        assert_eq!(
            7654321.to_chinese_currency(),
            "柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
        assert_eq!(
            87654321.to_chinese_currency(),
            "捌拾柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
        assert_eq!(
            987654321.to_chinese_currency(),
            "玖佰捌拾柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
        assert_eq!(
            8987654321.to_chinese_currency(),
            "捌仟玖佰捌拾柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
        assert_eq!(
            78987654321.to_chinese_currency(),
            "柒亿捌仟玖佰捌拾柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
        assert_eq!(
            70987654321.to_chinese_currency(),
            "柒亿零玖佰捌拾柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
        assert_eq!(
            (-70987654321).to_chinese_currency(),
            "负柒亿零玖佰捌拾柒万陆仟伍佰肆拾叁圆贰角壹分"
        );
    }
}
