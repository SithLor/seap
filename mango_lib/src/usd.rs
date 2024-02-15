pub enum Money {
    Penny, // 1 cent
    Nickel, // 5 cents
    Dime, // 10 cents
    Quarter, // 25 cents
    FiftyCentCoin, // 50 cents
    DollarCoin, // 100 cents
    TwoDollarCoin, // 200 cents
    OneDollarBill, // 100 cents
    TwoDollarBill, // 200 cents
    FiveDollarBill, // 500 cents
    TenDollarBill, // 1000 cents
    TwentyDollarBill, // 2000 cents
    FiftyDollarBill, // 5000 cents
    HundredDollarBill, // 10000 cents
}
pub fn get_value(val:Money){
    match val {
        Money::Penny => 0.01,
        Money::Nickel => 0.50,
        Money::Dime => 0.10,
        Money::Quarter => 0.25,
        Money::FiftyCentCoin => 0.50,
        Money::DollarCoin => 1.0,
        Money::TwoDollarCoin=>2.0,
        Money::OneDollarBill=>1.0,
        Money::TwoDollarBill=>2.0,
        Money::FiveDollarBill=>5.0,
        Money::TenDollarBill=>10.0,
        Money::TwentyDollarBill=>20.0,
        Money::FiftyDollarBill=>50.0,
        Money::HundredDollarBill=>100.0,
    };
}//I dont care this does not need to be tested it is just a simple match statement


