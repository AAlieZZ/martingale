fn main() {
    let mut before: u128 = 0;
    let mut after: u128 = 0;
    let mut max: u128 = 0;
    let mut num: u128 = 0;
    let mut outcome: bool = true;
    let principal: u128 = env::args().nth(1).expect("参数不完整！").parse().expect("参数错误！");
    let mut game = Game::new(principal, env::args().nth(2).expect("参数不完整！").parse().expect("参数错误！"));
    while outcome || after<before {
        before = game.get_principal();
        outcome = game.play();
        after = game.get_principal();
        println!("{}\t{}",after,game.get_stake());
        if max < after {
            max = after;
        }
        num += 1;
    }
    if max < principal {
        println!("血本无归！");
    }
    else {
        println!("赌局进行了{}回，最巅峰时拥有存款{}，最多赚了{}",num,max,max-principal);
    }
}
