use rand::Rng;
fn main() {
    for _ in 0..10 {
        let random_num = rand::thread_rng().gen_range(0..2);
        if random_num == 0 {
            println!("You got heads");
        } else if random_num == 1 {
            println!("You got tails");
        }
    }
}
