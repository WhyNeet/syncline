use crdt::{Rga, RgaInsertQuery};

#[test]
pub fn single_actor_rga_right_insertion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    for (idx, c) in "Hello, world!".chars().enumerate() {
        rga.insert(RgaInsertQuery::Right((actor_id, idx as u64)), c, None);
    }

    assert_eq!(rga.to_string(), "Hello, world!");
}

#[test]
pub fn single_actor_rga_middle_insertion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    let input = "Hello, world!";
    let mut chars = input.chars();

    let first = chars.next().unwrap();
    let start_id = rga.insert(RgaInsertQuery::Right((actor_id, 0)), first, None);

    let middle = chars.by_ref().take(input.len() - 2).collect::<Vec<_>>();

    let end = chars.next().unwrap();

    let end_id = rga.insert(RgaInsertQuery::Right(start_id), end, None);

    let mut start_id = start_id;

    for c in middle {
        start_id = rga.insert(RgaInsertQuery::Middle(start_id, end_id), c, None);
    }

    assert_eq!(rga.to_string(), "Hello, world!");
}

#[test]
pub fn multi_actor_rga_left_insertion_works() {
    let current_actor_id = 0;
    let other_actor_id = 1;

    let mut rga = Rga::new(0);

    // Insert "Hello" as current actor
    let mut prev_id = (current_actor_id, 0);
    for c in "Hello".chars() {
        prev_id = rga.insert(RgaInsertQuery::Right(prev_id), c, None);
    }

    // Current actor wants to put 'a' after letter 'H'
    // Other actor wants to put 'b' after letter 'H'

    // Case 1:
    // Edit from current actor comes first, edit from last actor comes second
    {
        let mut rga = rga.clone();
        rga.insert(
            RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
            'a',
            None,
        );
        rga.insert(
            RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
            'b',
            Some(other_actor_id),
        );
        assert_eq!(rga.to_string(), "Habello");
    }

    // Case 2:
    // Edit from current actor comes second, edit from last actor comes first
    {
        let mut rga = rga.clone();
        rga.insert(
            RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
            'b',
            Some(other_actor_id),
        );
        rga.insert(
            RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
            'a',
            None,
        );
        assert_eq!(rga.to_string(), "Habello");
    }
}
