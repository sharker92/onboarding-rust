pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::new();
        let mut memory = false;
        if !s.is_empty() && p.len() <= s.len() {
                let mut jump = 0;
                for i in 0..=(s.len() - p.len()) {
                        if i < jump {
                                continue;
                        }
                        let mut p_clone = p.clone();
                        let mut s_clone = s.clone();
                        let mut s_drain: String = s_clone.drain(i..(p.len() + i)).collect();

                        while !s_drain.is_empty() {
                                if memory {
                                        memory = false;
                                        let s_last = s_clone[(i - 1)..i].chars().next().unwrap();
                                        let s_new = s_drain.pop().unwrap();
                                        if s_new == s_last {
                                                result.push(i as i32);
                                                memory = true;
                                                break;
                                        } else {
                                                if !p.contains(s_new) {
                                                        jump = p.len() + i;
                                                }
                                                break;
                                        }
                                } else {
                                        memory = false;
                                        if s_drain.contains(&p_clone) {
                                                result.push(i as i32);
                                                memory = true;
                                                break;
                                        }
                                        let s_char = s_drain.remove(0);
                                        if let Some(p_found_index) = p_clone.find(s_char) {
                                                p_clone.remove(p_found_index);
                                        } else {
                                                if !p.contains(s_char) {
                                                        jump = p.len() + i - s_drain.len();
                                                }
                                                break;
                                        }
                                }
                        }
                }
        }
        result
}
