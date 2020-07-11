use rscache::{ Loader, CacheError };

mod common;

#[test]
fn load_woodsman_tutor() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let npc_loader = common::load_npcs(&cache)?;

    let npc = npc_loader.load(3226).unwrap();

    assert_eq!("Woodsman tutor", npc.name);
    assert!(npc.interactable);

    Ok(())
}

#[test]
fn load_last_npc() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let npc_loader = common::load_npcs(&cache)?;

    let npc = npc_loader.load(8691).unwrap();

    assert_eq!("Mosol Rei", npc.name);
    assert!(npc.interactable);

    Ok(())
}

#[test]
fn incorrect_npc_id() -> Result<(), CacheError> {
    let cache = common::setup()?;
    let npc_loader = common::load_npcs(&cache)?;

    let npc = npc_loader.load(65_535);

    assert!(npc.is_none());

    Ok(())
}