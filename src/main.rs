use std::fs::File;
use image::GenericImageView;
use bitbit::BitWriter;

fn main() {
    let input_file:&String = &std::env::args().collect::<Vec<String>>()[1];
    let input_image = image::open(input_file).unwrap();
    let mut bitwriter = BitWriter::new(File::create(input_file.replace("png", "ximg")).unwrap());
    let mut pixel_data:(Vec<String>, Vec<u8>) = (vec![], vec![]);

    for pixel in input_image.pixels() {
        let mut result:String = if pixel.2[3] > 127 { "".to_string() } else { "000001111111111".to_string() };
        if result != "000001111111111".to_string() {
            for i in 0..3 as usize {
                result.push_str(format!("{:0>5}", pixel.2[i] >> 3).as_str());
            }
        }
        if pixel_data.0.len() > 0 && result == pixel_data.0[pixel_data.0.len() - 1] && pixel_data.1[pixel_data.1.len() - 1] != u8::MAX {
            let last_color_entry = &pixel_data.1.len() - 1;
            pixel_data.1[last_color_entry] += 1;
        } else {
            pixel_data.0.push((&result).to_string());
            pixel_data.1.push(0);
        }
    }

    write_data(&mut bitwriter, vec![&format!("{:0>16}", &input_image.dimensions().0), &format!("{:0>16}", &input_image.dimensions().1)]);

    for i in 0..pixel_data.0.len() {
        let mut color_reference = (&pixel_data.0[i]).to_string();
        if pixel_data.1[i] == 0 {
            color_reference.push('0');
            write_data(&mut bitwriter, vec![&color_reference]);
        } else {
            color_reference.push('1');
            write_data(&mut bitwriter, vec![&color_reference, &format!("{:0>8}", &pixel_data.1[i])]);
        }
    }
}

fn write_data(bitwriter:&mut BitWriter<File>, data:Vec<&String>) {
    for entry in data {
        for bit in entry.chars() { bitwriter.write_bit(bit == '1').unwrap(); }
    }
}