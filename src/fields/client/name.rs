pub fn hi() -> String{
    String::from("Hi from model::name")
}

pub struct Name{
    first_name: String,
    middle_name: String,
    last_name: String,
}

impl Default for Name{
    fn default() -> Self {
        Name{
            first_name: String::from(""),
            middle_name: String::from(""),
            last_name: String::from(""),
        }
    }
}

impl Name{
    pub fn set_value(&mut self, name: &str, value: String) -> Result<String, String> {
        match &(name.to_lowercase())[..]{
            "first_name" => {
                self.first_name = value.clone();
                Ok(self.first_name.clone())
            },
            "middle_name" => {
                self.middle_name = value.clone();
                Ok(self.middle_name.clone())
            },
            "last_name" => {
                self.last_name = value.clone();
                Ok(self.last_name.clone())
            }
            _ => {
                Err(String::from("Invalid name type"))
            }
        }
    }

    pub fn set_first_name(&mut self, value: String) -> Result<String, String>{
        self.set_value("first_name", value)
    }

    pub fn set_middle_name(&mut self, value: String) -> Result<String, String>{
        self.set_value("middle_name", value)
    }

    pub fn set_last_name(&mut self, value: String) -> Result<String, String>{
        self.set_value("last_name", value)
    }

    pub fn get_value(&self, name: &str) -> Result<String, String>{
        match &(name.to_lowercase())[..]{
            "first_name" => {
                Ok(self.first_name.clone())
            },
            "middle_name" => {
                Ok(self.middle_name.clone())
            },
            "last_name" => {
                Ok(self.last_name.clone())
            }
            _ => {
                Err(String::from("Invalid name type"))
            }
        }
    }

    pub fn get_first_name(&self) -> String {
        self.get_value("first_name").unwrap()
    }

    pub fn get_middle_name(&self) -> String {
        self.get_value("middle_name").unwrap()
    }

    pub fn get_last_name(&self) -> String {
        self.get_value("last_name").unwrap()
    }

    pub fn get_name(&self) -> String {
        let mut name = self.get_last_name();
        name.push_str(", ");
        name.push_str(&self.get_first_name()[..]);
        name
    }
}
