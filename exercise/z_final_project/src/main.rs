// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

extern crate clap; // cargo add clap, or add clap = "4.4.18" to the Cargo.toml file
use clap::{Arg, Command};
use image::DynamicImage;
use std::fmt;

#[derive(Debug)]
enum Rot {
    R0,
    R90,
    R180,
    R270
}

impl fmt::Display for Rot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/

    // Let's use Clap
    let matches = Command::new("Image Editor")
                        .version("1.0")
                        .author("Amar Bhatt <bhatt.amar.a@gmail.com")
                        .about("Image processing application")
                        .help_template("\
                         {before-help}{name} {version}
                         {author-with-newline}{about-with-newline}
                         {usage-heading} {usage}
                         
                         {all-args}{after-help}")
                        .flatten_help(true)
                        .arg_required_else_help(true)
                        .arg(Arg::new("infile")
                            .short('i')
                            .long("infile")
                            .value_name("INFILE")
                            .help("Input image to process (required for blur, brighten, crop, rotate, invert, grayscale)")
                            .default_value(""))
                        .arg(Arg::new("outfile")
                            .short('o')
                            .long("outfile")
                            .value_name("OUTFILE")
                            .help("Output file")
                            .required(true))
                        .subcommand(Command::new("blur")
                            .long_flag("blur")
                            .about("Blur image by x")
                            .arg(Arg::new("value")
                                .short('v')
                                .help("Blur value")
                                .required(true)))
                        .subcommand(Command::new("brighten")
                            .long_flag("brighten")
                            .about("Brighten image by x")
                            .arg(Arg::new("value")
                                .short('v')
                                .allow_hyphen_values(true) // allows negative number inputs
                                .help("Change image brightness, negative numbers will darken image and positive numbers will brighten")
                                .required(true)))
                        .subcommand(Command::new("crop")
                            .long_flag("crop")
                            .about("Crop image")
                            .arg(Arg::new("x")
                                .long("x")
                                .help("Crop origin X")
                                .default_value("0"))
                            .arg(Arg::new("y")
                                .long("y")
                                .help("Crop origin Y")
                                .default_value("0"))
                            .arg(Arg::new("w")
                                .long("w")
                                .help("Crop width")
                                .required(true))
                            .arg(Arg::new("h")
                                .long("h")
                                .help("Crop height")
                                .required(true)))
                        .subcommand(Command::new("rotate")
                            .long_flag("rotate")
                            .about("Rotate image by 90, 180, 270")
                            .arg(Arg::new("value")
                                .short('v')
                                .help("Rotate image by 90, 180, 270")
                                .required(true)))
                        .subcommand(Command::new("invert")
                            .long_flag("invert")
                            .about("Invert image colors"))    
                        .subcommand(Command::new("grayscale")
                            .long_flag("grayscale")
                            .about("Grayscale image colors"))
                        .subcommand(Command::new("generate")
                            .long_flag("generate")
                            .about("Generate solid image")
                            .arg(Arg::new("r")
                                .long("r")
                                .help("Red [0 255]")
                                .default_value("0"))
                            .arg(Arg::new("g")
                                .long("g")
                                .help("Green [0 255]")
                                .default_value("0"))
                            .arg(Arg::new("b")
                                .long("b")
                                .help("Blue [0 255]")
                                .default_value("0"))) 
                            .subcommand(Command::new("fractal")
                                .long_flag("fractal")
                                .about("Generate fractal"))
                        .get_matches();

    let infile: String = matches.get_one::<String>("infile").unwrap().to_string();
    let outfile: String = matches.get_one::<String>("outfile").unwrap().to_string();

    // now let's check for the commands
    match matches.subcommand() {
        Some(("blur", sub_m)) => {
            let val = sub_m.get_one::<String>("value").unwrap().parse::<f32>().expect("value must be of type f32");
            blur(infile, outfile,val)
        },
        Some(("brighten", sub_m)) => {
            let val = sub_m.get_one::<String>("value").unwrap().parse::<i32>().expect("value must be of type i32");
            brighten(infile, outfile,val);
        },     
        Some(("crop", sub_m)) => {
            let x = sub_m.get_one::<String>("x").unwrap().parse::<u32>().expect("x must be of type u32");
            let y = sub_m.get_one::<String>("y").unwrap().parse::<u32>().expect("y must be of type u32");
            let w = sub_m.get_one::<String>("w").unwrap().parse::<u32>().expect("w must be of type u32");
            let h = sub_m.get_one::<String>("h").unwrap().parse::<u32>().expect("h must be of type u32");

            crop(infile, outfile, x, y, w, h);
        },  
        Some(("rotate", sub_m)) => {
            if let Ok(val) = sub_m.get_one::<String>("value").unwrap().parse::<u32>() {
                let r = if val < 90 {
                            Rot::R0
                        } else if val < 180 {
                            Rot::R90
                        } else if val < 270 {
                            Rot::R180
                        } else {
                            Rot::R270
                        };
                rotate(infile, outfile, r);
            } else {
                print_usage_and_exit();
            }
        },
        Some(("invert", _)) => {
            invert(infile, outfile);
        },    
        Some(("grayscale", _)) => {
            grayscale(infile, outfile);
        },     
        Some(("generate", sub_m)) => {
            let r = sub_m.get_one::<String>("r").unwrap().parse::<u8>().expect("r must be of type u8");
            let g = sub_m.get_one::<String>("g").unwrap().parse::<u8>().expect("g must be of type u8");
            let b = sub_m.get_one::<String>("b").unwrap().parse::<u8>().expect("b must be of type u8");

            generate(outfile, r, g, b);
        },       
        Some(("fractal", _)) => {
            fractal(outfile);
        },     
        _ => {},
    };

}

