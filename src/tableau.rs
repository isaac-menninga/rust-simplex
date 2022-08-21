use crate::helpers::*;
use std::f64::INFINITY;

pub struct Tableau {
    pub objective: Vec<f64>,
    pub constraints: Vec<Vec<f64>>,
    pub n_variables: usize,
    pub n_constraints: usize,
    pub solution: Vec<f64>,
}

impl Tableau {
    pub fn new(mut o: Vec<f64>, n_vars: usize, n_cons: usize) -> Self {
        if o.len() != n_vars {
            panic!(
                "length of given vector does not equal expected number of variables in constraint."
            );
        }

        let mut o_slack = vec![];

        // add zeros for slack variables
        for _ in 0..n_cons {
            o_slack.push(0.0);
        }

        // 1 coeff for z, 0 for value
        o_slack.push(1.0);
        o_slack.push(0.0);

        for i in 0..o.len() {
            o[i] = -o[i]
        }

        o.append(&mut o_slack);

        Self {
            objective: o,
            n_variables: n_vars,
            n_constraints: n_cons,
            constraints: vec![],
            solution: vec![],
        }
    }

    pub fn push(&mut self, mut c: Vec<f64>, p: f64) {
        if c.len() != self.n_variables {
            panic!(
                "length of given vector does not equal expected number of variables in constraint."
            );
        }

        let mut slack = vec![0.0; self.n_constraints];
        slack[self.constraints.len()] = 1.0;
        slack.push(0.0);
        slack.push(p);

        c.extend_from_slice(&slack);

        self.constraints.push(c);
    }

    pub fn pivot(&mut self, r: usize, c: usize) {
        let mut row = self.constraints.get(r).unwrap().to_vec();
        let pivot_val = *row.get(c).unwrap();

        row.scale_by(1.0 / pivot_val);

        self.constraints[r] = row.to_vec();

        for (i, v) in self.constraints.iter_mut().enumerate() {
            if i != r {
                v.reduce(row.to_vec(), c);
            }
        }
        self.objective.reduce(row.to_vec(), c);
    }

    pub fn solve(&mut self) {
        // get minimum coefficient in the objective function
        let m = self.objective.minimum();

        // if the minimum is negative, then pivot table and call solve() again
        if m.1 < 0.0 {
            let mut pivot_row = (0, INFINITY);

            for i in 0..self.constraints.len() {
                let x = self.constraints.get(i).unwrap().last().unwrap();
                let y = self.constraints.get(i).unwrap().get(m.0).unwrap();
                if x / y < pivot_row.1 {
                    pivot_row.1 = x / y;
                    pivot_row.0 = i;
                }
            }

            self.pivot(m.0, pivot_row.0);

            self.solve();

        // if there are no negative coefficients, search tableau for basic variables
        } else {
            for i in 0..self.n_variables + 1 {
                let mut column: Vec<f64> = vec![];
                let mut is_basic: bool = false;
                let mut value: Option<f64> = None;

                for j in 0..&self.constraints.len() + 1 {
                    let v;

                    // if we're at the last iteration, get v from self.objective rather than self.constraints
                    if j == self.constraints.len() {
                        v = *self.objective.get(i).unwrap();
                    } else {
                        let x = self.constraints.get(j).unwrap();

                        v = *x.get(i).unwrap();
                    }

                    column.push(v);

                    if v == 0.0 {
                    } else if v == 1.0 {
                        if is_basic {
                            is_basic = false;
                            break;
                        } else {
                            is_basic = true;
                            let x = self.constraints.get(j).unwrap();
                            value = Some(*x.last().unwrap());
                        }
                    } else {
                        is_basic = false;
                        break;
                    }
                }

                if is_basic {
                    match value {
                        Some(i) => self.solution.push(i),
                        None => {}
                    }
                } else {
                    self.solution.push(0.0);
                }
            }
        }
    }

    pub fn print(&self) {
        for c in &self.constraints {
            println!("{:?}", c);
        }
        println!("{:?}", self.objective);
    }
}
