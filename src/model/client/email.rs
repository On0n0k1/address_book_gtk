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
    pub fn set(&mut self, value: String) -> String{
        self.email = value.clone();
        value
    }

    pub fn get(& self) -> String{
        self.email.clone()
    }
}

