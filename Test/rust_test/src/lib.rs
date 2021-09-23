#[cfg(test)]
mod tests{
    #[test]
    fn exploation() { 
        assert_eq!(2+2,4);
    }

    #[test]
    fn another()
    {
        panic!("make this fail");
    }
}
