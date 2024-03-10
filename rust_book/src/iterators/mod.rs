struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    //Below won't work because iter() produces immutable references
    // shoes.iter().filter(|shoe| shoe.size == shoe_size).collect()
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}
