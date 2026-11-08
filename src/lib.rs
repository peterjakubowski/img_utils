pub fn list_image_paths() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_image_paths() {
        let result = list_image_paths();
        assert_eq!(result, ());
    }
}
