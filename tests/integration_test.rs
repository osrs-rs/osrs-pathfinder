use osrs_pathfinder::Pathfinder;

#[test]
fn test_one() {
    let pathfinder = Pathfinder::from_cache("yes").unwrap();
    assert_eq!(1, 1);
}
