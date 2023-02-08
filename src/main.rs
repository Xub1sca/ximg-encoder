use std::fs::File;
use image::GenericImageView;
use bitbit::BitWriter;

fn main() {
    let input_file:&String = &std::env::args().collect::<Vec<String>>()[1];
    let input_image = image::open(input_file).unwrap();
    let mut bitwriter = BitWriter::new(File::create(input_file.replace("png", "ximg")).unwrap());
    let mut color_data:Vec<String> = vec![];
    let mut color_repeat:Vec<u8> = vec![];

    for pixel in input_image.pixels() {
        let mut result:String = if pixel.2[3] > 127 { "".to_string() } else { "000001111111111".to_string() };
        if result != "000001111111111".to_string() {
            for i in 0..3 as usize {
                result.push_str(format_numbers(&pixel.2[i], 5).as_str());
            }
        }
        if color_data.len() > 0 && result == color_data[color_data.len() - 1] && color_repeat[color_repeat.len() - 1] != u8::MAX {
            let last_color_entry = &color_repeat.len() - 1;
            color_repeat[last_color_entry] += 1;
        } else {
            color_data.push((&result).to_string());
            color_repeat.push(0);
        }
    }

    write_data(&mut bitwriter, vec![&format_numbers(&input_image.dimensions().0, 16), &format_numbers(&input_image.dimensions().1, 16)]);

    for i in 0..color_data.len() {
        let mut color_reference = (&color_data[i]).to_string();
        if color_repeat[i] == 0 {
            color_reference.push('0');
            write_data(&mut bitwriter, vec![&color_reference]);
        } else {
            color_reference.push('1');
            write_data(&mut bitwriter, vec![&color_reference, &format_numbers(&color_repeat[i], 8)]);
        }
    }
}

fn write_data(bitwriter:&mut BitWriter<File>, data:Vec<&String>) {
    for entry in data {
        for bit in entry.chars() { bitwriter.write_bit(bit == '1').unwrap(); }
    }
}

fn format_numbers<T: std::fmt::Binary>(data:&T, bits:usize) -> String{
    let mut stream = format!("{data:b}");
    stream = stream.chars().rev().collect();
    while stream.len() < bits { stream.push('0'); }
    while stream.len() > bits { stream.pop(); }
    stream.chars().rev().collect()
}