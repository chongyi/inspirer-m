#[macro_use]
extern crate async_trait;

mod content;
mod process_channel;
mod context;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
