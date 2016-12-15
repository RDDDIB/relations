#[derive(Debug)]
struct Link<A, B> {
    from: A,
    to: B,
}

#[derive(Debug)]
struct Relation<'a, A: 'a, B: 'a> {
    links: &'a mut Vec<Link<A, B>>,
}

fn add_link<A, B>(r: &mut Relation<A, B>, l: Link<A, B>) {
    r.links.push(l);
}

fn add_links<A, B>(r: &mut Relation<A, B>, ls: Vec<(A, B)>) {
    for l in ls {
        let link = Link { from: l.0, to: l.1 };
        add_link(r, link);
    }
}

fn main() {
    println!("Hello, world!");
    let list = vec![(0, 1), (1, 2)];
    let mut b = Relation { links: &mut Vec::new() };
    println!("{:?}", b);
    add_links(&mut b, list);
    println!("{:?}", b);
}
