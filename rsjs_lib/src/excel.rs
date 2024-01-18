pub struct UsFiat {
    pennies: f64 ,
    nickels: f64,
    dimes: f64,
    quarters: f64,
    half_dollars: f64,
    dollars: f64,
    two_dollars: f64,
    five_dollars: f64,
    ten_dollars: f64,
    twenty_dollars: f64,
    fifty_dollars: f64,
    hundred_dollars: f64,
    pennies_count: i64,
    nickels_count: i64,
    dimes_count: i64,
    quarters_count: i64,
    half_dollars_count: i64,
    dollars_count: i64,
    two_dollars_count: i64,
    five_dollars_count: i64,
    ten_dollars_count: i64,
    twenty_dollars_count: i64,
    fifty_dollars_count: i64,
    hundred_dollars_count: i64,
    value: f64,
}
impl UsFiat {
    pub fn new() -> UsFiat {
        UsFiat {
            pennies: 0.01,
            nickels: 0.05,
            dimes: 0.10,
            quarters: 0.25,
            half_dollars: 0.50,
            dollars: 1.00,
            two_dollars: 2.00,
            five_dollars: 5.00,
            ten_dollars: 10.00,
            twenty_dollars: 20.00,
            fifty_dollars: 50.00,
            hundred_dollars: 100.00,
            pennies_count: 0,
            nickels_count: 0,
            dimes_count: 0,
            quarters_count: 0,
            half_dollars_count: 0,
            dollars_count: 0,
            two_dollars_count: 0,
            five_dollars_count: 0,
            ten_dollars_count: 0,
            twenty_dollars_count: 0,
            fifty_dollars_count: 0,
            hundred_dollars_count: 0,
            value: 0.0,
        }
    }
    pub fn add_pennies(&mut self, count: i64) {
        self.pennies_count += count;
        self.value += self.pennies * count as f64;
    }
    pub fn add_nickels(&mut self, count: i64) {
        self.nickels_count += count;
        self.value += self.nickels * count as f64;
    }
    pub fn add_dimes(&mut self, count: i64) {
        self.dimes_count += count;
        self.value += self.dimes * count as f64;
    }
    pub fn add_quarters(&mut self, count: i64) {
        self.quarters_count += count;
        self.value += self.quarters * count as f64;
    }
    pub fn add_half_dollars(&mut self, count: i64) {
        self.half_dollars_count += count;
        self.value += self.half_dollars * count as f64;
    }
    pub fn add_dollars(&mut self, count: i64) {
        self.dollars_count += count;
        self.value += self.dollars * count as f64;
    }
    pub fn add_two_dollars(&mut self, count: i64) {
        self.two_dollars_count += count;
        self.value += self.two_dollars * count as f64;
    }
    pub fn add_five_dollars(&mut self, count: i64) {
        self.five_dollars_count += count;
        self.value += self.five_dollars * count as f64;
    }
    pub fn add_ten_dollars(&mut self, count: i64) {
        self.ten_dollars_count += count;
        self.value += self.ten_dollars * count as f64;
    }
    pub fn add_twenty_dollars(&mut self, count: i64) {
        self.twenty_dollars_count += count;
        self.value += self.twenty_dollars * count as f64;
    }
    pub fn add_fifty_dollars(&mut self, count: i64) {
        self.fifty_dollars_count += count;
        self.value += self.fifty_dollars * count as f64;
    }
    pub fn add_hundred_dollars(&mut self, count: i64) {
        self.hundred_dollars_count += count;
        self.value += self.hundred_dollars * count as f64;
    }
    pub fn get_pennies(&self) -> i64 {
        self.pennies_count
    }
    pub fn get_nickels(&self) -> i64 {
        self.nickels_count
    }
    pub fn get_dimes(&self) -> i64 {
        self.dimes_count
    }
    pub fn get_quarters(&self) -> i64 {
        self.quarters_count
    }
    pub fn get_half_dollars(&self) -> i64 {
        self.half_dollars_count
    }
    pub fn get_dollars(&self) -> i64 {
        self.dollars_count
    }
    pub fn get_two_dollars(&self) -> i64 {
        self.two_dollars_count
    }
    pub fn get_five_dollars(&self) -> i64 {
        self.five_dollars_count
    }
    pub fn get_ten_dollars(&self) -> i64 {
        self.ten_dollars_count
    }
    pub fn get_twenty_dollars(&self) -> i64 {
        self.twenty_dollars_count
    }
    pub fn get_fifty_dollars(&self) -> i64 {
        self.fifty_dollars_count
    }
    pub fn get_hundred_dollars(&self) -> i64 {
        self.hundred_dollars_count
    }
    pub fn get_value(&self) -> f64 {
        self.value
    }
}
//
