use std::ops::Add;

pub fn hi() -> String{
    String::from("Hi from model::address")
}

pub struct Address{
    address_1: String,
    address_2: String,
    city: String,
    state: String,
    country: String,
    zip: String,
}

impl Default for Address{
    fn default() -> Self{
        return Address{
            address_1: String::from(""),
            address_2: String::from(""),
            city: String::from(""),
            state: String::from(""),
            country: String::from(""),
            zip: String::from(""),
        }
    }
}

impl Address{

    pub fn set_value(&mut self, name: &str, value: String) -> Result<String, String> {
        match &(name.to_lowercase())[..]{
            "address_1" => {
                self.address_1 = value;
                Ok(self.address_1.clone())
            },
            "address_2" => {
                self.address_2 = value;
                Ok(self.address_2.clone())
            },
            "city" => {
                self.city = value;
                Ok(self.city.clone())
            },
            "state" => {
                self.state = value;
                Ok(self.state.clone())
            },
            "country" => {
                self.country = value;
                Ok(self.country.clone())
            },
            "zip" => {
                self.zip = value;
                Ok(self.zip.clone())
            },
            _ => {
                Err(format!("Invalid address type: {} ", name))
            }
        }
    }

    pub fn set_address_1(&mut self, value: String) -> Result<String, String>{
        self.set_value("address_1", value)
    }

    pub fn set_address_2(&mut self, value: String) -> Result<String, String>{
        self.set_value("address_2", value)
    }

    pub fn set_city(&mut self, value: String) -> Result<String, String>{
        self.set_value("city", value)
    }

    pub fn set_state(&mut self, value: String) -> Result<String, String>{
        self.set_value("state", value)
    }

    pub fn set_country(&mut self, value: String) -> Result<String, String>{
        self.set_value("country", value)
    }

    pub fn set_zip(&mut self, value: String) -> Result<String, String>{
        self.set_value("zip", value)
    }

    pub fn get_value(&self, name: &str) -> Result<String, String>{
        // matches lowercase name with each existing value
        match &(name.to_lowercase())[..]{
            "address_1" => {
                Ok(self.address_1.clone())
            },
            "address_2" => {
                Ok(self.address_2.clone())
            },
            "city" => {
                Ok(self.city.clone())
            },
            "state" => {
                Ok(self.state.clone())
            },
            "country" => {
                Ok(self.country.clone())
            },
            "zip" => {
                Ok(self.zip.clone())
            },
            _ => {
                Err(format!("Invalid address type: {} ", name))
            }
        }
    }


    /// return address_1 and address_2 together
    pub fn get_address(&self) -> String{
        let address_1 = self.get_address_1();
        let address_2 = self.get_address_2();
        
        let mut result = address_1.clone();
        result.push_str(&address_2[..]);
        result
    }

    pub fn get_address_1(&self) -> String{
        self.get_value("address_1").unwrap()
    }

    pub fn get_address_2(&self) -> String{
        self.get_value("address_2").unwrap()
    }

    pub fn get_city(&self) -> String{
        self.get_value("city").unwrap()
    }

    pub fn get_state(&self) -> String{
        self.get_value("state").unwrap()
    }

    pub fn get_country(&self) -> String{
        self.get_value("country").unwrap()
    }

    pub fn get_zip(&self) -> String{
        self.get_value("zip").unwrap()
    }
}

