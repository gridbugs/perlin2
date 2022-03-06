use perlin2::Perlin2;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

fn main() {
    let mut rng = XorShiftRng::from_entropy();
    let perlin = Perlin2::new(&mut rng);
    for y in 0..50 {
        for x in 0..100 {
            let noise = perlin.noise01((x as f64 / 20., y as f64 / 10.));
            if noise > 0.5 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
