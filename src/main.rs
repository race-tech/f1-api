mod standing;

fn main() {
    let standing = standing::Standing::get_current();

    println!("{:?}", standing);
}
