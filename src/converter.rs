use crate::structs::*;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn convert(entries: Vec<Entry>) {
    println!("Converting outfits...");

    let outfit_chance = if entries.is_empty() {
        1
    } else {
        (100 / entries.len()).max(1)
    };

    let file = File::create("WardrobeINIConverter/ConvertedLines.txt")
        .expect("Failed to create output file");
    let mut writer = BufWriter::with_capacity(65536, file);

    for entry in entries {
        let mut glasses_set = CompCombo::default();
        let mut hat_set = CompCombo::default();
        let mut ear_set = CompCombo::default();
        let mut beard_set = CompCombo::default();
        let mut shirt_set = CompCombo::default();
        let mut undercoat_set = CompCombo::default();
        let mut decal_set = CompCombo::default();
        let mut accessory_set = CompCombo::default();
        let mut pants_set = CompCombo::default();
        let mut shoes_set = CompCombo::default();
        let mut vest_set = CompCombo::default();
        let mut upper_skin_set = CompCombo::default();
        let mut parachute_set = CompCombo::default();

        for combo in &entry.combos {
            let CompCombo {
                comp_name,
                comp_id,
                comp_tex,
            } = combo;
            match *comp_name {
                "Glasses" => {
                    glasses_set.comp_id = *comp_id;
                    glasses_set.comp_tex = *comp_tex;
                }
                "Hat" => {
                    hat_set.comp_id = *comp_id;
                    hat_set.comp_tex = *comp_tex;
                }
                "Ear" => {
                    ear_set.comp_id = *comp_id;
                    ear_set.comp_tex = *comp_tex;
                }
                "Mask" => {
                    beard_set.comp_id = *comp_id;
                    beard_set.comp_tex = *comp_tex;
                }
                "Top" => {
                    shirt_set.comp_id = *comp_id;
                    shirt_set.comp_tex = *comp_tex;
                }
                "UnderCoat" => {
                    undercoat_set.comp_id = *comp_id;
                    undercoat_set.comp_tex = *comp_tex;
                }
                "Decal" => {
                    decal_set.comp_id = *comp_id;
                    decal_set.comp_tex = *comp_tex;
                }
                "Accessories" => {
                    accessory_set.comp_id = *comp_id;
                    accessory_set.comp_tex = *comp_tex;
                }
                "Pants" => {
                    pants_set.comp_id = *comp_id;
                    pants_set.comp_tex = *comp_tex;
                }
                "Shoes" => {
                    shoes_set.comp_id = *comp_id;
                    shoes_set.comp_tex = *comp_tex;
                }
                "Armor" => {
                    vest_set.comp_id = *comp_id;
                    vest_set.comp_tex = *comp_tex;
                }
                "UpperSkin" => {
                    upper_skin_set.comp_id = *comp_id;
                    upper_skin_set.comp_tex = *comp_tex;
                }
                "Parachute" => {
                    parachute_set.comp_id = *comp_id;
                    parachute_set.comp_tex = *comp_tex;
                }
                _ => {}
            }
        }

        let outfit_comment = &entry.entry_name;
        let gender = if entry.gender == "Male" {
            "MP_M_FREEMODE_01"
        } else {
            "MP_F_FREEMODE_01"
        };

        writeln!(
            writer,
            "<!-- {} --> <Ped chance=\"{}\" \
            prop_glasses=\"{}\" tex_glasses=\"{}\" \
            prop_hats=\"{}\" tex_hats=\"{}\" \
            prop_ears=\"{}\" tex_ears=\"{}\" \
            comp_beard=\"{}\" tex_beard=\"{}\" \
            comp_shirtoverlay=\"{}\" tex_shirtoverlay=\"{}\" \
            comp_shirt=\"{}\" tex_shirt=\"{}\" \
            comp_decals=\"{}\" tex_decals=\"{}\" \
            comp_accessories=\"{}\" tex_accessories=\"{}\" \
            comp_pants=\"{}\" tex_pants=\"{}\" \
            comp_shoes=\"{}\" tex_shoes=\"{}\" \
            comp_eyes=\"{}\" tex_eyes=\"{}\" \
            comp_tasks=\"{}\" tex_tasks=\"{}\" \
            comp_hands=\"{}\" tex_hands=\"{}\">{}</Ped>",
            outfit_comment,
            outfit_chance,
            glasses_set.comp_id,
            glasses_set.comp_tex,
            hat_set.comp_id,
            hat_set.comp_tex,
            ear_set.comp_id,
            ear_set.comp_tex,
            beard_set.comp_id,
            beard_set.comp_tex,
            shirt_set.comp_id,
            shirt_set.comp_tex,
            upper_skin_set.comp_id,
            upper_skin_set.comp_tex,
            decal_set.comp_id,
            decal_set.comp_tex,
            undercoat_set.comp_id,
            undercoat_set.comp_tex,
            pants_set.comp_id,
            pants_set.comp_tex,
            shoes_set.comp_id,
            shoes_set.comp_tex,
            accessory_set.comp_id,
            accessory_set.comp_tex,
            vest_set.comp_id,
            vest_set.comp_tex,
            parachute_set.comp_id,
            parachute_set.comp_tex,
            gender
        )
        .expect("Failed to write to file");
    }

    println!("Finished converting and writing to file.");
}
