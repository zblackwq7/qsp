pub enum Seq<T, I> {
    Iter(I),
    Vec(Vec<T>),
}

pub enum SeqIter<T, I> {
    Iter(I),
    Vec(<Vec<T> as IntoIterator>::IntoIter),
}

impl<T, I: Iterator<Item = T>> Iterator for SeqIter<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SeqIter::Iter(mut ref i) => i.next(),
            SeqIter::Vec(mut ref i) => i.next(),
        }
    }
}

impl<T, I> IntoIterator for Seq<T, I> {
    type Item = T;
    type IntoIter = SeqIter<T, I>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Seq::Iter(i) => SeqIter::Iter(i),
            Seq::Vec(items) => SeqIter::Vec(items.into_iter()),
        }
    }
}

impl<T> Seq<Seq<T>> {
    pub fn flatten(self) -> Seq<T> {
        Seq(self.0.into_iter().flatten().collect())
    }
}
