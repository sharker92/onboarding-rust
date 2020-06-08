pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        // let s_clone = s.clone();
        let mut result = Vec::new();
        let mut jump = 0;
        if !s.is_empty() && p.len() <= s.len() {
                for i in (0..=(s.len() - p.len())).skip(jump) {
                        println!("-jump {}, i {}", jump, i);
                        if i < jump {
                                continue;
                        }
                        let mut p_clone = p.clone();
                        let mut s_clone = s.clone();
                        let mut s_drain: String = s_clone.drain(i..(p.len() + i)).collect();
                        println!("{} drain", s_drain);

                        for s_char in s_drain.chars() {

                                if let Some(p_found_index) = p_clone.find(s_char) {
                                        println!("char: {} pfi: {}", s_char, p_found_index);
                                        println!("{} plast", p_clone);
                                        if s_drain.contains(&p_clone) {
                                                println!("simon");
                                                let last_cut = p_found_index + p_clone.len();
                                                p_clone.drain(p_found_index..last_cut);

                                        } else {
                                                p_clone.remove(p_found_index);
                                        }
                                        //--s_drain.remove(0); TODO ex5
                                        println!("{} plast", p_clone);
                                        //println!("{}", p_clone);
                                        if p_clone.is_empty() {
                                                result.push(i as i32);
                                                println!("vacio");
                                                break;
                                        }
                                } else {
                                        if let Some(x) = s.find(s_char) {
                                                jump = x;
                                        }
                                        break;
                                }

                        }
                }
        }
        result
}