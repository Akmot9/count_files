
#[cfg(test)] // Indique que ce module est uniquement compilé en mode test.
mod tests {

    use crate::get_os;

    #[test]
    fn test_get_os() {
        let result = get_os();
        assert!(result.is_ok(), "get_os should return Ok but returned {:?}", result);
    }
}
