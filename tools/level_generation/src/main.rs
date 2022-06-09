use crate::bag_fillings::generate;

mod bag_fillings;

fn main() {
    let bags = generate(4, 5);
    dbg!(&bags, bags.len());
}
