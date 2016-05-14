#![feature(slice_patterns, advanced_slice_patterns)]
type Product = u32;

const PRODUCT_A: u32 = 50;
const PRODUCT_B: u32 = 30;
const PRODUCT_C: u32 = 20;

fn price(product: u32) -> u32 { product }

fn empty_basket() -> Vec<Product> { vec!() }

fn sum_price(basket: &[Product]) -> u32 {
    let count_a = basket.iter().filter(|&x| *x == PRODUCT_A).count() as u32;
    let count_b = basket.iter().filter(|&x| *x == PRODUCT_B).count() as u32;
    let count_c = basket.iter().filter(|&x| *x == PRODUCT_C).count() as u32;

    return
        price(PRODUCT_A) * (count_a % 3) + (count_a / 3) * 130 +
        price(PRODUCT_B) * (count_b % 2) + (count_b / 2) * 45 +
        price(PRODUCT_C) * count_c;
}

fn total_price(basket: Vec<Product>) -> u32 {
    sum_price(basket.as_slice())
}

#[test]
fn total_price_of_an_empty_basket_is_0() {
    assert_eq!(total_price(empty_basket()), 0);
}

#[test]
fn total_price_of_a_basket_with_a_product_is_the_price_of_the_product() {
    let basket = vec!(PRODUCT_A);

    assert_eq!(total_price(basket), PRODUCT_A);
}

#[test]
fn total_price_of_a_basket_with_two_products_is_their_sum() {
    let basket = vec!(PRODUCT_B, PRODUCT_C);

    assert_eq!(total_price(basket), 50);
}

#[test]
fn total_price_of_a_basket_with_offer_products_a_is_discounted() {
    let basket = vec!(PRODUCT_A, PRODUCT_B, PRODUCT_A, PRODUCT_A);

    assert_eq!(total_price(basket), 160);
}

#[test]
fn total_price_of_a_basket_with_offer_products_b_is_discounted() {
    let basket = vec!(PRODUCT_A, PRODUCT_B, PRODUCT_A, PRODUCT_B);

    assert_eq!(total_price(basket), 145);
}
