mod helpers;
mod tableau;
use crate::tableau::*;

fn main() {
    let mut t = Tableau::new(vec![3.0, 2.0, 1.0], 3, 3);

    t.push(vec![2.0, 1.0, 1.0], 6.0);
    t.push(vec![1.0, 3.0, 2.0], 4.0);
    t.push(vec![1.0, 1.0, 4.0], 6.0);

    t.print();
    t.solve();

    println!("solution: ");
    for (i, v) in t.solution.iter().enumerate() {
        print!("x{i} = {}, ", v)
    }
    print!("value = {}", t.objective.last().unwrap());
}
