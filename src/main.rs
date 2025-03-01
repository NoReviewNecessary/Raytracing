use std::fs;


fn main() {

    let window_len: usize = 400;
    let window_height: usize = 300;
    let shape_len: usize = 50;

    create_diamond(window_len, window_height, shape_len);

}


fn create_diamond(window_len: usize, window_height:usize, shape_len: usize)-> Vec<[usize;2]>{
    //Uses formula for diamond shading and creates 2 dim vector of shaded coords
    //Creates shape in center of rendered area
    
    let offset_len = window_len / 2;
    let offset_height = window_height / 2;
    
    render(window_len, window_height, shape_len);

    return Vec::new();

}


fn render(window_len: usize, window_height: usize, shape_len: usize) -> (){
    let mut data = format!("P3\n{} {}\n255\n", window_len, window_height);

    let r_2: i32 = (shape_len * shape_len) as i32;
    for i in 0..window_height{
        let y_value: i32 = (i) as i32 - (window_height/2) as i32;
        for j in 0..window_len{
            let x_value: i32 = (j) as i32 - (window_len/2) as i32;
            if (y_value * y_value) + (x_value * x_value) < r_2{
                data += "255 255 255 ";
            }
            else{
                data += "0 0 0 ";
            }
        }
        data += "\n";
    }

    fs::write("ppm_test1.ppm", data);

}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_intercepts(){
        assert_eq!(get_intercepts(2), vec![[-2, 0], [0, 2], [2, 0], [0, -2]]);
        assert_eq!(get_intercepts(5), vec![[-5, 0], [0, 5], [5, 0], [0, -5]]); } }
