use regex::Regex;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Order {
    pub qty: u32,
    pub product: String,
    pub source: String,
}

impl Order {
    pub fn new<S: Into<String>>(qty: u32, product: S, source: S) -> Self {
        Self {
            qty,
            product: product.into(),
            source: source.into(),
        }
    }

    pub fn parse<S: Into<String>>(command: S) -> Result<Self, Box<dyn Error>> {
        // Grammar Representationconst
        let optional_space = " ?";
        let qty = String::from("x(?P<qty>\\d+)") + optional_space;
        let product = String::from("'(?P<product>[\\w ]+)'") + optional_space;
        let source = "from (?P<source>\\w+)";
        let order_command =
            String::from("order") + optional_space + qty.as_str() + product.as_str() + source;

        let re = Regex::new(&order_command)?;
        let command = command.into();
        let x = re.captures(command.as_ref()).ok_or("Bad command")?;

        Ok(Self {
            product: x
                .name("product")
                .ok_or("Product not defined")?
                .as_str()
                .to_string(),
            qty: x.name("qty").ok_or("Qty not defined")?.as_str().parse()?,
            source: x
                .name("source")
                .ok_or("Source not defined")?
                .as_str()
                .to_string(),
        })
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::new(1, "Ice cream", "Five")
    }
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn test_1() {
        let order = Order::new(12, "1L milk packs", "Macro");
        let command = "order x12 '1L milk packs' from Macro";
        assert_eq!(order, Order::parse(command).unwrap());
    }

    #[test]
    fn test_2() {
        let order = Order::new(1, "a bag of potatoes", "Tesco");
        let command = "order x1 'a bag of potatoes' from Tesco";
        assert_eq!(order, Order::parse(command).unwrap());
    }
}
