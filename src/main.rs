use image::GenericImageView;
use std::{fs::File, io::Write};

fn main() {
    let path: &String = &std::env::args().nth(1).unwrap();
    let image = image::open(path).unwrap();
    let mut data: (Vec<String>, Vec<u8>, Vec<String>) = (vec![], vec![], vec![]);

    for pixel in image.pixels() {
        let mut result: String = if pixel.2[3] > 127 {
            "".to_string()
        } else {
            "000001111111111".to_string()
        };
        if result != *"000001111111111".to_string() {
            for i in 0..3_usize {
                result.push_str(format!("{:0>5}", pixel.2[i] >> 3).as_str());
            }
        }
        if !data.0.is_empty()
            && result == data.0[data.0.len() - 1]
            && data.1[data.1.len() - 1] != u8::MAX
        {
            let last_color_entry = &data.1.len() - 1;
            data.1[last_color_entry] += 1;
        } else {
            data.0.push((result).to_string());
            data.1.push(0);
        }
    }

    data.2.push(format!("{:0>16}", &image.dimensions().0));
    data.2.push(format!("{:0>16}", &image.dimensions().1));

    for i in 0..data.0.len() {
        let mut color_reference = (data.0[i]).to_string();
        if data.1[i] == 0 {
            color_reference.push('0');
            data.2.push(color_reference);
        } else {
            color_reference.push('1');
            data.2.push(color_reference);
            data.2.push(format!("{:0>8}", data.1[i]));
        }
    }

    let mut file = File::create(path.replace("png", "ximg")).unwrap();
    let mut byte: [u8; 1] = [0];
    let mut shift = 0;

    for entry in data.2 {
        for bit in entry.chars() {
            byte[0] <<= 1;
            if bit == '1' {
                byte[0] |= 0x1;
            }
            if {
                shift += 1;
                shift
            } == 8
            {
                file.write_all(&byte).unwrap();
                byte[0] = 0;
                shift = 0;
            }
        }
    }
}
