use crate::bag_fillings::generate;

mod bag_fillings;

fn main() {
    for xy in [(3, 4), (4, 4), (4, 5), (5, 5)] {
        let bags = generate(xy.0, xy.1);
        dbg!(xy, bags.len());
    }
}
