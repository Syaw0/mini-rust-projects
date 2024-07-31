#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn rectangle_builder(width: u32, height: u32) -> Rectangle {
    Rectangle {
        height,
        width,
    }
}

fn main() {
    // * Step 1:
    // let width = 100;
    // let height = 140;

    // let area = rectangle_area(width, height);

    // * Step 2:
    // let rect1 = Rectangle { height: 140, width: 100 };
    // let area = rect1.area();

    // * Step 3:

    let rect1 = rectangle_builder(100, 140);

    let area = rect1.area();

    // println!("{:#?}", rect1);
    println!("the area of rectangle is : {area}");
    dbg!(&rect1);
}

// fn rectangle_area(width: u32, height: u32) -> u32 {
//     width * height
// }
