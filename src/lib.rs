#[derive(Debug, Clone)]
pub struct Link<A, B> {
    from: A,
    to: B,
}

impl<A: PartialEq, B: PartialEq> PartialEq for Link<A, B> {

    fn eq(&self, other: &Link< A, B>) -> bool {
        (self.from == other.from) & (self.to == other.to)
    }
}

impl<A: PartialEq, B: PartialEq> Link<A, B> {

    pub fn new(f: A, t: B) -> Link<A, B> {
        Link { from: f, to: t}
    }
}

#[derive(Debug)]
pub struct Relation<'a, A: 'a, B: 'a> {
    links: &'a mut Vec<Link<A, B>>,
}

impl<'a, A: PartialEq + 'a, B: PartialEq + 'a> PartialEq for Relation<'a, A, B> {

    fn eq(&self, other: &Relation<'a, A, B>) -> bool {
        let ref a = *self.links;
        if self.links.len() != other.links.len() { return false; }
        for item in a {
            if !other.has(item) { return false; }
        }
        return true;
    }
}

impl<'a, A: PartialEq + 'a, B: PartialEq + 'a> Relation<'a, A, B> {

    pub fn add_link(&mut self, l: Link<A, B>) {
        if !self.has(&l) {
            self.links.push(l);
        }
    }

    pub fn add_links(&mut self, ls: Vec<(A, B)>) {
        for l in ls {
            let link = Link::new(l.0, l.1);
            if !self.has(&link) {
                self.add_link(link);
            }
        }
    }

    pub fn has(&self, l: &Link<A, B>) -> bool {
        self.links.iter().any(|x| x == l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_eq() {
        assert_eq!(Link::new(0, 1), Link::new(0, 1));
        assert_ne!(Link::new(0, 1), Link::new(0, 2));
        assert_ne!(Link::new(1, 1), Link::new(0, 1));
    }

    #[test]
    fn test_relation_eq() {
        assert_eq!(Relation { links: &mut vec![Link::new(0, 1)] }, Relation { links: &mut vec![Link::new(0, 1)] });
        assert_ne!(Relation { links: &mut vec![Link::new(0, 1)] }, Relation { links: &mut vec![Link::new(0, 2)] });
        assert_ne!(Relation { links: &mut vec![Link::new(0, 2)] }, Relation { links: &mut Vec::new() });

        let r1 = Relation { links: &mut vec![Link::new(1, 0), Link::new(0, 1)] };
        let r2 = Relation { links: &mut vec![Link::new(0, 1), Link::new(1, 0)] };
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_link() {
        let mut a = Relation { links: &mut Vec::new() };
        let b = Relation { links: &mut vec![Link::new(0, 1)] };
        a.add_link(Link::new(0, 1));
        a.add_link(Link::new(0, 1));
        assert_eq!(b, a);
    }

    #[test]
    fn test_has() {
        let a = Relation { links: &mut vec![Link::new(0, 1)] };
        assert!(a.has(&Link::new(0, 1)));
        assert!(!a.has(&Link::new(1, 1)));
    }

}
