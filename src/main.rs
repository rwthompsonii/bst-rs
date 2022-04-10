use bst_rs::Bst;

fn main() {
    // build sample tree containing 3, 5
    let mut sample = Bst::new(5);
    sample.insert(3);
    // clone
    let sample2 = sample.clone();
    // add 7 to original tree (should not affect clone)
    sample.insert(7);

    // print trees
    println!("Testing insert\n{}\n{}", sample, sample2);
    println!("Testing iterator");
    for val in sample.iter() {
        println!("{}", val);
    }
    println!("Testing sum\n{}", sample.sum());
}
