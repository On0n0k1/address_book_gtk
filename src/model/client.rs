pub mod address;
pub mod email;
pub mod name;
pub mod phone;


pub struct Client{
    address: address::Address,
    email: email::Email,
    name: name::Name,
    phone: phone::Phone,
}

impl Default for Client{
    fn default() -> Self {
        Client{
            address: address::Address::default(),
            email: email::Email::default(),
            name: name::Name::default(),
            phone: phone::Phone::default(),
        }
    }
}

impl Client{
    
}
