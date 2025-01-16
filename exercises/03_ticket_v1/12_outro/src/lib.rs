pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        val_product_name(&product_name);
        val_quantity(&quantity);
        val_unit_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn set_product_name(&mut self, product_name: String) {
        val_product_name(&product_name);
        self.product_name = product_name;
    }
    pub fn set_quantity(&mut self, quantity: u32) {
        val_quantity(&quantity);
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self, unit_price: u32) {
        val_unit_price(&unit_price);
        self.unit_price = unit_price;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}

fn val_product_name(product_name: &str) {
    if product_name.is_empty() {
        panic!("Product name cannot be empty");
    }
    if product_name.len() > 300 {
        panic!("Product name cannot be longer than 300 bytes");
    }
}

fn val_quantity(quantity: &u32) {
    if *quantity == 0 {
        panic!("Quantity must be greater than zero");
    }
}

fn val_unit_price(unit_price: &u32) {
    if *unit_price == 0 {
        panic!("Unit price must be greater than zero");
    }
}
