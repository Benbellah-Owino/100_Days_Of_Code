#[derive(Debug)]
struct Grill {
    vegetarian: i8,
    non_vegetarian: i8,
}

impl Grill {
    pub fn new() -> Grill {
        Grill {
            vegetarian: 0,
            non_vegetarian: 0,
        }
    }
}
fn main() {
    let grill = Vec::from([
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----",
    ]);

    let grill = sort(grill);
    println!("{:?}", grill);
}

fn sort(grill: Vec<&str>) -> [i8; 2] {
    let mut grill_count: Grill = Grill::new();

    for line in grill {
        if line.contains("-x") {
            grill_count.non_vegetarian += 1;
        } else if !line.contains("-x") {
            grill_count.vegetarian += 1;
        }
    }

    [grill_count.vegetarian, grill_count.non_vegetarian]
}
