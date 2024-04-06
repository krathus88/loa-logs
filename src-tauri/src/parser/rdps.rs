use crate::parser::entity_tracker::Entity;
use crate::parser::models::{CombatEffectConditionData, CombatEffectData, CombatEffectDetail, EncounterEntity, EntityType, PassiveOption, SkillBuffData, SkillData, IDENTITY_CATEGORY, NPC_GRADE, STAT_TYPE_MAP, STAT_TYPE_MAP_TRA, Skill};
use hashbrown::HashMap;
use crate::parser::encounter_state::is_support_class_id;

pub fn get_buff_after_tripods(
    buff: &SkillBuffData,
    entity: &EncounterEntity,
    skill_id: u32,
    skill_effect_id: u32,
) -> SkillBuffData {
    let mut buff_data = buff.clone();
    let skill_effect_id = skill_effect_id as i32;
    let skill = &entity.skills.get(&skill_id);
    if let Some(skill) = skill.cloned() {
        if let Some(tripod_data) = skill.tripod_data {
            for tripod in tripod_data {
                for tripod_option in tripod.options {
                    let params = tripod_option.param;
                    let feature_type = tripod_option.effect_type;
                    let i0 = params.first().cloned().unwrap_or_default();
                    if feature_type == "change_buff_stat" {
                        if i0 == 0 || i0 == skill_effect_id {
                            let buff_id = params.get(1).cloned().unwrap_or_default();
                            if buff.id == buff_id {
                                let mut change_map: HashMap<i32, i32> = HashMap::new();
                                for i in (2..params.len()).step_by(2) {
                                    let stat_type = params.get(i).cloned();
                                    let value = params.get(i + 1).cloned();
                                    if let (Some(stat_type), Some(value)) = (stat_type, value) {
                                        change_map.insert(stat_type, value);
                                    }
                                }
                                for passive_option in buff_data.passive_option.iter_mut() {
                                    let change = change_map.get(
                                        &(STAT_TYPE_MAP[passive_option.key_stat.as_str()] as i32),
                                    );
                                    if passive_option.option_type == "stat" {
                                        if let Some(change) = change.cloned() {
                                            if tripod_option.param_type == "absolute" {
                                                passive_option.value += change;
                                            } else {
                                                passive_option.value = (passive_option.value as f32
                                                    * (1.0 + change as f32 / 100.0))
                                                    .round()
                                                    as i32;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if feature_type == "add_buff_stat" {
                        if i0 == 0 || i0 == skill_effect_id {
                            let buff_id = params.get(1).cloned().unwrap_or_default();
                            if buff.id == buff_id {
                                let key_stat = params.get(2).cloned();
                                let value = params.get(3).cloned();
                                if let (Some(key_stat), Some(value)) = (key_stat, value) {
                                    buff_data.passive_option.push(PassiveOption {
                                        option_type: "stat".to_string(),
                                        key_stat: STAT_TYPE_MAP_TRA
                                            .get(&(key_stat as u32))
                                            .unwrap()
                                            .to_string(),
                                        key_index: 0,
                                        value,
                                    });
                                }
                            }
                        }
                    } else if feature_type == "change_buff_param" {
                        if let Some(status_effect_values) = buff_data.status_effect_values.as_mut()
                        {
                            if i0 == 0 || i0 == skill_effect_id {
                                let buff_id = params.get(1).cloned().unwrap_or_default();
                                if buff.id == buff_id {
                                    if params.get(2).cloned().unwrap_or_default() == 0 {
                                        buff_data.status_effect_values = Some(params[3..].to_vec());
                                    } else {
                                        let mut new_values: Vec<i32> = vec![];
                                        for i in
                                            0..(status_effect_values.len()).max(params.len() - 3)
                                        {
                                            if params.get(i + 3).is_some() {
                                                let old_value = status_effect_values
                                                    .get(i)
                                                    .cloned()
                                                    .unwrap_or_default();
                                                let new_value = (old_value as f32
                                                    * (1.0
                                                        + params
                                                            .get(i + 3)
                                                            .cloned()
                                                            .unwrap_or_default()
                                                            as f32
                                                            / 100.0)
                                                        .round())
                                                    as i32;
                                                new_values.push(new_value);
                                            }
                                        }
                                        buff_data.status_effect_values = Some(new_values);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    buff_data
}

pub fn get_crit_multiplier_from_combat_effect(
    ce: &CombatEffectData,
    ce_condition_data: CombatEffectConditionData,
) -> f64 {
    let mut crit_damage_rate = 0.0;

    ce.effects
        .iter()
        .filter(|effect| {
            effect
                .actions
                .iter()
                .any(|action| action.action_type == "modify_critical_multiplier")
        })
        .for_each(|effect| {
            if is_combat_effect_condition_valid(
                effect,
                ce_condition_data.self_entity,
                ce_condition_data.target_entity,
                ce_condition_data.caster_entity,
                ce_condition_data.skill,
                ce_condition_data.hit_option,
                ce_condition_data.target_count,
            ) {
                effect
                    .actions
                    .iter()
                    .filter(|action| action.action_type == "modify_critical_multiplier")
                    .for_each(|action| {
                        if action.action_type == "modify_critical_multiplier" {
                            let val = action.args.first().cloned().unwrap_or_default() as f64 / 100.0;
                            crit_damage_rate += val;
                        }
                    })
            }
        });

    crit_damage_rate
}

pub fn is_combat_effect_condition_valid(
    effect: &CombatEffectDetail,
    self_entity: Option<&Entity>,
    target_entity: Option<&Entity>,
    caster_entity: Option<&Entity>,
    skill: Option<&SkillData>,
    hit_option: Option<i32>,
    target_count: Option<i32>,
) -> bool {
    let mut is_valid = true;
    for condition in effect.conditions.iter() {
        if !is_valid {
            break;
        }

        let actor = &condition.actor;
        match condition.condition_type.as_str() {
            "target_count" => {
                if target_count.is_none() || target_count.unwrap() != condition.arg {
                    is_valid = false;
                }
            }
            "current_skill" => {
                if skill.is_none() || skill.unwrap().id != condition.arg {
                    is_valid = false;
                }
            }
            "pc" => {
                let is_player = |entity_option: Option<&Entity>| -> bool {
                    match entity_option {
                        Some(entity) => entity.entity_type == EntityType::PLAYER,
                        None => false,
                    }
                };

                is_valid = match actor.as_str() {
                    "self" => is_player(self_entity),
                    "target" => is_player(target_entity),
                    "caster" => is_player(caster_entity),
                    _ => false,
                };
            }
            "skill_identity_category" => {
                if let Some(skill) = skill {
                    if let Some(identity_category) = &skill.identity_category {
                        if *IDENTITY_CATEGORY.get(identity_category.as_str()).unwrap()
                            != condition.arg
                        {
                            is_valid = false;
                        }
                    } else {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "abnormal_move_immune" => {
                if let Some(target_entity) = target_entity {
                    if target_entity.entity_type != EntityType::BOSS || !target_entity.push_immune {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "abnormal_move_all" | "abnormal_move" | "abnormal_status" => {
                is_valid = false;
            }
            "current_skill_group" => {
                if let Some(skill) = skill {
                    if let Some(groups) = &skill.groups {
                        if !groups.contains(&condition.arg) {
                            is_valid = false;
                        }
                    } else {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "hp_less" => {
                let check_validity = |entity_option: Option<&Entity>| -> bool {
                    if let Some(entity) = entity_option {
                        if let (Some(hp), Some(max_hp)) =
                            (entity.stats.get(&1), entity.stats.get(&27))
                        {
                            *hp * 100 >= *max_hp * condition.arg as i64
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                is_valid = !match actor.as_str() {
                    "self" => check_validity(self_entity),
                    "target" => check_validity(target_entity),
                    "caster" => check_validity(caster_entity),
                    _ => false, // Default case if actor doesn't match any known roles
                };
            }
            "npc_scaled_level_less" => {
                if actor == "target" {
                    if let Some(target_entity) = target_entity {
                        if target_entity.entity_type != EntityType::BOSS
                            || target_entity.balance_level < condition.arg as u16
                        {
                            is_valid = false;
                        }
                    } else {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "npc_grade_less" => {
                if actor == "target" {
                    if let Some(target_entity) = target_entity {
                        let grade = NPC_GRADE
                            .get(target_entity.grade.as_str())
                            .cloned()
                            .unwrap_or_default();
                        if target_entity.entity_type != EntityType::BOSS || grade > condition.arg {
                            is_valid = false;
                        }
                    } else {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "npc_grade_greater" => {
                if actor == "target" {
                    if let Some(target_entity) = target_entity {
                        let grade = NPC_GRADE
                            .get(target_entity.grade.as_str())
                            .cloned()
                            .unwrap_or_default();
                        if target_entity.entity_type != EntityType::BOSS || grade < condition.arg {
                            is_valid = false;
                        }
                    } else {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "identity_stance" => {
                if actor == "self" {
                    if self_entity.is_none()
                        || self_entity.unwrap().entity_type != EntityType::PLAYER
                        || self_entity.unwrap().stance as i32 != condition.arg
                    {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            "directional_attack" => {
                if let Some(hit_option) = hit_option {
                    if (hit_option + 1) & condition.arg == 0 {
                        is_valid = false;
                    }
                } else {
                    is_valid = false;
                }
            }
            _ => {
                is_valid = false;
            }
        }
    }

    is_valid
}

pub fn apply_rdps(damage_owner: &mut EncounterEntity, source_entity: Option<&mut EncounterEntity>, skill: &mut Skill, delta: i64) {
    if let Some(source) = source_entity {
        source.damage_stats.rdps_damage_given += delta;
        
        if is_support_class_id(source.class_id) {
            damage_owner.damage_stats.rdps_damage_received_support += delta;
            skill.rdps_damage_received_support += delta;
        }
    }
    
    damage_owner.damage_stats.rdps_damage_received += delta;
    skill.rdps_damage_received += delta;    
}
