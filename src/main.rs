#![feature(slice_patterns, advanced_slice_patterns)]
type Product = u32;

const PRODUCT_A: u32 = 50;
const PRODUCT_B: u32 = 30;
const PRODUCT_C: u32 = 20;

fn price(product: u32) -> u32 { product }

fn empty_basket() -> Vec<Product> { vec!() }

fn sorted(basket: Vec<Product>) -> Vec<Product> {
    let mut tmp = basket.clone();
    tmp.sort();
    return tmp;
}

fn sum_price(basket: &[Product]) -> u32 {
    match basket {
        [] => 0,
        [PRODUCT_A, PRODUCT_A, PRODUCT_A, rest..] => 130 + sum_price(rest),
        [PRODUCT_B, PRODUCT_B, rest..] => 45 + sum_price(rest),
        [x, rest..] => price(x) + sum_price(rest)
    }
}

fn total_price(basket: Vec<Product>) -> u32 {
    sum_price(sorted(basket).as_slice())
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
