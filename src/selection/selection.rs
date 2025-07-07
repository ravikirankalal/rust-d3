// Selection implementation

pub struct Selection<T> {
    data: Vec<T>,
}

impl<T> Selection<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn map<U, F>(&self, mut f: F) -> Selection<U>
    where
        F: FnMut(&T) -> U,
    {
        Selection {
            data: self.data.iter().map(|d| f(d)).collect(),
        }
    }

    pub fn filter<F>(&self, mut f: F) -> Selection<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        Selection {
            data: self.data.iter().cloned().filter(|d| f(d)).collect(),
        }
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        for d in &self.data {
            f(d);
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    /// D3.js data join API: replaces the current data with new data, returns a new selection.
    pub fn set_data<U: Clone>(&self, new_data: Vec<U>) -> Selection<U> {
        // Implements a simple data join: returns new selection with new data
        Selection::new(new_data)
    }

    /// D3.js enter selection: returns items in new_data not in current data
    pub fn enter<U: Clone + Eq + PartialEq<T>>(&self, new_data: Vec<U>) -> Selection<U> where T: PartialEq<U> {
        let enter_data = new_data.into_iter().filter(|d| !self.data.iter().any(|x| x == d)).collect();
        Selection::new(enter_data)
    }

    /// D3.js exit selection: returns items in current data not in new_data
    pub fn exit<U: Clone + Eq + PartialEq<T>>(&self, new_data: Vec<U>) -> Selection<T> where T: Clone + PartialEq<U> {
        let exit_data = self.data.iter().cloned().filter(|d| !new_data.iter().any(|x| x == d)).collect();
        Selection::new(exit_data)
    }

    /// D3.js join: returns a tuple of (enter, update, exit) selections
    pub fn join<U: Clone + Eq + PartialEq<T>>(&self, new_data: Vec<U>) -> (Selection<U>, Selection<U>, Selection<T>) where T: Clone + PartialEq<U> {
        let mut enter = Vec::new();
        let mut update = Vec::new();
        let mut exit = Vec::new();
        let old_len = self.data.len();
        let new_len = new_data.len();
        let min_len = old_len.min(new_len);
        // Update: items with both old and new data (by index)
        for i in 0..min_len {
            update.push(new_data[i].clone());
        }
        // Enter: new data with no corresponding old data
        for i in min_len..new_len {
            enter.push(new_data[i].clone());
        }
        // Exit: old data with no corresponding new data
        for i in min_len..old_len {
            exit.push(self.data[i].clone());
        }
        (Selection::new(enter), Selection::new(update), Selection::new(exit))
    }

    pub fn select<F>(&self, mut f: F) -> Selection<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        if let Some(item) = self.data.iter().find(|d| f(d)) {
            Selection { data: vec![item.clone()] }
        } else {
            Selection { data: Vec::new() }
        }
    }

    pub fn select_all<F>(&self, mut f: F) -> Selection<T>
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        Selection {
            data: self.data.iter().cloned().filter(|d| f(d)).collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn nodes(&self) -> Vec<&T> {
        self.data.iter().collect()
    }

    pub fn node(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn call<F>(&self, f: F)
    where
        F: FnOnce(&Selection<T>),
    {
        f(self);
    }

    pub fn each<F>(&self, mut f: F)
    where
        F: FnMut(&T, usize),
    {
        for (i, d) in self.data.iter().enumerate() {
            f(d, i);
        }
    }

    pub fn sort_by<F>(&self, mut cmp: F) -> Selection<T>
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
        T: Clone,
    {
        let mut data = self.data.clone();
        data.sort_by(|a, b| cmp(a, b));
        Selection { data }
    }

    pub fn merge(&self, other: &Selection<T>) -> Selection<T>
    where
        T: Clone,
    {
        let mut data = self.data.clone();
        data.extend(other.data.iter().cloned());
        Selection { data }
    }

    // D3.js selection API stubs for full parity
    pub fn attr(&mut self, name: &str, value: &str) -> &mut Self
    where
        T: AttrSet,
    {
        for d in &mut self.data {
            d.set_attr(name, value);
        }
        self
    }
    pub fn style(&mut self, name: &str, value: &str) -> &mut Self
    where
        T: StyleSet,
    {
        for d in &mut self.data {
            d.set_style(name, value);
        }
        self
    }
    pub fn property(&mut self, name: &str, value: &str) -> &mut Self
    where
        T: PropertySet,
    {
        for d in &mut self.data {
            d.set_property(name, value);
        }
        self
    }
    pub fn classed(&mut self, name: &str, value: bool) -> &mut Self
    where
        T: ClassedSet,
    {
        for d in &mut self.data {
            d.set_classed(name, value);
        }
        self
    }
    pub fn text(&mut self, value: &str) -> &mut Self
    where
        T: TextSet,
    {
        for d in &mut self.data {
            d.set_text(value);
        }
        self
    }
    pub fn html(&mut self, value: &str) -> &mut Self
    where
        T: HtmlSet,
    {
        for d in &mut self.data {
            d.set_html(value);
        }
        self
    }
}

pub trait AttrSet { fn set_attr(&mut self, name: &str, value: &str); }
pub trait StyleSet { fn set_style(&mut self, name: &str, value: &str); }
pub trait PropertySet { fn set_property(&mut self, name: &str, value: &str); }
pub trait ClassedSet { fn set_classed(&mut self, name: &str, value: bool); }
pub trait TextSet { fn set_text(&mut self, value: &str); }
pub trait HtmlSet { fn set_html(&mut self, value: &str); }

// Re-export traits for downstream use
// (Do not re-export if already in scope)
