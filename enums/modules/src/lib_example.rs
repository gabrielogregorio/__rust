// network::connect
mod network { // this is namespace
    fn connect() { // function inside namespace

    }


    // network::wifi::connect()
    mod wifi { // inside module network
        fn connect() {

        }
    }
}

mod client { // other module
    fn connect() {

    }
}