mod testing_import{
    use address_book_gtk::model::client;
    use address_book_gtk::gtk_custom;

    #[test]
    fn import_listbox(){
        assert_eq!(gtk_custom::listbox::hi(), String::from("Hi from listbox"));
    }

    #[test]
    fn import_liststore(){
        assert_eq!(gtk_custom::liststore::hi(), String::from("Hi from liststore"));
    }

    #[test]
    fn import_address(){
        assert_eq!(client::address::hi(), String::from("Hi from model::address"));
    }

    #[test]
    fn import_email(){
        assert_eq!(client::email::hi(), String::from("Hi from model::email"));
    }

    #[test]
    fn import_name(){
        assert_eq!(client::name::hi(), String::from("Hi from model::name"));
    }

    #[test]
    fn import_phone(){
        assert_eq!(client::phone::hi(), String::from("Hi from model::phone"));
    }

    #[test]
    fn containment(){
        // Just checking if this function works to use it in set/get methods
        assert!(
            ["name", "another_name", "other"].contains(&"name")
        );
    }
}