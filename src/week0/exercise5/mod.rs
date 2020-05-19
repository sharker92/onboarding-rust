pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let mut times2 = times.clone();
    let mut stack = Vec::new();
    let mut buffer = Vec::new();
    buffer.push(vec![0, k, 0]);
    stack.push(vec![0, 0]);

    while !buffer.is_empty() {
        let mut remove = Vec::new();
        let last = (*buffer.last().unwrap())[1];
        if stack.iter().any(|x| x[0] == last) {
            buffer.pop();
            continue;
        }
        stack.push(vec![last, (*buffer.last().unwrap())[2]]);
        buffer.pop();
        for (i, x) in times2.iter().enumerate() {

            if x[0] == last {
                remove.push(i);
            }
        }
        for r in remove.into_iter().rev() {
            let mut to_push = times2.remove(r);
            to_push[2] += (*stack.last().unwrap())[1];
            buffer.push(to_push);
        }
        buffer.sort_by_cached_key(|x| x[2]);
        buffer.reverse();
    }
    if stack.len() < (n + 1) as usize {
        return -1;
    }
    (*stack.last().unwrap())[1]
}
