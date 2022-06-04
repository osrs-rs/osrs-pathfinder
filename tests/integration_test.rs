use osrs_pathfinder::Pathfinder;

#[test]
fn test_one() {
    let pathfinder = Pathfinder::new("./data/cache").unwrap();
    assert_eq!(1, 1);
}
