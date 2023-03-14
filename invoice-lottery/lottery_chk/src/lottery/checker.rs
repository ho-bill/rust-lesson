use regex::Regex;

pub struct WinningPair {
    number: String,
    winning: u32,
}

impl WinningPair {
    pub fn new(number: String, winning: u32) -> Self {
        WinningPair { number, winning }
    }

    pub fn re_str(&self) -> String {
        format!("{}{}", self.number, "$")
    }
}

pub fn check_winning(winning: &WinningPair, num: u32) -> u32 {
    let mut winner = 0;
    let re = Regex::new(&winning.re_str()).unwrap();
    if re.is_match(&format!("{:08}", num).to_string()) {
        winner = winning.winning;
    }
    winner
}