fn print_usage_and_exit() { // NOT USED
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, v: f32) {
    // Blurs image
    let img = get_image(&infile);
    let img2 = img.blur(v);
    save_image(img2, &outfile);
    println!("BLUR: Applied blur of {} to {} and saved to {}.", v, infile, outfile);
}

fn brighten(infile: String, outfile: String, v: i32) {
    // Positive numbers brighten the image. Negative numbers darken it. 
    let img = get_image(&infile);
    let img2 = img.brighten(v);

    save_image(img2, &outfile);
    println!("BRIGHTEN: Applied brightness of {} to {} and saved to {}.", v, infile, outfile);
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = get_image(&infile);
    let img2 = img.crop(x,y,width,height);
    save_image(img2, &outfile);
    println!("CROP: Cropped {} at {},{} with size {}x{} and saved to {}.", infile, x, y, width, height, outfile);
}

fn rotate(infile: String, outfile: String, r: Rot) {
    let img = get_image(&infile);
    let img2 = match r {
        Rot::R0 => {img.clone()},
        Rot::R90 => {img.rotate90()},
        Rot::R180 => {img.rotate180()},
        Rot::R270 => {img.rotate270()},
    };
    save_image(img2, &outfile);    
    println!("ROTATE: Rotated {} by {} and saved to {}.", infile, r.to_string(), outfile);
}

fn invert(infile: String, outfile: String) {  
    let mut img = get_image(&infile);
    img.invert();
    save_image(img, &outfile);

    println!("INVERT: Inverted {} and saved to {}.", infile, outfile);
}

fn grayscale(infile: String, outfile: String) {
    let img = get_image(&infile);
    let img2 = img.grayscale();
    save_image(img2, &outfile);

    println!("GRAYSCALE: Converted {} to grayscale and saved to {}.", infile, outfile);
}

fn generate(outfile: String, r: u8, g: u8, b: u8) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over coordinates/pixels
    for(_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save(&outfile).unwrap();

    println!("GENERATE: Generated solid color ({},{},{}) of size {}x{} and saved to {}.", r, g, b, width, height, &outfile);

}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(&outfile).unwrap();
    println!("FRACTAL: Generated fractal image of size {}x{} and saved to {}.", width, height, &outfile);
}

fn get_image(file: &String) -> DynamicImage {
    image::open(file).expect(&format!("Failed to open image file {}.",file).to_string())
}

fn save_image(img: DynamicImage, file: &String) {
    img.save(file).expect(&format!("Failed writing image file {}.",file));
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
