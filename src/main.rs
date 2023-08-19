use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;

fn main() {
    println!("{:?}", env::args().nth(1));
    let font = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
    print!("size: {:?}\n", font.bounds());
    let width = font.bounds().width;
    let height = font.bounds().height;

    let glyph = font.glyphs();

    let mut bmps: HashMap<&char, Vec<u8>> = HashMap::new();
    let x_bytes = (width + 7) / 8;
    // 按顺序收集所有的点
    for (x, y) in glyph {
        let mut points = Vec::new();
        y.pixels().for_each(|((x, y), vis)| {
            points.push((x, y, vis));
        });

        points.sort_by(|a: &(u32, u32, bool), b| {
            if a.1 < b.1 {
                return Ordering::Less;
            } else if a.1 > b.1 {
                return Ordering::Greater;
            } else {
                if a.0 < b.0 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
        });
        // println!("points: {:?}", points.len());
        let mut bmp = vec![0u8; (x_bytes * height) as usize];
        for p in points {
            let x = p.0;
            let y = p.1;
            let vis = p.2;
            if vis {
                let index = (y * x_bytes + x / 8) as usize;
                let bit = x % 8;
                bmp[index] |= 1 << (7 - bit);
            }
        }
        bmps.insert(x, bmp);
    }

    let mut keys = bmps.keys().map(|x| **x as u32).collect::<Vec<_>>();
    keys.sort();
    println!("keys: {:?}", keys);

    let txt = bmps.get(&'a').unwrap();
    for i in 0..height {
        for j in 0..x_bytes {
            for k in 0..8 {
                if txt[(i * x_bytes + j) as usize] & (1 << (7 - k)) != 0 {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }

    generate_result(bmps);
}

fn generate_result(bmps: HashMap<&char, Vec<u8>>) {
    // 根据需要，将字符分组，以节省bitmap的空间

    // let splits_termius = [
    //     (0, 1274),
    //     (7732, 10009),
    //     (57504, 57524),
    //     (63166, 63167),
    //     (65533, 65536),
    // ];

    let splits_dina = [(0, 256)];

    let splits = &splits_dina;

    let mut result_bytes = Vec::new();
    for sp in splits {
        for i in sp.0..sp.1 {
            if let Some(bts) = bmps.get(&char::from_u32(i).unwrap()) {
                result_bytes.extend_from_slice(bts);
            } else {
                let bts_q = bmps.get(&'?').unwrap();
                result_bytes.extend_from_slice(bts_q);
            }
        }
    }
    println!("result: {:?}", result_bytes.len());

    // 写入文件

    let resulr_path = env::args().nth(2).expect("missing result file");
    std::fs::write(resulr_path, result_bytes).expect("write result file failed");
}
