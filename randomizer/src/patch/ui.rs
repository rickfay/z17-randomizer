use crate::{Patcher, Result};
use log::info;
use rom::{File, lyt::{Animation, Keyframes, Layout, Target}};

pub fn patch(patcher: &mut Patcher) -> Result<()> {
    info!("Patching UI...");
    add_button_to_pause_menu(patcher)?;
    Ok(())
}

fn add_button_to_pause_menu(patcher: &mut Patcher) -> Result<()> {
    let mut layout_file = get_layout(patcher, "Game", "Gm_PauseT")?;
    let layout = layout_file.get_mut();
    layout.get_pane_mut("N_HeadLine_00")?.pos_y = 84.0;
    layout.get_pane_mut("L_BtnRT_00")?.pos_y = 24.0;
    layout.get_pane_mut("L_BtnLB_00")?.pos_y = -80.0;
    let mut btn3 = layout.get_pane("L_BtnLB_00")?.clone();
    btn3.name = "L_Btn3_00".to_string();
    btn3.pos_y = -28.0;
    layout.add_child("RootPane", btn3)?;
    layout.get_group_mut("G_Btn_00")?.add_pane("L_Btn3_00");
    update_layout(patcher, "Game", layout_file)?;

    let mut anim_file = get_animation(patcher, "Game", "Gm_PauseT_InBtnTB")?;
    let anim = anim_file.get_mut();
    if let Keyframes::Float(keyframes) = &mut anim.get_object_mut("L_BtnRT_00")?.get_target_mut(Target::TranslationY)?.keyframes {
        keyframes[0].value = 24.0;
        keyframes[1].value = 36.0;
        keyframes[2].value = 34.0;
    }
    if let Keyframes::Float(keyframes) = &mut anim.get_object_mut("L_BtnLB_00")?.get_target_mut(Target::TranslationY)?.keyframes {
        keyframes[0].value = -80.0;
        keyframes[1].value = -68.0;
        keyframes[2].value = -70.0;
    }
    let mut anim_btn3 = anim.get_object("L_BtnLB_00")?.clone();
    anim_btn3.name = "L_Btn3_00".to_string();
    if let Keyframes::Float(keyframes) = &mut anim_btn3.get_target_mut(Target::TranslationY)?.keyframes {
        keyframes[0].value = -28.0;
        keyframes[1].value = -16.0;
        keyframes[2].value = -18.0;
    }
    anim.add_object(anim_btn3)?;
    update_animation(patcher, "Game", anim_file)?;

    let mut anim_file = get_animation(patcher, "Game", "Gm_PauseT_OutBtnTB")?;
    let anim = anim_file.get_mut();
    if let Keyframes::Float(keyframes) = &mut anim.get_object_mut("L_BtnRT_00")?.get_target_mut(Target::TranslationY)?.keyframes {
        keyframes[0].value = 34.0;
        keyframes[1].value = 24.0;
    }
    if let Keyframes::Float(keyframes) = &mut anim.get_object_mut("L_BtnLB_00")?.get_target_mut(Target::TranslationY)?.keyframes {
        keyframes[0].value = -70.0;
        keyframes[1].value = -80.0;
    }
    let mut anim_btn3 = anim.get_object("L_BtnLB_00")?.clone();
    anim_btn3.name = "L_Btn3_00".to_string();
    if let Keyframes::Float(keyframes) = &mut anim_btn3.get_target_mut(Target::TranslationY)?.keyframes {
        keyframes[0].value = -18.0;
        keyframes[1].value = -28.0;
    }
    anim.add_object(anim_btn3)?;
    update_animation(patcher, "Game", anim_file)?;

    Ok(())
}

fn get_layout(patcher: &mut Patcher, archive_name: &str, layout_name: &str) -> Result<File<Layout>> {
    Ok(patcher.lyt(archive_name)?.get_layout(layout_name)?)
}

fn update_layout(patcher: &mut Patcher, archive_name: &str, file: File<Layout>) -> Result<()> {
    Ok(patcher.lyt(archive_name)?.update_layout(file)?)
}

fn get_animation(patcher: &mut Patcher, archive_name: &str, anim_name: &str) -> Result<File<Animation>> {
    Ok(patcher.lyt(archive_name)?.get_animation(anim_name)?)
}

fn update_animation(patcher: &mut Patcher, archive_name: &str, file: File<Animation>) -> Result<()> {
    Ok(patcher.lyt(archive_name)?.update_animation(file)?)
}
