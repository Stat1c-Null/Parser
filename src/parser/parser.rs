pub struct Parser {
  input : String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser { input }
    }

    pub fn parse(&self, code: &str) -> Result<(), String> {
        // Placeholder for parsing logic
        println!("Parsing code:\n{}", code);
        Ok(())
    }
}