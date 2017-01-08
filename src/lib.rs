/// Represents a discrete set of objects.
#[derive(Debug, Clone)]
pub struct Set<T> {

    /// The objects.
    items: Vec<T>,
}

impl<T: Ord + Clone> PartialEq for Set<T> {
    fn eq(&self, other: &Set<T>) -> bool {
        let ref a = *self.items;
        if self.items.len() != other.items.len() {
            return false;
        }
        for item in a {
            if !other.has(item) {
                return false;
            }
        }
        return true;
    }
}

impl<T: Ord + Clone> Set<T> {
    /// Creates a new `Set<T>` with a given `Vec` of objects.
    pub fn new(items: &Vec<T>) -> Set<T> {
        Set { items: items.clone() }
    }

    /// Returns `true` if this `Set<T>` contains the given object.
    pub fn has(&self, l: &T) -> bool {
        self.items.iter().any(|x| x == l)
    }

    /// Returns the size of this `Set<T>`.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

/// Creates a `Set<T>` that is the union of two `Set<T>`.
///
/// The union of two sets R and S, is the set whose elements
/// belong either to R or to S or to both.
///
/// # Examples
///
/// ```rust
/// # use relations::{Set, union};
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

/// Creates a `Set<T>` that is the intersection of two `Set<T>`.
///
/// The intersection of two sets, R and S, is the set whose elements
/// belong to both R and S.
///
/// # Examples
///
/// ```rust
/// # use relations::{Set, inter};
/// let a = Set::new(&vec![0, 1, 2, 3, 4, 5]);
/// let b = Set::new(&vec![4, 5, 6, 7, 8, 9]);
/// let c = Set::new(&vec![4, 5]);
/// assert_eq!(inter(&a, &b), c);
/// ```
pub fn inter<T: Clone + Ord>(this: &Set<T>, that: &Set<T>) -> Set<T> {
    Set::new(&this.items
        .iter()
        .filter(|x| that.has(&x))
        .map(|x| x.clone())
        .collect())
}

/// Creates a `Set<T>` that is the complement of a `Set<T>` relative to
/// another `Set<T>`.
///
/// The complement of set, R, relative to a set, S, is the set whose
/// elements belong to S but not to R.
///
/// ```rust
/// # use relations::{Set, compl};
/// let a = Set::new(&vec![0, 1, 2, 3, 4, 5]);
/// let b = Set::new(&vec![4, 5, 6, 7, 8, 9]);
/// let c = Set::new(&vec![0, 1, 2, 3]);
/// assert_eq!(compl(&a, &b), c);
/// ```
pub fn compl<T: Clone + Ord>(this: &Set<T>, that: &Set<T>) -> Set<T> {
    Set::new(&this.items
        .iter()
        .filter(|x| !that.has(&x))
        .map(|x| x.clone())
        .collect())
}

/// Represents links between objects in a `Set<T>`.
#[derive(Debug)]
pub struct Relation<T> {
    set: Set<T>,
    links: Vec<(T, T)>,
}

impl<T: Ord + Clone> PartialEq for Relation<T> {
    fn eq(&self, other: &Relation<T>) -> bool {
        let ref a = *self.links;
        if self.links.len() != other.links.len() {
            return false;
        }
        for item in a {
            if !other.has(item) {
                return false;
            }
        }
        return true;
    }
}

impl<T: Ord + Clone> Relation<T> {

    /// Creates a new `Relation<T>` with a given `Set<T>` and a `Vec` of links.
    pub fn new(set: &Set<T>, links: &Vec<(T, T)>) -> Relation<T> {
        Relation {
            set: set.clone(),
            links: links.clone(),
        }
    }

    /// Adds a link of the form `(T, T)`.
    pub fn add_link(&mut self, l: (T, T)) {
        if !self.has(&l) && self.set.has(&l.0) && self.set.has(&l.1) {
            self.links.push(l);
        }
    }

    /// Adds each link in a `Vec` of links.
    pub fn add_links(&mut self, ls: Vec<(T, T)>) {
        for l in ls {
            if !self.has(&l) && self.set.has(&l.0) && self.set.has(&l.1) {
                self.add_link(l);
            }
        }
    }

    /// Returns `true` if this `Relation<T>` contains the given link.
    pub fn has(&self, l: &(T, T)) -> bool {
        self.links.iter().any(|x| x == l)
    }

