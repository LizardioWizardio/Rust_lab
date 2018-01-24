use std::io;
use std::mem;
static E: f64 = 0.00000001;
#[derive(Clone)]
struct point
{
    x: f64,
    y: f64
}
#[derive(Clone)]
struct segment
{
    p: point,
    q: point
}
#[derive(Clone)]
struct line
{
    a: f64,
    b: f64,
    c: f64
}
fn distance(a: &point, b: &point) -> f64
{
    return ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt();
}
fn best<'t>(a: &'t point, b: &'t point, c: &'t point) -> &'t point
{
    if distance(a, b) < distance(a, c) {
        return b;
    }
    else {
        return c;
    }
}
fn line(s: &segment) -> line
{
    let a = s.p.y - s.q.y;
    let b = s.q.x - s.p.x;
    let c = s.p.x * s.q.y - s.q.x * s.p.y;
    let norm = (a * a + b * b).sqrt();
    return line{a: a / norm, b: b / norm, c: c / norm};
}
fn in_1d(l: f64, m: f64, r: f64) -> bool
{
    return l.min(r) <= m && m <= r.max(l);
}
fn inter(a: &segment, b: &segment) -> Option<point>
{
    let l1 = line(a);
    let l2 = line(b);
    let denom = l1.a * l2.b - l2.a * l1.b;
    if denom.abs() < E  {
        if (l1.c - l2.c).abs() < E {
            if in_1d(b.p.x, a.p.x, b.q.x)
                && in_1d(b.p.y, a.p.y, b.q.y) {
                return Some(a.p.clone());
            }
            if (a.p.x - a.q.x) * (a.p.x - b.p.x) >= 0.
                && (a.p.y - a.q.y) * (a.p.y - b.p.y) >= 0. {
                return Some(best(&a.p, &b.p, &b.q).clone());
            }
        }
    }
    else {
        let x = (l1.b * l2.c - l2.b * l1.c) / denom;
        let y = (l2.a * l1.c - l1.a * l2.c) / denom;
        if (a.p.x - a.q.x) * (a.p.x - x) >= 0.
            && (a.p.y - a.q.y) * (a.p.y - y) >= 0.
            && in_1d(b.p.x, x, b.q.x)
            && in_1d(b.p.y, y, b.q.y) {
            return Some(point{ x: x, y: y });
        }
    }
    return None;
}
fn read_segment() -> Option<segment>
{
    let mut buf = String::new();
    let result = io::stdin().read_line(&mut buf);
    match result {
        Ok(n) => {
            if n == 0 {
                return None;
            }
            let mut iter = buf.split(' ');
            let mut iter2 = iter.next().unwrap().split(',');
            let mut x1 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            let mut y1 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            iter2 = iter.next().unwrap().split(',');
            let mut x2 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            let mut y2 = iter2.next().unwrap().trim().parse::<f64>().expect("invalid input");
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
                mem::swap(&mut y1, &mut y2);
            }
            let p = point{ x: x1, y: y1 };
            let q = point{ x: x2, y: y2 };
            return Some(segment{ p: p, q: q});
        }
        Err(_) => return None
    }
}
fn main()
{
    let beam = read_segment().unwrap();
    let mut answer: Option<point> = None;
    loop {
        match read_segment() {
            Some(segment) => {
                match inter(&beam, &segment) {
                    Some(p) => {
                        match answer {
                            Some(cur) => answer = Some(best(&beam.p, &cur, &p).clone()),
                            None => answer = Some(p)
                        }
                    }
                    None => {}
                }
            }
            None => break
        }
    }
    match answer {
        Some(p) => {
            println!("{} {}", p.x, p.y);
        }
        None => {}
    }
}
