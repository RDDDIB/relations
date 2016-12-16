#[derive(Debug)]
pub struct Relation<'a, A: 'a, B: 'a> {
    links: &'a mut Vec<(A, B)>,
}

impl<'a, A: Ord + Clone + 'a, B: Ord + Clone + 'a> PartialEq for Relation<'a, A, B> {

    fn eq(&self, other: &Relation<'a, A, B>) -> bool {
        let ref a = *self.links;
        if self.links.len() != other.links.len() { return false; }
        for item in a {
            if !other.has(item) { return false; }
        }
        return true;
    }
}

impl<'a, A: Ord + Clone + 'a, B: Ord + Clone + 'a> Relation<'a, A, B> {

    pub fn add_link(&mut self, l: (A, B)) {
        if !self.has(&l) {
            self.links.push(l);
        }
    }

    pub fn add_links(&mut self, ls: Vec<(A, B)>) {
        for l in ls {
            if !self.has(&l) {
                self.add_link(l);
            }
        }
    }

    pub fn has(&self, l: &(A, B)) -> bool {
        self.links.iter().any(|x| x == l)
    }

    pub fn domain(&self) -> Vec<A> {
        let mut a = self.links.iter().map(|x| x.0.clone()).collect::<Vec<A>>();
        a.sort();
        a.dedup();
        a
    }

    pub fn codomain(&self) -> Vec<B> {
        let mut a = self.links.iter().map(|x| x.1.clone()).collect::<Vec<B>>();
        a.sort();
        a.dedup();
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_eq() {
        assert_eq!((0, 1), (0, 1));
        assert_ne!((0, 1), (0, 2));
        assert_ne!((1, 1), (0, 1));
    }

    #[test]
    fn test_relation_eq() {
        assert_eq!(Relation { links: &mut vec![(0, 1)] }, Relation { links: &mut vec![(0, 1)] });
        assert_ne!(Relation { links: &mut vec![(0, 1)] }, Relation { links: &mut vec![(0, 2)] });
        assert_ne!(Relation { links: &mut vec![(0, 2)] }, Relation { links: &mut Vec::new() });

        let r1 = Relation { links: &mut vec![(1, 0), (0, 1)] };
        let r2 = Relation { links: &mut vec![(0, 1), (1, 0)] };
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_link() {
        let mut a = Relation { links: &mut Vec::new() };
        let b = Relation { links: &mut vec![(0, 1)] };
        a.add_link((0, 1));
        a.add_link((0, 1));
        assert_eq!(b, a);
    }

    #[test]
    fn test_has() {
        let a = Relation { links: &mut vec![(0, 1)] };
        assert!(a.has(&(0, 1)));
        assert!(!a.has(&(1, 1)));
    }

    #[test]
    fn test_domain() {
        let mut a = Relation { links: &mut vec![(0, 1)] };
        a.add_links(vec![(1, 2), (2, 2), (3, 5)]);
        assert_eq!(a.domain(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_codomain() {
        let mut a = Relation { links: &mut vec![(0, 1)] };
        a.add_links(vec![(1, 2), (2, 2), (3, 5)]);
        assert_eq!(a.codomain(), vec![1, 2, 5]);
    }
}
