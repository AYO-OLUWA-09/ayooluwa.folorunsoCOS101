
struct Laptop {
    brand:String,
    price:u32,
}

impl Laptop {

    // using method to calculate cost for buying a quntity
    fn cost(&self, quantity:u32)-> u32 {
        self.price * quantity
    }
}
fn main() {
    // create laptop objects

    let hp = Laptop {
        brand:String::from("HP"),
        price:650_000
    };
    let imb = Laptop {
        brand:String::from("IMB"),
        price:750_000
    };
    let toshiba = Laptop {
        brand:String::from("Toshiba"),
        price:550_000
    };
    let dell = Laptop {
        brand:String::from("Dell"),
        price:850_000
    };
    let quantity = 3;

    let total_cost = hp.cost(quantity) + imb.cost(quantity) + toshiba.cost(quantity) + dell.cost(quantity);
    println!("Total cost for 3 units in each brand is:#{}", total_cost);


}