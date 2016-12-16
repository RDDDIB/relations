#[derive(Debug)]
pub struct Link<A, B> {
    from: A,
    to: B,
}

impl<A: PartialEq, B: PartialEq> PartialEq for Link<A, B> {

    fn eq(&self, other: &Link< A, B>) -> bool {
        (self.from == other.from) & (self.to == other.to)
    }
}

#[derive(Debug)]
pub struct Relation<'a, A: 'a, B: 'a> {
    links: &'a mut Vec<Link<A, B>>,
}

impl<'a, A: PartialEq + 'a, B: PartialEq + 'a> PartialEq for Relation<'a, A, B> {

    fn eq(&self, other: &Relation<'a, A, B>) -> bool {
        let ref this = *self.links;
        let ref that = *other.links;
        (this.len() == that.len()) &&
            this.iter().zip(that).all(|(a, b)| a == b)
    }
}

impl<'a, A: 'a, B: 'a> Relation<'a, A, B> {

    pub fn add_link(&mut self, l: Link<A, B>) {
        self.links.push(l);
    }

    pub fn add_links(&mut self, ls: Vec<(A, B)>) {
        for l in ls {
            let link = Link { from: l.0, to: l.1 };
            self.add_link(link);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_eq() {
        assert_eq!(Link { from: 0, to: 1}, Link { from: 0, to: 1 });
        assert_ne!(Link { from: 0, to: 1}, Link { from: 0, to: 2 });
        assert_ne!(Link { from: 1, to: 1}, Link { from: 0, to: 1 });
    }

    #[test]
    fn test_relation_eq() {
        assert_eq!(Relation { links: &mut vec![Link { from: 0, to: 1}] }, Relation { links: &mut vec![Link { from: 0, to: 1}] });
        assert_ne!(Relation { links: &mut vec![Link { from: 0, to: 1}] }, Relation { links: &mut vec![Link { from: 0, to: 2}] });
        assert_ne!(Relation { links: &mut vec![Link { from: 0, to: 2}] }, Relation { links: &mut Vec::new() });
    }

    #[test]
    fn test_link() {
        let mut a = Relation { links: &mut Vec::new() };
        let b = Relation { links: &mut vec![Link { from: 0, to: 1 }] };
        a.add_link(Link { from: 0, to: 1 });
        assert_eq!(b, a);
    }
}
