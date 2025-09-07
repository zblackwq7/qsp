pub struct Seq<T>(Vec<T>);

impl<T> Seq<T> {
    pub fn once(t: T) -> Self {
        Self(vec![t])
    }
}

impl<T> FromIterator<T> for Seq<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for Seq<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Seq<T> {
    type Item = &'a T;

    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T> From<Vec<T>> for Seq<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
