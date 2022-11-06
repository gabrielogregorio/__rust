mod network;
mod client;


mod use_method;
use use_method::first_level::second_level::three_level;

fn main() {
    println!("Hello, world!");
    network::connect();

    three_level::show_name();
}


#[cfg(test)]
mod tests {
    use super::network; // best

    #[test]
    fn it_works() {
        network::connect();
    }
}