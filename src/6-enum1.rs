enum Shape {
    Rectangle,
    Circle
}

fn main() {
    let my_shape = Shape::Rectangle;
    print_area(my_shape);
}

fn print_area(shape: Shape){
    println!("Print area of shape");
}