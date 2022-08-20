// input is objective function
// methods for adding constraints
// method for has_solution
pub struct Tableau {
    pub objective: Vec<f64>,
    pub constraints: Vec<Vec<f64>>,
    pub n_variables: usize,
    pub n_constraints: usize,
}

impl Tableau {
    pub fn new(mut o: Vec<f64>, n_vars: usize, n_cons: usize) -> Self {
        if o.len() != n_vars {
            panic!(
                "length of given vector does not equal expected number of variables in constraint."
            );
        }

        let mut o_slack = vec![];

        for _ in 0..n_cons {
            o_slack.push(0.0);
        }

        // add zeros for slack variables
        o_slack.push(1.0);

        // 1 coeff for z, 0 for value
        o_slack.push(0.0);

        o.append(&mut o_slack);

        Self {
            objective: o,
            n_variables: n_vars,
            n_constraints: n_cons,
            constraints: vec![],
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

    pub fn print(&self) {
        for c in &self.constraints {
            println!("{:?}", c);
        }
        println!("{:?}", self.objective);
    }
}
