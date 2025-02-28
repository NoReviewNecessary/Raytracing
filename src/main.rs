fn main() {


    let window_len: usize = 64;
    let window_height: usize = 48;
    let shape_len: usize = 20;

    create_diamond(window_len, window_height, shape_len);

}


fn create_diamond(window_len: usize, window_height:usize, shape_len: usize)-> Vec<[usize;2]>{
    //Uses formula for diamond shading and creates 2 dim vector of shaded coords
    //Creates shape in center of rendered area
    
    let offset_len = window_len / 2;
    let offset_height = window_height / 2;
    
    //Start with x and y intercepts
    let intercepts = get_intercepts(shape_len);
    println!("intercepts length is {}", intercepts.len());

    let shaded_points = plot(intercepts, shape_len as i32);

    render(window_len, window_height, &shaded_points);

    println!("{:?}", shaded_points);
    return Vec::new();

}

fn get_intercepts(shape_len: usize) -> Vec<[i32;2]>{ 
    //Both diamond and circle intercepts are the same as +- shape_len

    let mut intercepts: Vec<[i32;2]> = Vec::new();
    let intercept_coord:i32 = shape_len as i32;

    //TODO: we can do the cross join of [0,0] and [intercept_coord, -intercept_coord]
    intercepts.push([-intercept_coord,0]);
    intercepts.push([0, intercept_coord]);
    intercepts.push([intercept_coord, 0]);
    intercepts.push([0, -intercept_coord]);

    intercepts
}

fn plot(mut intercepts: Vec<[i32;2]>, shape_len: i32) -> Vec<[i32;2]>{

    println!("intercepts.len() = {}", intercepts.len());
    let mut shaded_points = Vec::new();

    println!("intercepts.len() = {}", intercepts.len());
    println!("intercepts[0].len() = {}", intercepts[0].len());
    for i in 0..intercepts.len(){
        let int_1 = i;
        let mut int_2 = i + 1;
        if int_2 == intercepts.len(){
            int_2 = 0;
        }
        
        // Get slope between two intercept points
        //let slope = (intercepts[int_2][1] - intercepts[int_1][1]) / (intercepts[int_2][0] - intercepts[int_1][0]);
        
        // Determine to use + or - in +- circle function
        let mut pos_or_neg: i32 = 0;
        if intercepts[int_1][0] < intercepts[int_2][0]{
            pos_or_neg = 1;
        }
        else{
            pos_or_neg = -1;
        }

        let mut current_x = intercepts[int_1][0];
        let ending_x = intercepts[int_2][0];

        println!("Current x: {} Ending x: {}", current_x, ending_x);

        let mut y_int = 0;
        // Get y intercept
        if intercepts[int_1][1] == 0{
            y_int = intercepts[int_2][1];
        }
        else{
            y_int = intercepts[int_1][1];
        }

        println!("plotting");
        //iterate to int_2 x and find points between
        while current_x != ending_x{
            if current_x < ending_x{
                current_x += 1;
            }
            else{
                current_x -= 1;
            }
            //let current_y = slope*current_x + y_int;
            let current_y = pos_or_neg * (f64::sqrt(((shape_len * shape_len) - (current_x * current_x)) as f64) as i32);
            shaded_points.push([current_x, current_y]);
        }
    }
    shaded_points.append(&mut intercepts);
    return shaded_points;
}



fn render(window_len: usize, window_height: usize, shaded: &Vec<[i32;2]>) -> (){
    for i in 0..window_height{
        let y_value: i32 = (i as i32) - 24;
        for j in 0..window_len{
            let x_value: i32 = (j as i32) - 32;
            if shaded.contains(&[x_value, y_value]){
                print!("\x1b[38;2;255;0;0m█ ");
            }
            else{
                print!("\x1b[38;255;255;0;0m█ ");
            }

        }
        print!("\n");
    }

}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_intercepts(){
        assert_eq!(get_intercepts(2), vec![[-2, 0], [0, 2], [2, 0], [0, -2]]);
        assert_eq!(get_intercepts(5), vec![[-5, 0], [0, 5], [5, 0], [0, -5]]);
    }
}

