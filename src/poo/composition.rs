
struct Category {
    id_category: i32,
    category: String 
}

struct Product {
    id_product: i32,
    fk_id_category: Category,
    brand: String,
    model: String,
    quantity: i32,
    unit_price: f32
}

pub fn show_composition() {
    let category = Category{
        id_category: 1,
        category: String::from("Electronics")
    };
    let product = Product{
        id_product: 1,
        fk_id_category: category,
        brand: String::from("Nokia"),
        model: String::from("G21"),
        quantity: 20,
        unit_price: 189.99
    };

    println!(
        "id: {}\nCategory: {}\nBrand: {}\nModel: {}\nQuantity: {}\nUnit Price: {}",
        product.id_product,
        product.fk_id_category.category,
        product.brand,
        product.model,
        product.quantity,
        product.unit_price
    );
}
