use std::collections::HashMap;
use std::io::{stdin, Read};
use std::process;

pub fn decode(src: String, debug: bool) -> String {
    let mut res = String::new();
    let code = src.chars().collect::<Vec<char>>();
    let call_map = map_brackets(src);

    let mut mem = vec![0 as u8];
    let mut call_stack = Vec::new();
    let (mut pc, mut ptr) = (0 as usize, 0 as usize);

    let mut reader = stdin().bytes();

    while pc < code.len() {
        let val = mem[ptr];

        match code[pc] {
            '.' => { res.push(mem[ptr] as char); },
            ',' => { mem[ptr] = reader.next().unwrap().unwrap(); },
            '+' => { mem[ptr] += 1; },
            '-' => { 
                if mem[ptr] > 0 {
                    mem[ptr] -= 1; 
                }
            },
            '>' => {
                if ptr + 1 >= mem.len() {
                    mem.push(0);
                }
                ptr += 1;
            },
            '<' => {
                if ptr > 0 {
                    ptr -= 1;
                }
            },
            '[' => {
                if val == 0 {
                    pc = call_map[&pc];
                } else {
                    call_stack.push(pc);
                }
            },
            ']' => {
                let top = call_stack.pop().unwrap();
                if val != 0 {
                    pc = top - 1;
                }
            },
            _ => { },
        }

        pc += 1;
    }

    if debug {
        println!("Memory => {:?}", mem);
    }

    res
}

fn map_brackets(src: String) -> HashMap<usize, usize> {
    let (mut map, mut stack) = (HashMap::new(), Vec::new());

    for (idx, ch) in src.into_bytes().iter().enumerate() {
        match ch {
            b'[' => { stack.push(idx); },
            b']' => { map.insert(stack.pop().unwrap(), idx); },
            _ => { },
        }
    }

    if stack.len() != 0 {
        eprintln!("Brackets mismatch.\nTerminating program...\n");
        process::exit(1);
    }

    map
}

#[test]
fn test_bracket_scopes() {
    let src = "[[[]]]".to_string();
    let mut expected_res = HashMap::new();
    expected_res.insert(0, 5);
    expected_res.insert(1, 4);
    expected_res.insert(2, 3);

    let res = map_brackets(src);

    assert_eq!(expected_res, res);
}

#[test]
fn test_decode() {
    let src =
        "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>---.+++++++++++++++.---------------.+++++++++++++.".to_string();
    let expected_res = "apan";
    let res = decode(src, false);

    assert_eq!(expected_res, res);
}
