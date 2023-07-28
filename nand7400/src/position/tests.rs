#![cfg(test)]

use super::*;

#[test]
fn test_position() {
    let position = Position::new(0, 1);
    assert_eq!(position.starting_char(), 0);
    assert_eq!(position.ending_char(), 0);
    assert_eq!(position.len(), 1);

    let position = Position::new(0, 2);
    assert_eq!(position.starting_char(), 0);
    assert_eq!(position.ending_char(), 1);
    assert_eq!(position.len(), 2);
}

/// Tests the joining of `Positions` that are adjacent/next to each other.
#[test]
fn test_adjacent_joins() {
    let a = Position::new(0, 1);
    let b = Position::new(1, 2);

    let joined = a.join(&b);

    assert_eq!(joined.starting_char(), 0);
    assert_eq!(joined.ending_char(), 1);
    assert_eq!(joined.len(), 2);
}

/// Tests the joining of `Positions` that are not adjacent/next to each other.
#[test]
fn test_non_adjacent_joins() {
    let a = Position::new(0, 1);
    let b = Position::new(5, 6);

    let joined = a.join(&b);

    assert_eq!(joined.starting_char(), 0);
    assert_eq!(joined.ending_char(), 5);
    assert_eq!(joined.len(), 6);
}
