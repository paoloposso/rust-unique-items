fn unique<T>(list: &mut Vec<T>) -> &mut Vec<T> where T: PartialEq, T: Ord {
    list.sort();
    list.dedup();
    list
}

#[cfg(test)]
mod tests {
    use crate::unique;

    #[test]
    fn it_works_for_ints() {
        let list = &mut vec![1, 3, 4, 3,  5, 2];
        let result = unique(list);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn it_works_for_strings() {
        let list = &mut vec!["a", "aa", "bb", "a"];
        let result = unique(list);
        assert_eq!(result.len(), 3);
    }
}
