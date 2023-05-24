use std::time::UNIX_EPOCH;
use image::{GrayImage, Luma};

fn main() {
    let mut console_input = "".to_string();
    
    println!("Noise image height: ");
    std::io::stdin().read_line(&mut console_input).expect("failed to read line");
    let height = console_input.trim().parse::<u32>().expect("invalid height");

    console_input = "".to_string();

    println!("Noise image width: ");
    std::io::stdin().read_line(&mut console_input).expect("failed to read line");
    let width = console_input.trim().parse::<u32>().expect("invalid width");
    
    println!("image name: ");
    let mut image_name = "".to_string();
    std::io::stdin().read_line(&mut image_name).expect("failed to read line");

    println!("image path: ");
    let mut image_path = "".to_string();
    std::io::stdin().read_line(&mut image_path).expect("failed to read line");

    let mut noise_image = GrayImage::new(width, height); 

    for x in 0..noise_image.width(){
        for y in 0..noise_image.height(){
            noise_image.put_pixel(x, y, Luma([get_random_u8()]));
        } 
    }

    match noise_image.save(std::path::Path::new(format!("{}/{}.png", image_path.trim(), image_name.trim()).as_str())){
        Ok(_) => println!("successfully saved file"),
        Err(x) => println!("failed to create file, here is the error: '{}'", x),
    };
}

fn get_random_u8() -> u8{
    //X = (multiplier * seed + increment) % modulus do for iteration count
    //modulus > 0
    const GENERATOR_MODULUS: u128 = u128::pow(2, 97);
    //mutliplier > 0 && multiplier < modulus
    const GENERATOR_MULTIPLIER: u128 = 29;
    //iteration count
    const ITERATION_COUNT: u128 = 10;

    let mut seed_string = (std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() * 94031237).to_string();
    seed_string = seed_string[(seed_string.len() - 10)..].to_string(); 
    let mut result: u128 = seed_string.parse().unwrap();

    for _ in 0..ITERATION_COUNT{
        result = (GENERATOR_MULTIPLIER * result) % GENERATOR_MODULUS; 
    }

    while result > 255{
        result = result / 2;
    }

    return result as u8;
}
