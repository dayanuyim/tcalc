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

fn main() {
    let args: Vec<String> = env::args().collect();
    let (t1, op, t2) = match &args[..] {
        [_, t1, op, t2 ] => (
            parse_timestr(t1) as f32,
            op,
            t2.parse::<f32>().expect("not number")
            ),
        [prog, ..] => {
            println!("usage: {} hh:mm:ss <op> <num>", basename(prog));
            process::exit(1);
        },
        _ => {
            process::exit(2);
        }
    };

    let res = match op.as_str() {
        "+"     => t1 + t2,
        "-"     => t1 - t2,
        "*"|"x" => t1 * t2,
        "/"     => t1 / t2,
        _ => panic!("invalid operator '{}' ", op)
    } as u32;

    println!("{}", fmt_timestr(res));
}
