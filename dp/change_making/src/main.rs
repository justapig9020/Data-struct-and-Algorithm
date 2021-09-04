use std::cmp::min;

fn main() {
    let coin = vec![25, 18, 5, 1];
    let mut maker = ChangeMaker::new(coin);
    let change = maker.change(41);
    println!("Chage {} coins", change);
}

struct ChangeMaker {
    coin: Vec<usize>,
    dp: Vec<usize>,
}

impl ChangeMaker {
    fn new(coin: Vec<usize>) -> Self {
        Self {
            coin,
            dp: vec![0],
        }
    }
    fn change(&mut self, amount: usize) -> usize {
        if let Some(total) = self.dp.get(amount) {
            return *total;
        }
        let begin = self.dp.len();
        for i in begin..=amount {
            self.dp.push(i);
            for back in self.coin.iter() {
                if i >= *back {
                    let last = self.dp[i - *back];
                    self.dp[i] = min(self.dp[i], last + 1);
                }
            }
        }
        self.dp[amount]
    }
}
