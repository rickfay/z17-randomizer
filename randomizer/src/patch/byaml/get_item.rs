use crate::patch::Patcher;
use crate::Result;

/// GetItem.byaml patches
/// FIXME causes visual effects to stop working for... some reason.
#[allow(unused)]
pub fn patch(patcher: &mut Patcher) -> Result<()> {
    // Read and deserialize GetItem.byaml from RegionBoot
    // let raw = patcher.boot.archive.get_mut().read("World/Byaml/GetItem.byaml").unwrap();
    // let file_get_items: File<GetItems> = raw.try_map(|data| byaml::from_bytes(&data)).unwrap();

    //let get_items = &mut file_get_items.get_mut().0;

    // Unused "None" Item
    // let none_item = get_items.get_mut(Item::Empty as usize).expect("Couldn't get \"None\" GetItem entry");
    // none_item.1 = String::from("Actor/KeyBoss.bch");
    // none_item.2 = 1.0;
    // none_item.set_345(Vec3 { x: 0.0, y: 0.0, z: -0.2 });
    // none_item.set_678(Vec3 { x: 0.0, y: -0.64, z: 0.0 });
    // none_item.set_rotate(Vec3 { x: -56.16, y: 0.0, z: 0.0 });
    // none_item.15 = -2;
    // none_item.16 = -2;
    // none_item.17 = -2;
    // none_item.18 = 0;
    // none_item.19 = 1;

    // Update
    // let serialized = file_get_items.serialize();
    // patcher.boot.archive.get_mut().update(serialized).unwrap();

    Ok(())
}
