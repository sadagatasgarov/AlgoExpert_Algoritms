use std::cmp::Reverse;


fn class_photos(mut red_shirt_heights: Vec<usize>, mut blue_shirt_heights: Vec<usize>) {
    red_shirt_heights.sort_by(|a, b| b.cmp(a));
    blue_shirt_heights.sort_by(|a, b| b.cmp(a));


    let mut shirt_color_in_first_row = "";
    let mut red_shirt_height = 0;
    let mut blue_shirt_height = 0;

    if red_shirt_heights[0] < blue_shirt_heights[0] {
        shirt_color_in_first_row = "RED";
    } else {
        shirt_color_in_first_row = "BlUE";
    }

    for idx in 0..red_shirt_heights.len() {
        red_shirt_height = red_shirt_heights[idx];
        blue_shirt_height = blue_shirt_heights[idx];
 
        if shirt_color_in_first_row == "RED" {
            if red_shirt_height >= blue_shirt_height {
                println!("false")
            } 
        } else {
            if blue_shirt_height >= red_shirt_height {
                println!("false")
                
            }
        }
    }
    println!("true")
}


fn main() {
    let red_shirts= vec![5, 8, 1, 3, 4, 33];
    let blue_shirts= vec![6, 9, 2, 4, 5, 11];  

    class_photos(red_shirts, blue_shirts);
    // red_shirts.sort_by(|a, b| b.cmp(a));
    // red_shirts.sort_by_key(|w| Reverse(*w));
    // println!("{:?}", red_shirts)
}
