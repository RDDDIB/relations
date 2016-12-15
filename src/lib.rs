#[derive(Debug)]
pub struct Link<A, B> {
    from: A,
    to: B,
}

#[derive(Debug)]
pub struct Relation<'a, A: 'a, B: 'a> {
    links: &'a mut Vec<Link<A, B>>,
}

pub fn add_link<A, B>(r: &mut Relation<A, B>, l: Link<A, B>) {
    r.links.push(l);
}

pub fn add_links<A, B>(r: &mut Relation<A, B>, ls: Vec<(A, B)>) {
    for l in ls {
        let link = Link { from: l.0, to: l.1 };
        add_link(r, link);
    }
}
