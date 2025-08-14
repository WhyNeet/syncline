use crdt::{Rga, RgaInsertQuery};

#[test]
pub fn single_actor_rga_left_insertion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    for (idx, c) in "Hello, world!".chars().enumerate() {
        rga.insert(RgaInsertQuery::Left((actor_id, idx as u64)), c);
    }

    assert_eq!(rga.to_string(), "Hello, world!");
}

#[test]
pub fn single_actor_rga_right_insertion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    let mut chars = "Hello, world!".chars().enumerate();

    if let Some((idx, c)) = chars.next() {
        rga.insert(RgaInsertQuery::Left((actor_id, idx as u64)), c);
    }

    for (idx, c) in chars {
        rga.insert(RgaInsertQuery::Right((actor_id, idx as u64)), c);
    }

    assert_eq!(
        rga.to_string().chars().rev().collect::<String>(),
        "Hello, world!"
    );
}

#[test]
pub fn single_actor_rga_middle_insertion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    let input = "Hello, world!";
    let mut chars = input.chars().enumerate();

    let first = chars.next().unwrap();
    let start_id = rga.insert(RgaInsertQuery::Left((actor_id, 0)), first.1);

    let middle = chars.by_ref().take(input.len() - 2).collect::<Vec<_>>();

    let end = chars.next().unwrap();

    let end_id = rga.insert(RgaInsertQuery::Left(start_id), end.1);

    let mut start_id = start_id;

    for (_, c) in middle {
        start_id = rga.insert(RgaInsertQuery::Middle(start_id, end_id), c);
    }

    assert_eq!(rga.to_string(), "Hello, world!");
}
