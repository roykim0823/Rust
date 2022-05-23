use shape;

fn main() {
    let rect = shape::Rectangle::new(1.0, 2.0);
    let area = rect.get_feature(shape::Feature::Area);
    println!("rect area is {}", area);

    let circ = shape::Circle::new(3.0);
    let perimeter = circ.get_feature(shape::Feature::Perimeter);
    println!("circ perimeter is {}", perimeter);
}