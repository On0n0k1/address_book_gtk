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
    pub fn set(&mut self, value: String) -> Result<String, String>{
        self.phone = value.clone();
        Ok(value.clone())
    }

    pub fn get(&self) -> String{
        self.phone.clone()
    }
}
