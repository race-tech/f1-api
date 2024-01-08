mod livetiming;
mod standing;

fn main() {
    let standing = standing::Standing::get_current();

    println!("{:?}", standing);

    println!("Try signalr connection ...");
    let conn = livetiming::SignalrConn::negotiate();
    println!("{:?}", conn);
}
