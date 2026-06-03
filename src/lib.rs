pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let v = vector::Vector::<i32>::new();

        assert_eq!(v.capacity(), 0);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(v.at(100), Err("Vector index out of bounds".to_string()));
    }
    
    #[test]
    fn test_filled_vector() {
        let mut v = vector::Vector::<i32>::new();
        
        v.push(1);
        v.push(2);
        v.push(3);
        
        assert_eq!(v.capacity(), 16usize);
        assert_eq!(v.size(), 3usize);
        assert!(!v.is_empty());
        assert_eq!(v.at(0), Ok(1));
        assert_eq!(v.at(1), Ok(2));
        assert_eq!(v.at(2), Ok(3));
        assert_eq!(v.at(3), Err("Vector index out of bounds".to_string()));
    }
    
    #[test]
    fn test_vector_grow() {
        let mut v = vector::Vector::<i32>::new();
        
        for i in 0..16 {
            v.push(i);
        }
        
        assert_eq!(v.capacity(), 16usize);
        assert_eq!(v.size(), 16usize);
        
        v.push(100);
        v.push(101);
        
        assert_eq!(v.capacity(), 32usize);
        assert_eq!(v.size(), 18usize);
    }
}