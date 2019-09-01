extern crate echo;


#[cfg(test)]
mod tests {
    use echo::echo;

    #[test]
    fn print_input(){
        let input = echo("TestInput");
       
        assert_eq!(input,"TestInput");
    }

}