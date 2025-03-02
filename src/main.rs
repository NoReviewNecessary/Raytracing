use std::fs::File;
use std::io::Write;

enum Shape{
    Circle {radius: usize, origin: Point},
    Square {side: usize, origin: Point},
}

struct Pixel{
    // RGB format
    red: u8,
    green: u8,
    blue: u8,
}

struct Window{
    // Defines the width and height of the file to be rendered
    width: usize,
    height: usize
}

struct Point{
    x:i32,
    y:i32
}

impl Shape{
    fn shaded(&self, x_value:i32, y_value:i32)-> bool{
        match self{
            Shape::Circle { radius, origin } => {
                y_value.pow(2) + x_value.pow(2) < (*radius as i32).pow(2)
            }
            Shape::Square { side, origin } => {
                x_value.abs() <= *side as i32 && y_value.abs() <= *side as i32
            } 
        }
    }
    fn origin(&self) -> &Point{
        match self{
            Shape::Circle {origin, .. } => origin,
            Shape::Square {origin, .. } => origin,
        }
    }
}

impl Window{
    // Functions below translate x and y to graph style coordinates where
    // (0,0) is considered the center of the window. They take in offset as 
    // a parameter in case the user wants to plot somewhere besides the center
    fn translate_x(&self, x_value:i32, x_offset:i32) -> i32{
        (x_value + x_offset) - (self.width/2) as i32 
    }
    fn translate_y(&self, y_value:i32, y_offset:i32) -> i32{
        (y_value + y_offset) - (self.height/2) as i32
    }
}

fn main() {
    
    let window = Window{
        width: 400,
        height: 300,
    };

   // let shape = Shape::Circle(Circle {radius: 50});
    let shape = Shape::Square {
        side: 5, 
        origin: Point { x: 0, y: 0}
    };
    let pixel_grid = create_pixel_grid(window, shape);

    write_ppm3(&pixel_grid, "ppm_example2.ppm");

}

fn create_pixel_grid(window: Window, shape: Shape) -> Vec<Vec<Pixel>>{
    let mut grid = Vec::with_capacity(window.height);

    for y in 0..window.height{
        let y_value: i32 = window.translate_y(y as i32, shape.origin().y);
        let mut row = Vec::with_capacity(window.width);
        for x in 0..window.width{
            let x_value: i32 = window.translate_x(x as i32, shape.origin().x);
            if shape.shaded(x_value, y_value){
                row.push(Pixel{
                    red: 255,
                    green: 255,
                    blue: 255
                })
            }
            else{
                row.push(Pixel{
                    red: 0,
                    green: 0,
                    blue: 0
                })
            }
        }
        grid.push(row);
    }
    return grid;
}

fn write_ppm3(pixel_grid: &Vec<Vec<Pixel>>, filename: &str) -> std::io::Result<()>{ 
    let grid_length = pixel_grid[0].len();
    let grid_height = pixel_grid.len();
    let mut file = File::create(filename)?;

    //Header information
    writeln!(file, "P3"); // Defines the file type to the reader
    writeln!(file, "{} {}", grid_length, grid_height); // Sets the pixel dimension of the result PPM file
    writeln!(file, "255"); // Sets ppm to read RGB values between 0..255

    for row in pixel_grid{
        for pixel in row{
            write!(file, "{} {} {} ", pixel.red, pixel.green, pixel.blue)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

//TODO
fn horizontal_gradient(window_len: usize, color: Pixel){

}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_circle_shade(){
        let circle = Shape::Circle { 
            radius: 5, 
            origin: Point { x: 0, y: 0}
        };
        assert_eq!(circle.shaded(3,4), false);
        assert_eq!(circle.shaded(7,10), false);
        assert_eq!(circle.shaded(0,0), true);

    }

    #[test]
    fn test_square_shade(){
        let square = Shape::Square {
            side: 5, 
            origin: Point { x: 0, y: 0}
        };
        assert_eq!(square.shaded(3,4), true);
        assert_eq!(square.shaded(7,10), false);
        assert_eq!(square.shaded(0,0), true);

    }
    #[test]
    fn check_pows(){
        assert_eq!((3 as i32).pow(2), 9);
        assert_eq!((6 as i32).pow(2), 36);
    }

}
