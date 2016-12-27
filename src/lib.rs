#[derive(Debug, Clone)]
pub struct Set<T> {
    items: Vec<T>,
}

impl<T: Ord + Clone> PartialEq for Set<T> {

    fn eq(&self, other: &Set<T>) -> bool {
        let ref a = *self.items;
        if self.items.len() != other.items.len() { return false; }
        for item in a {
            if !other.has(item) { return false; }
        }
        return true;
    }
}

impl<T: Ord + Clone> Set<T> {

    pub fn new(items: &Vec<T>) -> Set<T> {
        Set { items: items.clone() }
    }

    pub fn has(&self, l: &T) -> bool {
        self.items.iter().any(|x| x == l)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

/// Creates a `Set<T>` that is the union of two `Set<T>`.
///
/// The union of two `Set<T>`, R and S, is the `Set<T>` whose elements
/// belong either to R or to S or to both.
///
/// # Examples
///
/// ```Rust
/// use relations;
/// let a = Set::new(&vec![0, 1, 2, 3, 4, 5]);
/// let b = Set::new(&vec![4, 5, 6, 7, 8, 9]);
/// let c = Set::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// assert_eq!(union(&a, &b), c);
/// ```
pub fn union<T: Clone + Ord>(this: &Set<T>, that: &Set<T>) -> Set<T> {
    let mut v = Vec::new();
    v.extend_from_slice(this.items.as_slice());
    v.extend_from_slice(that.items.as_slice());
    v.sort();
    v.dedup();
    Set { items: v }
}

pub fn inter<T: Clone + Ord>(this: &Set<T>, that: &Set<T>) -> Set<T> {
    Set::new(&this.items.iter()
             .filter(|x| that.has(&x))
             .map(|x| x.clone())
             .collect())
}

pub fn compl<T: Clone + Ord>(this: &Set<T>, that: &Set<T>) -> Set<T> {
    Set::new(&this.items.iter()
             .filter(|x| !that.has(&x))
             .map(|x| x.clone())
             .collect())
}

#[derive(Debug)]
pub struct Relation<T> {
    set: Set<T>,
    links: Vec<(T, T)>,
}

impl<T: Ord + Clone> PartialEq for Relation<T> {

    fn eq(&self, other: &Relation<T>) -> bool {
        let ref a = *self.links;
        if self.links.len() != other.links.len() { return false; }
        for item in a {
            if !other.has(item) { return false; }
        }
        return true;
    }
}

impl<T: Ord + Clone> Relation<T> {

    pub fn new(set: &Set<T>, links: &Vec<(T, T)>) -> Relation<T> {
        Relation { set: set.clone(), links: links.clone() }
    }

    pub fn add_link(&mut self, l: (T, T)) {
        if !self.has(&l) && self.set.has(&l.0) && self.set.has(&l.1) {
            self.links.push(l);
        }
    }

    pub fn add_links(&mut self, ls: Vec<(T, T)>) {
        for l in ls {
            if !self.has(&l) && self.set.has(&l.0) && self.set.has(&l.1) {
                self.add_link(l);
            }
        }
    }

    pub fn has(&self, l: &(T, T)) -> bool {
        self.links.iter().any(|x| x == l)
    }

    pub fn neighbours(&self, v: &T) -> Set<T> {
        let mut a = Vec::new();
        for item in self.links.iter() {
            if item.0 == *v {
                a.push(item.0.clone());
            } else if item.1 == *v {
                a.push(item.1.clone());
            }
        }
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    pub fn degree(&self, v: &T) -> usize {
        self.neighbours(v).len()
    }

    pub fn domain(&self) -> Set<T> {
        let mut a = self.links.iter().map(|x| x.0.clone()).collect::<Vec<T>>();
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    pub fn codomain(&self) -> Set<T> {
        let mut a = self.links.iter().map(|x| x.1.clone()).collect::<Vec<T>>();
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    pub fn is_reflexive(&self) -> bool {
        self.set.items.iter().all(|x| self.has(&(x.clone(), x.clone())))
    }

    pub fn is_symmetric(&self) -> bool {
        self.links.iter().all(|x| self.has(&(x.1.clone(), x.0.clone())))
    }

    pub fn is_transitive(&self) -> bool {
        let test = self.set.items.iter();
        for item1 in test.clone() {
            for item2 in test.clone() {
                if self.has(&(item1.clone(), item2.clone())) {
                    if !test.clone().any(|x| self.has(&(item1.clone(), x.clone()))
                                         && self.has(&(x.clone(), item2.clone()))) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn ref_closure(&self) -> Relation<T> {
        let mut v = Vec::new();
        v.extend_from_slice(self.links.as_slice());
        for k in self.set.items.iter() {
            for i in self.set.items.iter() {
                for j in self.set.items.iter() {
                    if !self.has(&(i.clone(), j.clone()))
                        && self.has(&(i.clone(), k.clone()))
                            && self.has(&(k.clone(), j.clone())) {
                                v.push((i.clone(), j.clone()));
                            }
                }
            }
        }
        Relation::new(&self.set, &v)
    }

    pub fn sym_closure(&self) -> Relation<T> {
        let mut v = Vec::new();
        v.extend_from_slice(self.links.as_slice());
        let mut v = Relation::new(&self.set, &v);
        for i in self.links.iter() {
            v.add_link((i.1.clone(), i.0.clone()));
        }
        v
    }
}

pub fn rel_union<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    let mut v = Vec::new();
    v.extend_from_slice(this.links.as_slice());
    v.extend_from_slice(that.links.as_slice());
    v.sort();
    v.dedup();
    Relation::new(&union(&this.set, &that.set), &v)
}        

pub fn rel_inter<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    Relation::new(
        &inter(&this.set, &that.set),
        &this.links.iter()
        .filter(|x| that.has(&x))
        .map(|x| x.clone())
        .collect()
        )
}

pub fn rel_compl<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    Relation::new(
        &compl(&this.set, &that.set),
        &this.links.iter()
        .filter(|x| !that.has(&x))
        .map(|x| x.clone())
        .collect()
        )
}

pub fn rel_compo<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    let mut v = Vec::new();
    for item in this.links.iter() {
        for item2 in that.links.iter().filter(|x| x.0 == item.1) {
            v.push((item.0.clone(), item2.1.clone()));
        }
    }
    Relation::new(&union(&this.domain(), &that.codomain()), &v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let a = Set::new(&vec![0, 1, 2, 3, 4, 5]);
        let b = Set::new(&vec![4, 5, 6, 7, 8, 9]);
        let c = Set::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(union(&a, &b), c);
    }

    #[test]
    fn test_inter() {
        let a = Set::new(&vec![0, 1, 2, 3, 4, 5]);
        let b = Set::new(&vec![4, 5, 6, 7, 8, 9]);
        let c = Set::new(&vec![4, 5]);
        assert_eq!(inter(&a, &b), c);
    }

    #[test]
    fn test_compl() {
        let a = Set::new(&vec![0, 1, 2, 3, 4, 5]);
        let b = Set::new(&vec![4, 5, 6, 7, 8, 9]);
        let c = Set::new(&vec![0, 1, 2, 3]);
        assert_eq!(compl(&a, &b), c);
    }

    #[test]
    fn test_link_eq() {
        assert_eq!((0, 1), (0, 1));
        assert_ne!((0, 1), (0, 2));
        assert_ne!((1, 1), (0, 1));
    }

    #[test]
    fn test_relation_eq() {
        let set = Set::new(&vec![0, 1, 2]);
        assert_eq!(Relation::new(&set, &vec![(0, 1)]), Relation::new(&set, &vec![(0, 1)]));
        assert_ne!(Relation::new(&set, &vec![(0, 1)]), Relation::new(&set, &vec![(0, 2)]));
        assert_ne!(Relation::new(&set, &vec![(0, 2)]), Relation::new(&set, &Vec::new()));

        let r1 = Relation::new(&set, &vec![(1, 0), (0, 1)]);
        let r2 = Relation::new(&set, &vec![(0, 1), (1, 0)]);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_link() {
        let set = Set::new(&vec![0, 1, 2]);
        let mut a = Relation::new(&set, &Vec::new());
        let b = Relation::new(&set, &vec![(0, 1)]);
        a.add_link((0, 1));
        a.add_link((0, 1));
        assert_eq!(b, a);
    }

    #[test]
    fn test_has() {
        let a = Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 1)]);
        assert!(a.has(&(0, 1)));
        assert!(!a.has(&(1, 1)));
    }

    #[test]
    fn test_domain() {
        let mut a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 5]), &vec![(0, 1)]);
        a.add_links(vec![(1, 2), (2, 2), (3, 5)]);
        assert_eq!(a.domain(), Set::new(&vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_codomain() {
        let mut a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 5]), &vec![(0, 1)]);
        a.add_links(vec![(1, 2), (2, 2), (3, 5)]);
        assert_eq!(a.codomain(), Set::new(&vec![1, 2, 5]));
    }

