#[derive(Debug)]
struct Child<T> {
    id: T,
    children: Option<Vec<usize>>,
}

impl<T> Child<T> {
    fn new(id: T) -> Self { Child { id, children: None } }
}

#[derive(Debug)]
pub struct ChildGraph<T>(Vec<Child<T>>);

impl<T> ChildGraph<T>
where
    T: Sized + PartialEq + Copy + Clone,
{
    pub fn with_capacity(s: usize) -> Self { ChildGraph(Vec::with_capacity(s)) }

    pub fn insert(&mut self, req: T) -> usize {
        if !self.contains(req) {
            let idx = self.0.len();
            self.0.push(Child::new(req));
            idx
        } else {
            self.0
                .iter()
                .enumerate()
                .find(|(_, e)| e.id == req)
                .map(|(i, _)| i)
                .unwrap()
        }
    }

    pub fn insert_child(&mut self, parent: usize, child: T) -> usize {
        let c_idx = self.0.len();
        self.0.push(Child::new(child));
        let parent = &mut self.0[parent];
        if let Some(ref mut v) = parent.children {
            v.push(c_idx);
        } else {
            let mut v = Vec::with_capacity(5);
            v.push(c_idx);
            parent.children = Some(v);
        }

        c_idx
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> { self.0.iter().map(|r| &r.id) }

    pub fn contains(&self, req: T) -> bool { self.0.iter().any(|r| r.id == req) }
}
