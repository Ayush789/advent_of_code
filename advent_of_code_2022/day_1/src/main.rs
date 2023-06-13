use std::fs;

fn main(){
    part1();
}

fn part1() {
    let filename = "input1.txt".to_string();
    let data = fs::read_to_string(filename).unwrap();

    let ans: i32 = data
        .split("\r\n\r\n")
        .map(|x| x.split("\r\n").map(|v| v.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap();
    dbg!(ans);
}
