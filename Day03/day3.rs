use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    puzzle1();
    puzzle2();
}

fn puzzle1() {
    let file = File::open("./input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let lines: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();
  
    let mut gamma = 0b0000;
    let mut epsilon = 0b000;

    for j in 0..lines[0].len() {
        
        let mut zeros = 0;
        let mut ones = 0;

        for i in 0..lines.len() {
            if lines[i][j] == '0' {
                zeros += 1;
            }
            else {
                ones += 1;
            }
        }
        if zeros > ones {
            gamma = (gamma << 1) | 1;
            epsilon = (epsilon << 1) | 0;
        }
        else {
            gamma = (gamma << 1) | 0;
            epsilon = (epsilon << 1) | 1;
        }
      }
    println!("{}", gamma * epsilon);

}


fn get_stats(co2:bool, lines: &Vec<Vec<char>>) -> Vec<char> {

    let mut zeros:Vec<usize> = Vec::new();
    let mut ones:Vec<usize> = Vec::new();

    for i in 0..lines.len() {
        if lines[i][0] == '0' { zeros.push(i) }
        else { ones.push(i) }
    }

    for j in 1..lines[0].len() {

        let mut new_zeros = Vec::new();
        let mut new_ones = Vec::new();

        if co2 {
            if zeros.len() > ones.len() {
                for i in 0..zeros.len() {
                    if lines[zeros[i]][j] == '0' { new_zeros.push(zeros[i]) }
                    else { new_ones.push(zeros[i]) }
                }
            }
            else {
                for i in 0..ones.len() {
                    if lines[ones[i]][j] == '0' { new_zeros.push(ones[i]) }
                    else { new_ones.push(ones[i]) }
                }
            }
        } else {
            if zeros.len() <= ones.len() {
                for i in 0..zeros.len() {
                    if lines[zeros[i]][j] == '0' { new_zeros.push(zeros[i]) }
                    else { new_ones.push(zeros[i]) }
                }
            }
            else {
                for i in 0..ones.len() {
                    if lines[ones[i]][j] == '0' { new_zeros.push(ones[i]) }
                    else { new_ones.push(ones[i]) }
                }
            }
        }

        zeros = new_zeros;
        ones = new_ones;
        
        if co2 && zeros.len() == 1 && ones.len() == 1 {
            return lines[ones[0]].clone();
        }
        if zeros.len() == 1 && ones.len() == 1 {
            return lines[zeros[0]].clone();
        }
    }
    return lines[0].clone();
}


fn puzzle2() {

    let file = File::open("./input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let lines: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    let c = isize::from_str_radix(&get_stats(true, &lines).iter().collect::<String>(),2).unwrap();
    let o = isize::from_str_radix(&get_stats(false, &lines).iter().collect::<String>(),2).unwrap();

    println!("{}", c*o);

}

    
