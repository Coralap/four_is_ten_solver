use std::io;
use evalexpr::*;
use itertools::Itertools;

fn main() {
    let mut inputs = vec![0, 0, 0, 0];
    let mut operators = vec!["+", "-", "*", "/"];
    let mut brackets: bool=true;
    let mut stop = false;
    while !stop {
        println!("Remove an operator?");
        let mut remove = String::new();
        io::stdin().read_line(&mut remove).expect("I didnt expect that");
        if operators.contains(&remove.trim()) {
            operators.retain(|&a| a != remove.trim());
        } else if remove.trim() ==")" || remove.trim()=="("{
            brackets=false;
        }
        else {
            stop = true;
        }
    }



    for i in 0..4 {
        println!("Input number bish {i}");
        let mut player_guess = String::new();
        io::stdin().read_line(&mut player_guess).expect("I didnt expect that");
        player_guess = player_guess.trim().to_string();
        inputs[i] = player_guess.parse().expect("Didnt parsy parsy the inty");
    }

    inputs.sort();
    inputs.reverse();
    let sum: f32 = inputs.iter().map(|&i| i as f32).sum();
    let mut best_answer_distance = (10.0 - sum).abs();
    let mut best_answer = String::new();
    let mut iter = 0;

    for perm in inputs.iter().permutations(4) {
        let perm: Vec<i32> = perm.into_iter().copied().collect();
        for &i in &operators {
            for &j in &operators {
                for &k in &operators {
                    let expressions: Vec<String>;
                    if brackets{
                        expressions =
                        vec![format!("{}.0 {} {}.0 {} {}.0 {} {}.0", perm[0], i, perm[1], j, perm[2], k, perm[3]),
                        format!("({}.0 {} {}.0) {} {}.0 {} {}.0", perm[0], i, perm[1], j, perm[2], k, perm[3]),
                        format!("{}.0 {} ({}.0 {} {}.0) {} {}.0", perm[0], i, perm[1], j, perm[2], k, perm[3]),
                        format!("{}.0 {} {}.0 {} ({}.0 {} {}.0)", perm[0], i, perm[1], j, perm[2], k, perm[3]),
                        format!("{}.0 {} ({}.0 {} {}.0 {} {}.0)", perm[0], i, perm[1], j, perm[2], k, perm[3]),
                        format!("({}.0 {} {}.0 {} {}.0) {} {}.0", perm[0], i, perm[1], j, perm[2], k, perm[3]),
                    ];
                    }else{
                        expressions =
                        vec![format!("{}.0 {} {}.0 {} {}.0 {} {}.0", perm[0], i, perm[1], j, perm[2], k, perm[3])];
                    }
                    
     

                    for eqe in expressions {
                        iter += 1;
                        if eqe.contains("/ 0") {
                            continue;
                        }
                        let answer: f32 = match eval(eqe.clone().trim()).unwrap() {
                            Value::Float(x) => x as f32,
                            Value::Int(x) => x as f32,
                            _ => 0.0,
                        };
                        if (10.0 - answer).abs() < best_answer_distance {
                            best_answer_distance = (10.0 - answer).abs();
                            best_answer = eqe.trim().to_string();
                            if best_answer_distance == 0.0 {
                                println!("{:?}", best_answer);
                                println!("{:?}", best_answer_distance);
                                println!("{}", iter);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", best_answer);
    println!("{:?}", best_answer_distance);
    println!("{}", iter);
}
