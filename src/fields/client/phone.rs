pub fn hi() -> String{
    String::from("Hi from model::phone")
}

pub struct Phone{
    phone: String,
}


impl Default for Phone{
    fn default() -> Self {
        Phone{
            phone: String::from(""),
        }
    }
}

impl Phone{
    pub fn set_value(&mut self, name: &str, value: String) -> Result<String, String> {
        match &(name.to_lowercase()[..]){
            "phone" => {
                self.phone = value.clone();
                Ok(value.clone())
            },
            _ => {
                Err(format!("Invalid address type: {} ", name))
            }
        }
    }

    pub fn set(&mut self, value: String) -> Result<String, String>{
        self.set_value("phone", value)
    }

    pub fn get_value(&self, name: &str) -> Result<String, String> {
        match &(name.to_lowercase())[..]{
            "phone" => {
                Ok(self.phone.clone())
            }
            _ => {
                Err(format!("Invalid address type: {} ", name))
            }
        }
    }
    
    pub fn get(&self) -> String{
        self.get_value("phone").unwrap()
    }
}
