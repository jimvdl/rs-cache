use rscache::CacheError;

mod common;

#[test]
fn load_blue_partyhat() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let item_loader = common::load_items(&cache)?;

    let item = item_loader.load(1042).unwrap();

    assert_eq!("Blue partyhat", item.name);
    assert_eq!(false, item.stackable);
    assert_eq!(false, item.members_only);

    Ok(())
}

#[test]
fn load_magic_logs() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let item_loader = common::load_items(&cache)?;

    let item = item_loader.load(1513).unwrap();

    assert_eq!("Magic logs", item.name);
    assert_eq!(false, item.stackable);
    assert_eq!(true, item.members_only);

    Ok(())
}

#[test]
fn load_logs_noted() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let item_loader = common::load_items(&cache)?;

    let item = item_loader.load(1512).unwrap();

    assert_eq!(true, item.stackable);
    assert_eq!(false, item.members_only);

    Ok(())
}

#[test]
fn incorrect_item_id() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let item_loader = common::load_items(&cache)?;

    let item = item_loader.load(65_535);

    assert_eq!(true, item.is_none());

    Ok(())
}