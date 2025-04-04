// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    /* let qty: Result<i32, ParseIntError> = item_quantity.parse::<i32>();

    if let Result::Err(e) = qty { // 这里的e解构把qty(作为Err)移走了
        return Err(e); // 写return Err(qty.unwrap_err());会提示qty在if let位置已经被partial move
    }
    Ok(qty.unwrap() * cost_per_item + processing_fee) */

    // 直接这样写
    let qty: i32 = item_quantity.parse::<i32>()?; // ?的作用是: 如果Result是Ok(value)，则解构并返回value；如果是Err(e)，直接返回Err。相当于便捷处理两种情况

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
