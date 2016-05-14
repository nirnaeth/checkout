#![feature(slice_patterns)]
type Product = u32;

const PRODUCT_A: u32 = 50;
const PRODUCT_B: u32 = 30;
const PRODUCT_C: u32 = 20;

fn price(product: u32) -> u32 { product }

fn empty_basket() -> Vec<Product> { vec!() }

fn total_price(basket: &[Product]) -> u32 {
    match basket {
        [] => 0,
        //1 => price(basket[0]),
        [x, rest..] => price(x) + total_price(rest)
    }
}


#[test]
fn total_price_of_an_empty_basket_is_0() {
    assert_eq!(total_price(empty_basket().as_slice()), 0);
}

#[test]
fn total_price_of_a_basket_with_a_product_is_the_price_of_the_product() {
    let basket = vec!(PRODUCT_A);

    assert_eq!(total_price(basket.as_slice()), PRODUCT_A);
}

#[test]
fn total_price_of_a_basket_with_two_products_is_their_sum() {
    let basket = vec!(PRODUCT_B, PRODUCT_C);

    assert_eq!(total_price(basket.as_slice()), 50);
}
