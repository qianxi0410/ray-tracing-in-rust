mod vec3;

fn main() {
    let mut v = vec3::Vec3::new(1, 2, 3);
    v += 2;

    println!("{}", v.x);
}