    #[test]
    fn test_reflexive() {
        assert!(Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 1)])
                .is_reflexive());
        assert!(!Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 0)])
                .is_reflexive());
    }

    #[test]
    fn test_symmetric() {
        assert!(Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 0)])
                .is_symmetric());
        assert!(!Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 1)])
                .is_symmetric());
    }

    #[test]
    fn test_transitive() {
        assert!(Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 0)])
                .is_transitive());
        assert!(!Relation::new(&Set::new(&vec![0, 1, 2]), &vec![(0, 0), (0, 1), (2, 1)])
                .is_transitive());
    }

    #[test]
    fn test_relation_union() {
        let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 0), (4, 5)]);
        let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(6, 6), (4, 5)]);
        let c = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), &vec![(0, 0), (4, 5), (6, 6)]);
        assert_eq!(rel_union(&a, &b), c);
    }

    #[test]
    fn test_relation_inter() {
        let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 0), (4, 5)]);
        let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(6, 6), (4, 5)]);
        let c = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), &vec![(4, 5)]);
        assert_eq!(rel_inter(&a, &b), c);
    }

    #[test]
    fn test_relation_compl() {
        let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 0), (4, 5)]);
        let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(6, 6), (4, 5)]);
        let c = Relation::new(&Set::new(&vec![0, 1, 2, 3]), &vec![(0, 0)]);
        assert_eq!(rel_compl(&a, &b), c);
    }

    #[test]
    fn test_relation_compo() {
        let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 1), (1, 1)]);
        let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(1, 2), (1, 3)]);
        let c = Relation::new(&Set::new(&vec![0, 1, 2, 3]), &vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
        assert_eq!(rel_compo(&a, &b), c);
    }

    #[test]
    fn test_ref_closure() {
        let r = Relation::new(&Set::new(&vec![0, 1, 2, 3]), &vec![(0, 0), (0, 1), (1, 3), (2, 1)]);
        let q = Relation::new(&Set::new(&vec![0, 1, 2, 3]), &vec![(0, 0), (0, 1), (1, 3), (2, 1), (0, 3), (2, 3)]);
        assert_eq!(r.ref_closure(), q);
    }

    #[test]
    fn test_sym_closure() {
        let r = Relation::new(&Set::new(&vec![0, 1, 2, 3]), &vec![(0, 0), (0, 1), (1, 3), (2, 1)]);
        let q = Relation::new(&Set::new(&vec![0, 1, 2, 3]), &vec![(0, 0), (0, 1), (1, 3), (2, 1), (1, 0), (3, 1), (1, 2)]);
        assert_eq!(r.sym_closure(), q);
    }
}
