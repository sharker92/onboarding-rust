pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut times2 = times.clone();
    let mut stack = Vec::new();
    let mut time_act = 0;
    let mut time_max = 0;
    let mut redundant = 0;
    stack.push(k);

    while !stack.is_empty() {
        let last = *stack.last().unwrap();
        for (i, x) in times2.iter().enumerate() {
            if x[0] == last {
                if !stack.contains(&x[1]) {
                    stack.push(x[1]);
                    time_act += x[2];
                    times2.remove(i);
                    break;
                } else {
                    redundant += 1;
                }
            }

        }
        // println!("{:?}", stack);
        if last == *stack.last().unwrap() {
            stack.pop();
            if time_act > time_max {
                time_max = time_act;
            }
            time_act = 0;
            // println!("{:?}**", stack);
        }

    }
    // println!("{}, {}", times2.len(), redundant);
    if times2.len() > redundant {
        return -1;
    }
    time_max
}
