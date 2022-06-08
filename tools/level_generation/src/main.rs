use crate::bag_fillings::generate;

mod bag_fillings;

fn main() {
    let bags = generate(3, 4);
    dbg!(&bags, bags.len());
}
