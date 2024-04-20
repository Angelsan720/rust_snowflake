
use rand::prelude::*;
use std::time::{Duration, SystemTime};
use image;
use num_complex;
fn pretty_print_bool_matrix(matrix : Vec<Vec<bool>>) -> () {

    let x_size = matrix.len();
    let y_size = matrix[0].len();

    let top    = "▀";
    let bottom = "▄";
    let full   = "█";
    let blank  = " ";
    let filler = "#";
    let mut s = String::new();
    s.push_str(full);
    s.push_str(top);

    for _ in 0..x_size {
        s.push_str(top);
    }
    s.push_str(top);
    s.push_str(format!("{}\n", full).as_str());


    for y in 0..y_size/2 {
        let mut tmp = String::new();
        tmp.push_str(full);
        tmp.push_str(blank);

        for x in 0..x_size {
            if matrix[x][2*y] && matrix[x][(2*y)+1] {
                tmp.push_str( blank); //add blank
            } else if matrix[x][2*y] && !matrix[x][(2*y)+1] {
                tmp.push_str(top); //add top
            } else if !matrix[x][2*y] && matrix[x][(2*y)+1] {
                tmp.push_str(bottom); //add bottom
            } else if !matrix[x][2*y] && !matrix[x][(2*y)+1] {
                tmp.push_str( blank); //add blank
            } else {
                tmp.push_str( filler);
            }
        }

        tmp.push_str(blank);
        tmp.push_str( full);

        s.push_str(format!("{}\n", tmp).as_str());
    }
    if y_size % 2 == 1 {
        let mut tmp = String::new();
        tmp.push_str(full);
        tmp.push_str(blank);

        for x in 0..x_size {
            if matrix[x][y_size-1]{
                tmp.push_str(top);
            } else if !matrix[x][y_size-1]{
                tmp.push_str( blank);
            } else {
                tmp.push_str( filler);
            }

        }
        tmp.push_str(blank);
        tmp.push_str( full);
        s.push_str(format!("{}\n", tmp).as_str());
        let mut tmp = String::new();
        tmp.push_str(top);
        tmp.push_str(top);
        for _ in 0..x_size {
            tmp.push_str(top)
        }
        tmp.push_str(top);
        tmp.push_str(top);
        s.push_str(format!("{}\n", tmp).as_str());
    } else     if y_size % 2 == 0 {
        let mut tmp = String::new();
        tmp.push_str(full);
        tmp.push_str(bottom);

        for x in 0..x_size {
            if matrix[x][y_size-1]{
                tmp.push_str(top);
            } else if !matrix[x][y_size-1]{
                tmp.push_str( bottom);
            } else {
                tmp.push_str( filler);
            }

        }
        tmp.push_str(bottom);
        tmp.push_str( full);
        s.push_str(format!("{}\n", tmp).as_str());

    }




    print!("{}", s);
}
fn random_range(min : usize, max : usize) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}
fn check_borders(matrix : Vec<Vec<bool>>, edge_size : usize) -> bool {
    let x_size = matrix.len();
    let y_size = matrix[0].len();
    for x in 0..x_size {
        for y in 0..edge_size {
            if matrix[x][y] {
                return false;
            }
        }
        for y in y_size-edge_size..y_size {
            if matrix[x][y] {
                return false;
            }
        }
    }
    for y in edge_size..y_size-edge_size {
        for x in 0..edge_size {
            if matrix[x][y] {
                return false;
            }            
        }
        for x in x_size-edge_size..x_size {
            if matrix[x][y] {
                return false;
            }            
        }
    }

    return true;
}
fn check_proximity(matrix : Vec<Vec<bool>>, x : usize, y : usize) -> bool {
    let x_size = matrix.len();
    let y_size = matrix[0].len();
    if x < x_size-1 && matrix[x+1][y] {
        return false;
    } else if x > 0 && matrix[x-1][y] {
        return false;
    } else if y < y_size-1 && matrix[x][y+1] {
        return false;
    } else if y > 0 && matrix[x][y-1] {
        return false;
    } else {
        return true;
    }      
}
fn walk(matrix : Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let x_size = matrix.len();
    let y_size = matrix[0].len();

    let mut x = random_range(0, x_size);
    let mut y = random_range(0, y_size);
    while matrix[x][y] {
        x = random_range(0, x_size);
        y = random_range(0, y_size);
    }

    while check_proximity(matrix.clone(), x, y) {
        let choice = random_range(0, 4);
        if choice == 0 && x < x_size-1 {
            x = x + 1;
        } else if choice == 1 && x > 0 {
            x = x - 1;
        } else if choice == 2 && y < y_size-1 {
            y = y + 1;            
        } else if choice == 3 && y > 0 {
            y = y - 1;            
        }
    }
    let mut tmp = matrix;
    tmp[x][y] = true;

    return tmp;
}
fn main() {
    let image_size_x = 500;
    let image_size_y = 500;

    let mut image: Vec<Vec<bool>> = Vec::with_capacity(image_size_x);
    for _ in 0..image_size_x {
        let row = vec![false; image_size_y]; // Create a row vector with zeros of size 'columns'
        image.push(row); // Push the row vector to the matrix
    }
    image[image_size_x/2][image_size_y/2] = true;

    //pretty_print_bool_matrix(image);
    let mut now = SystemTime::now();
    let i = 0;
    while check_borders(image.clone(),  3) {
        if SystemTime::now().duration_since(now).unwrap() > Duration::from_millis(16) {
            print!("Cycle: {}\r", i);
            //pretty_print_bool_matrix(image.clone());
            //println!("");
            now = SystemTime::now();
        }

        
        image = walk(image.clone());        
    }
    let mut imgbuf = image::ImageBuffer::new(image_size_x.try_into().unwrap(), image_size_y.try_into().unwrap());

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if image[usize::try_from(x).unwrap()][usize::try_from(y).unwrap()] {
            *pixel = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        } else {
            *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        }
    }
    imgbuf.save("snowflake.png").unwrap();

}



