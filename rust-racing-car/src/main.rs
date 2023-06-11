use std::cmp::max;
use std::io;
use rand::Rng;

fn main() {
    println!("경주할 자동차 이름을 입력하세요.(이름은 쉼표(,) 기준으로 구분)");
    let mut car_names_input = String::new();
    io::stdin().read_line(&mut car_names_input).expect("error");

    let car_names = car_names_input.trim().split(",");

    println!("시도할 회수는 몇회인가요?");
    let mut max_count_input = String::new();
    io::stdin().read_line(&mut max_count_input).expect("error");

    let max_count: i32 = match max_count_input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("fail")
    };

    #[derive(Debug)]
    struct Car {
        name: String,
        distance: u32,
    }

    let mut cars: Vec<Car> = car_names
        .map(|name| Car { name: String::from(name), distance: 0 })
        .collect();

    println!("실행 결과");

    let mut try_count = 0;
    let mut max_distance = 0;
    while try_count < max_count {
        try_count += 1;
        for car in &mut cars {
            let forward = rand::thread_rng().gen_bool(0.5);
            if forward == true {
                car.distance += 1;
                max_distance = max(max_distance, car.distance)
            }
        }
        for car in &cars {
            println!("{} : {}", car.name, "-".repeat(car.distance as usize));
        }
        println!()
    }

    let mut winners = cars.iter()
        .filter(|car| car.distance == max_distance)
        .map(|car| car.name.as_str())
        .fold(String::new(), |a, b| a + b + ", ");
    winners.pop();
    winners.pop();
    println!("최종 우승자 : {}", winners)
}