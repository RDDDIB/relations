#[derive(Debug)]
pub struct Set<A> {
    items: Vec<A>,
}

impl<A: Ord + Clone> PartialEq for Set<A> {

    fn eq(&self, other: &Set<A>) -> bool {
        let ref a = *self.items;
        if self.items.len() != other.items.len() { return false; }
        for item in a {
            if !other.has(item) { return false; }
        }
        return true;
    }
}

impl<A: Ord + Clone> Set<A> {

    pub fn has(&self, l: &A) -> bool {
        self.items.iter().any(|x| x == l)
    }

    pub fn union(&self, other: &Set<A>) -> Set<A> {
        let mut v = Vec::new();
        v.extend_from_slice(self.items.as_slice());
        v.extend_from_slice(other.items.as_slice());
        v.sort();
        v.dedup();
        Set { items: v }
    }

    pub fn inter(&self, other: &Set<A>) -> Set<A> {
        let mut v = Vec::new();
        for item in self.items.iter() {
            if other.has(&item) { v.push(item.clone()); }
        }
        Set { items: v }
    }
}

#[derive(Debug)]
pub struct Relation<'a, A: 'a> {
    links: &'a mut Vec<(A, A)>,
}

impl<'a, A: Ord + Clone + 'a> PartialEq for Relation<'a, A> {

    fn eq(&self, other: &Relation<'a, A>) -> bool {
        let ref a = *self.links;
        if self.links.len() != other.links.len() { return false; }
        for item in a {
            if !other.has(item) { return false; }
        }
        return true;
    }
}

impl<'a, A: Ord + Clone + 'a> Relation<'a, A> {

    pub fn add_link(&mut self, l: (A, A)) {
        if !self.has(&l) {
            self.links.push(l);
        }
    }

    pub fn add_links(&mut self, ls: Vec<(A, A)>) {
        for l in ls {
            if !self.has(&l) {
                self.add_link(l);
            }
        }
    }

    pub fn has(&self, l: &(A, A)) -> bool {
        self.links.iter().any(|x| x == l)
    }

    pub fn domain(&self) -> Vec<A> {
        let mut a = self.links.iter().map(|x| x.0.clone()).collect::<Vec<A>>();
        a.sort();
        a.dedup();
        a
    }

    pub fn codomain(&self) -> Vec<A> {
        let mut a = self.links.iter().map(|x| x.1.clone()).collect::<Vec<A>>();
        a.sort();
        a.dedup();
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let a = Set { items: vec![0, 1, 2, 3, 4, 5] };
        let b = Set { items: vec![4, 5, 6, 7, 8, 9] };
        let c = Set { items: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };
        assert_eq!(a.union(&b), c);
    }

    #[test]
    fn test_inter() {
        let a = Set { items: vec![0, 1, 2, 3, 4, 5] };
        let b = Set { items: vec![4, 5, 6, 7, 8, 9] };
        let c = Set { items: vec![4, 5] };
        assert_eq!(a.inter(&b), c);
    }

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
