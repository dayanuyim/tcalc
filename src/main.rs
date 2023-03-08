use std::env;
use std::process;

fn basename(s: &str) -> &str {
    match s.rsplit('/').next() {
        Some(name) => name,
        _ => s,
    }
}

fn parse_timestr(s: &str) -> u32 {
    let mut tokens: Vec<&str> = s.split(':').collect();

    let mut sec = 0;
    while tokens.len() > 0 {
        sec *= 60;
        sec += tokens.remove(0).parse::<u32>().expect("not number");
    }
    sec
}

fn fmt_timestr(mut t: u32) -> String {
    let s = t % 60;
    t /= 60;
    let m = t % 60;
    t /= 60;
    format!("{:02}:{:02}:{:02}", t, m, s)
}

fn str_to_num(s: &String) -> f32
{
    if s.contains(":") {
        parse_timestr(s) as f32
    }
    else {
        s.parse::<f32>().expect("not number")
    }
}


fn main() {
    let output_num;
    let args: Vec<String> = env::args().collect();
    let (t1, op, t2) = match &args[..] {
        [_, t1, op, t2 ] => {
            output_num = t1.contains(":") && t2.contains(":");
            (str_to_num(t1), op, str_to_num(t2))
        },
        [prog, ..] => {
            println!("usage: {} hh:mm:ss <op> <num>", basename(prog));
            process::exit(1);
        },
        _ => {
            process::exit(1);
        }
    };

    let res = match op.as_str() {
        "+"     => t1 + t2,
        "-"     => t1 - t2,
        "*"|"x" => t1 * t2,
        "/"     => t1 / t2,
        _ => panic!("invalid operator '{}' ", op)
    };

    //println!("{} ({})", fmt_timestr(res as u32), res);
    if output_num {
        println!("{}", res);
    } else {
        println!("{}", fmt_timestr(res as u32));
    }
}
