use std::env;

fn s_converter(s: &str, v: u32) -> String {
    let tokens: Vec<&str> = s.split('=').collect();
    // Capacityを指定
    let mut result = Vec::with_capacity(tokens.len() - 1);

    for t in &tokens[..tokens.len() - 1] {
        result.push(format!("A({},{})", v, t))
    }

    result.join("=")
}

fn inputs(args: &Vec<String>) -> Result<(u32, u32), String> {
    fn read_int(s: &str) -> Result<u32, String> {
        s.parse::<u32>().map_err(|err| err.to_string())
    }

    if args.len() != 3 {
        Err(format!("invalid number of arguments: {}", args.len()))
    } else {
        let u: Vec<u32> = args.iter().skip(1).flat_map(|s| read_int(s)).collect();
        if u.len() != 2 {
            Err(format!("invalid digit found in string"))
        } else {
            Ok((u[0], u[1]))
        }
    }
}

fn ackermann(m: u32, n: u32) -> String {
    fn work(m: u32, n: u32) -> (u32, String) {
        if m == 0 {
            (n + 1, format!("A({},{})={}", m, n, n + 1))
        } else if n == 0 {
            let (v, s) = work(m - 1, 1);
            (v, format!("A({},{})={}", m, n, s))
        } else {
            let (v, s) = work(m, n - 1);
            let (v2, s2) = work(m - 1, v);
            (
                v2,
                format!("A({},{})={}={}", m, n, s_converter(&s, m - 1), s2),
            )
        }
    }

    let (v, s) = work(m, n);
    s
}

fn main() {
    match inputs(&env::args().map(|s| s).collect()) {
        Ok((m, n)) => println!("{}", ackermann(m, n).replace("=", "\n=")),
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path1() {
        assert_eq!(ackermann(0, 0), "A(0,0)=1");
    }

    #[test]
    fn path2() {
        assert_eq!(ackermann(1, 0), "A(1,0)=A(0,1)=2");
    }

    #[test]
    fn path3() {
        assert_eq!(ackermann(1, 1), "A(1,1)=A(0,A(1,0))=A(0,A(0,1))=A(0,2)=3");
        assert_eq!(
            ackermann(1, 2),
            "A(1,2)=A(0,A(1,1))=A(0,A(0,A(1,0)))=A(0,A(0,A(0,1)))=A(0,A(0,2))=A(0,3)=4"
        );
        assert_eq!(
            ackermann(2, 1),
            "A(2,1)=A(1,A(2,0))=A(1,A(1,1))=A(1,A(0,A(1,0)))=A(1,A(0,A(0,1)))=A(1,A(0,2))=A(1,3)=A(0,A(1,2))=A(0,A(0,A(1,1)))=A(0,A(0,A(0,A(1,0))))=A(0,A(0,A(0,A(0,1))))=A(0,A(0,A(0,2)))=A(0,A(0,3))=A(0,4)=5");
    }
}
