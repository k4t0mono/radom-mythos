// testing

#[cfg(test)]
mod tests {
    
    #[test]
    fn dummy_test() {
        assert_eq!(2+2, 4);
    }

    #[test]
    #[should_panic]
    fn can_not_create_relations_of_invalid_size() {
        use Relations;

        let r = Relations::init(0);
    }

}
