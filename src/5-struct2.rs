struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    
    fn perimeter(&self, num: i32) -> i32 {
        num * (self.width * self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let rect = Rect{
        width: 5,
        height: 10
    };
    println!("Area of the rectangle is: {}",rect.area());
    println!("Perimeter of the rectangle is: {}",rect.perimeter(2));
    println!("Debug of the rectangle is: {}",Rect::debug());
}