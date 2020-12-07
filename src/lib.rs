pub struct AllPos<'a> {
    input: &'a str,
    position: usize,
    search: char,
}

impl Iterator for AllPos<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.input[self.position..]
            .chars()
            .position(|c| c == self.search)
        {
            Some(new_pos) => {
                self.position += new_pos + 1;
                return Some(self.position - 1);
            }
            None => return None,
        }
    }
}

pub fn find_all_positions(input: &str, search: char) -> AllPos {
    AllPos {
        input,
        position: 0,
        search,
    }
}

#[test]
fn test_find_all_pos() {
    let input = "abc def g h  ";
    let mut space_positions = find_all_positions(&input, ' ');
    assert_eq!(space_positions.next(), Some(3));
    assert_eq!(space_positions.next(), Some(7));
    assert_eq!(space_positions.next(), Some(9));
    assert_eq!(space_positions.next(), Some(11));
    assert_eq!(space_positions.next(), Some(12));
    assert_eq!(space_positions.next(), None);
}