    /// Creates a `Set<T>` containing all objects to which the given object
    /// is linked.
    pub fn neighbours(&self, v: &T) -> Set<T> {
        union(&self.links_to(&v), &self.links_from(&v))
    }

    /// Creates a `Set<T>` containing all objects to which the given object reaches.
    pub fn links_to(&self, v: &T) -> Set<T> {
        let mut a = Vec::new();
        for item in self.links.iter() {
            if item.0 == *v {
                a.push(item.1.clone());
            }
        }
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    /// Creates a `Set<T>` containing all objects from which the given object is
    /// reachable.
    pub fn links_from(&self, v: &T) -> Set<T> {
        let mut a = Vec::new();
        for item in self.links.iter() {
            if item.1 == *v {
                a.push(item.0.clone());
            }
        }
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    /// Returns the number of objects to which the given object is linked.
    pub fn degree(&self, v: &T) -> usize {
        self.neighbours(v).len()
    }

    /// Creates a `Set<T>` containing all objects that are the root of at
    /// least one link.
    pub fn domain(&self) -> Set<T> {
        let mut a = self.links.iter().map(|x| x.0.clone()).collect::<Vec<T>>();
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    /// Creates a `Set<T>` containing all objects that are the tail of at
    /// least one link.
    pub fn codomain(&self) -> Set<T> {
        let mut a = self.links.iter().map(|x| x.1.clone()).collect::<Vec<T>>();
        a.sort();
        a.dedup();
        Set::new(&a)
    }

    /// Returns `true` if the `Relation<T>` is reflexive.
    ///
    /// A `Relation<T>` is said to be reflexive on a set if it contains the link
    /// (x, x) for all x in the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use relations::{Set, Relation};
    /// assert!(Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 1)])
    ///        .is_reflexive());
    /// assert!(!Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 0)])
    ///        .is_reflexive());
    /// ```
    pub fn is_reflexive(&self) -> bool {
        self.set.items.iter().all(|x| self.has(&(x.clone(), x.clone())))
    }

    /// Returns `true` if the `Relation<T>` is symmetric.
    ///
    /// A `Relation<T>` is said to be symmetric on a set if it contains the link
    /// (x, y) for all (y, x).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use relations::{Set, Relation};
    /// assert!(Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 0)])
    ///         .is_symmetric());
    /// assert!(!Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 1)])
    ///         .is_symmetric());
    /// ```
    pub fn is_symmetric(&self) -> bool {
        self.links.iter().all(|x| self.has(&(x.1.clone(), x.0.clone())))
    }

    /// Returns `true` if the `Relation<T>` is transitive.
    ///
    /// A `Relation<T>` is said to be transitive on a set if for all (x, z),
    /// there is an object, y, in the set such that the `Relation<T>` contains
    /// (x, y) and (y, z).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use relations::{Set, Relation};
    /// assert!(Relation::new(&Set::new(&vec![0, 1]), &vec![(0, 0), (0, 1), (1, 0)])
    ///         .is_transitive());
    /// assert!(!Relation::new(&Set::new(&vec![0, 1, 2]), &vec![(0, 0), (0, 1), (2, 1)])
    ///         .is_transitive());
    /// ```
    pub fn is_transitive(&self) -> bool {
        let test = self.set.items.iter();
        for item1 in test.clone() {
            for item2 in test.clone() {
                if self.has(&(item1.clone(), item2.clone())) {
                    if !test.clone().any(|x| {
                        self.has(&(item1.clone(), x.clone())) &&
                            self.has(&(x.clone(), item2.clone()))
                    }) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Creates the `Relation<T>` transitive closure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use relations::{Set, Relation};
    /// let r = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
    /// &vec![(0, 0), (0, 1), (1, 3), (2, 1)]);
    /// let q = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
    /// &vec![(0, 0), (0, 1), (1, 3), (2, 1), (1, 1), (2, 2), (3, 3)]);
    /// assert_eq!(r.refl_closure(), q);
    /// ```
    pub fn refl_closure(&self) -> Relation<T> {
        let mut v = Vec::new();
        for item in self.set.items.iter() {
            v.push((item.clone(), item.clone()));
        }
        rel_union(&self, &Relation::new(&self.set, &v))
    }

    /// Creates the `Relation<T>` transitive closure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use relations::{Set, Relation};
    /// let r = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
    /// &vec![(0, 0), (0, 1), (1, 3), (2, 1)]);
    /// let q = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
    /// &vec![(0, 0), (0, 1), (1, 3), (2, 1), (0, 3), (2, 3)]);
    /// assert_eq!(r.trans_closure(), q);
    /// ```
    pub fn trans_closure(&self) -> Relation<T> {
        let mut v = Vec::new();
        v.extend_from_slice(self.links.as_slice());
        for k in self.set.items.iter() {
            for i in self.set.items.iter() {
                for j in self.set.items.iter() {
                    if !self.has(&(i.clone(), j.clone())) && self.has(&(i.clone(), k.clone())) &&
                        self.has(&(k.clone(), j.clone())) {
                            v.push((i.clone(), j.clone()));
                        }
                }
            }
        }
        Relation::new(&self.set, &v)
    }

    /// Creates the `Relation<T>` symmetric closure.
    ///
    /// The symmetric closure is generated by adding the link (y, x) for all
    /// (x, y) already in the `Relation<T>`.
    ///
    /// ```rust
    /// # use relations::{Set, Relation};
    /// let r = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
    /// &vec![(0, 0), (0, 1), (1, 3), (2, 1)]);
    /// let q = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
    /// &vec![(0, 0), (0, 1), (1, 3), (2, 1), (1, 0), (3, 1), (1, 2)]);
    /// assert_eq!(r.sym_closure(), q);
    /// ```
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

/// Creates a `Relation<T>` that is the union of two `Relation<T>'.
///
/// # Examples
///
/// ```rust
/// # use relations::{Set, Relation, rel_union};
/// let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 0), (4, 5)]);
/// let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(6, 6), (4, 5)]);
/// let c = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
/// &vec![(0, 0), (4, 5), (6, 6)]);
/// assert_eq!(rel_union(&a, &b), c);
/// ```
pub fn rel_union<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    let mut v = Vec::new();
    v.extend_from_slice(this.links.as_slice());
    v.extend_from_slice(that.links.as_slice());
    v.sort();
    v.dedup();
    Relation::new(&union(&this.set, &that.set), &v)
}

/// Creates a `Relation<T>` that is the intersection of two `Relation<T>'.
///
/// # Examples
///
/// ```rust
/// # use relations::{Set, Relation, rel_inter};
/// let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 0), (4, 5)]);
/// let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(6, 6), (4, 5)]);
/// let c = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
/// &vec![(4, 5)]);
/// assert_eq!(rel_inter(&a, &b), c);
/// ```
pub fn rel_inter<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    Relation::new(&inter(&this.set, &that.set),
    &this.links
    .iter()
    .filter(|x| that.has(&x))
    .map(|x| x.clone())
    .collect())
}

/// Creates a `Relation<T>` that is the complement of two `Relation<T>'.
///
/// # Examples
///
/// ```rust
/// # use relations::{Set, Relation, rel_compl};
/// let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 0), (4, 5)]);
/// let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(6, 6), (4, 5)]);
/// let c = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
/// &vec![(0, 0)]);
/// assert_eq!(rel_compl(&a, &b), c);
/// ```
pub fn rel_compl<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    Relation::new(&compl(&this.set, &that.set),
    &this.links
    .iter()
    .filter(|x| !that.has(&x))
    .map(|x| x.clone())
    .collect())
}

/// Creates a `Relation<T>` that is the composition of two `Relation<T>'.
///
/// # Examples
///
/// ```rust
/// # use relations::{Set, Relation, rel_compo};
/// let a = Relation::new(&Set::new(&vec![0, 1, 2, 3, 4, 5]), &vec![(0, 1), (1, 1)]);
/// let b = Relation::new(&Set::new(&vec![4, 5, 6, 7, 8, 9]), &vec![(1, 2), (1, 3)]);
/// let c = Relation::new(&Set::new(&vec![0, 1, 2, 3]),
/// &vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
/// assert_eq!(rel_compo(&a, &b), c);
/// ```
pub fn rel_compo<T: Clone + Ord>(this: &Relation<T>, that: &Relation<T>) -> Relation<T> {
    let mut v = Vec::new();
    for item in this.links.iter() {
        for item2 in that.links.iter().filter(|x| x.0 == item.1) {
            v.push((item.0.clone(), item2.1.clone()));
        }
    }
    Relation::new(&union(&this.domain(), &that.codomain()), &v)
}
