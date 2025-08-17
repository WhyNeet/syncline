use crdt::{Rga, RgaInsertQuery};

#[test]
pub fn single_actor_rga_right_insertion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    for (idx, c) in "Hello, world!".chars().enumerate() {
        assert!(
            rga.insert(RgaInsertQuery::Right((actor_id, idx as u64)), c, None, None)
                .is_some()
        );
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
    let start_id = {
        let id = rga.insert(RgaInsertQuery::Right((actor_id, 0)), first, None, None);
        assert!(id.is_some());
        id.unwrap()
    };

    let middle = chars.by_ref().take(input.len() - 2).collect::<Vec<_>>();

    let end = chars.next().unwrap();

    let end_id = {
        let id = rga.insert(RgaInsertQuery::Right(start_id), end, None, None);
        assert!(id.is_some());
        id.unwrap()
    };

    let mut start_id = start_id;

    for c in middle {
        start_id = {
            let id = rga.insert(RgaInsertQuery::Middle(start_id, end_id), c, None, None);
            assert!(id.is_some());
            id.unwrap()
        };
    }

    assert_eq!(rga.to_string(), "Hello, world!");
}

#[test]
pub fn multi_actor_rga_insertion_works() {
    let current_actor_id = 0;
    let other_actor_id = 1;

    let mut rga = Rga::new(0);

    // Insert "Hello" as current actor
    let mut prev_id = (current_actor_id, 0);
    for c in "Hello".chars() {
        prev_id = {
            let id = rga.insert(RgaInsertQuery::Right(prev_id), c, None, None);
            assert!(id.is_some());
            id.unwrap()
        };
    }

    // Current actor wants to put 'a' after letter 'H'
    // Other actor wants to put 'b' after letter 'H'

    // Case 1:
    // Edit from current actor comes first, edit from last actor comes second
    {
        let mut rga = rga.clone();
        assert!(
            rga.insert(
                RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
                'a',
                None,
                None
            )
            .is_some()
        );
        assert!(
            rga.insert(
                RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
                'b',
                Some(other_actor_id),
                None
            )
            .is_some()
        );
        assert_eq!(rga.to_string(), "Habello");
    }

    // Case 2:
    // Edit from current actor comes second, edit from last actor comes first
    {
        let mut rga = rga.clone();
        assert!(
            rga.insert(
                RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
                'b',
                Some(other_actor_id),
                None
            )
            .is_some()
        );
        assert!(
            rga.insert(
                RgaInsertQuery::Middle((current_actor_id, 1), (current_actor_id, 2)),
                'a',
                None,
                None
            )
            .is_some()
        );
        assert_eq!(rga.to_string(), "Habello");
    }
}

#[test]
pub fn multi_actor_rga_end_insertion_works() {
    let current_actor_id = 0;
    let other_actor_id = 1;

    let mut rga = Rga::new(0);

    // Insert "Hello" as current actor
    let mut prev_id = (current_actor_id, 0);
    for c in "Hello".chars() {
        prev_id = {
            let id = rga.insert(RgaInsertQuery::Right(prev_id), c, None, None);
            assert!(id.is_some());
            id.unwrap()
        };
    }

    // Case 1:
    // Edit on the end from current actor comes first, edit from last actor comes second
    {
        let mut rga = rga.clone();

        assert!(
            rga.insert(RgaInsertQuery::Right(prev_id), 't', None, None)
                .is_some()
        );
        rga.insert(
            RgaInsertQuery::Right(prev_id),
            'h',
            Some(other_actor_id),
            None,
        );
        assert_eq!(rga.to_string(), "Helloth");
    }

    // Case 2:
    // Edit on the end from current actor comes second, edit from last actor comes first
    {
        let mut rga = rga.clone();

        assert!(
            rga.insert(
                RgaInsertQuery::Right(prev_id),
                'h',
                Some(other_actor_id),
                None,
            )
            .is_some()
        );

        rga.insert(RgaInsertQuery::Right(prev_id), 't', None, None);
        assert_eq!(rga.to_string(), "Helloth");
    }
}

#[test]
pub fn single_actor_rga_deletion_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    for (idx, c) in "Hello, world!".chars().enumerate() {
        rga.insert(RgaInsertQuery::Right((actor_id, idx as u64)), c, None, None);
    }

    rga.delete((actor_id, 6));

    assert_eq!(rga.to_string(), "Hello world!");
}

#[test]
pub fn rga_compaction_works() {
    let actor_id = 0;
    let mut rga = Rga::new(actor_id);

    for (idx, c) in "Hello, world!".chars().enumerate() {
        rga.insert(RgaInsertQuery::Right((actor_id, idx as u64)), c, None, None);
    }

    rga.delete((actor_id, 6));
    rga.delete((actor_id, 7));
    assert!(
        rga.insert(
            RgaInsertQuery::Middle((actor_id, 7), (actor_id, 8)),
            ' ',
            None,
            None
        )
        .is_some()
    );

    rga.compact();

    assert!(
        rga.insert(
            RgaInsertQuery::Middle((actor_id, 7), (actor_id, 8)),
            ' ',
            None,
            None
        )
        .is_none()
    );

    assert_eq!(rga.to_string(), "Hello world!");
}
