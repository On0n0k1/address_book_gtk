pub fn hi() -> String{
    String::from("Hi from model::email")
}

pub struct Email{
    email: String,
}

impl Default for Email{
    fn default() -> Self {
        Email{
            email: String::from(""),
        }
    }
}

impl Email{
    pub fn set_value(&mut self, name: &str, value: String) -> Result<String, String> {
        match &(name.to_lowercase())[..]{
            "email" => {
                self.set(value.clone()).unwrap();
                Ok(value.clone())
            }
            _ => {
                Err(format!("Invalid address type: {} ", name))
            }
        }
    }

    pub fn get_value(&self, name: &str) -> Result<String, String> {
        match &(name.to_lowercase())[..]{
            "email" => {
                Ok(self.email.clone())
            }
            _ => {
                Err(format!("Invalid address type: {} ", name))
            }
        }
    }
    pub fn set(&mut self, value: String) -> Result<String, String>{
        self.set_value("email", value)
    }

    pub fn get(& self) -> String{
        self.get_value("email").unwrap()
    }
}

