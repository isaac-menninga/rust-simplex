pub trait Helpers {
    fn minimum(&self) -> (usize, f64);
    fn scale_by(&mut self, j: f64);
    fn reduce(&mut self, r: Vec<f64>, c: usize);
    fn diff(&mut self, r: Vec<f64>);
}

impl Helpers for Vec<f64> {
    fn minimum(&self) -> (usize, f64) {
        let min_value = self.iter().fold(f64::INFINITY, |a, b| a.min(*b));
        let index = self.iter().position(|v| v <= &min_value).unwrap();

        (index, min_value)
    }

    fn scale_by(&mut self, j: f64) {
        self.iter_mut().for_each(|v| *v *= j);
    }

    fn diff(&mut self, r: Vec<f64>) {
        self.iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = *v - r.get(i).unwrap());
    }

    fn reduce(&mut self, c: Vec<f64>, r: usize) {
        let mut c = c;
        c.scale_by(*self.get(r).unwrap());
        self.diff(c);
    }
}
