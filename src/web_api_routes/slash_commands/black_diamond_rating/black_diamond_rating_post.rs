use crate::app_state::ao_data::AO;
use crate::db::queries::users::get_user_by_slack_id;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::interactive_events::interaction_payload::BasicValue;
use crate::web_api_routes::slash_commands::modal_utils::value_utils;
use sqlx::PgPool;
use std::collections::HashMap;

pub mod post_ids {
    pub const PAX_COUNT: &str = "pax-count.input";
    pub const VESTS_REMOVED: &str = "vests-removed.input";
    pub const MILES: &str = "miles.input";
    pub const AVG_HR: &str = "avg-heart-rate.input";
    pub const WHERE_POST: &str = "where_to_post.select";
}

#[derive(Debug)]
pub struct BlackDiamondRatingPost {
    pax_count: usize,
    vests_removed: usize,
    miles: f32,
    avg_hr: f32,
    post_where: AO,
}

mod calculation_consts {
    pub const MILES_DIVIDED: f32 = 2.;
    pub const HEART_RATE_DIVIDED: f32 = 140.;
}

fn float_value_formatted(value: f32) -> String {
    format!("{:.2}", value)
}

impl BlackDiamondRatingPost {
    fn miles_fmt(&self) -> String {
        float_value_formatted(self.miles)
    }

    fn avg_hr_fmt(&self) -> String {
        float_value_formatted(self.avg_hr)
    }

    fn part_one(&self) -> f32 {
        self.vests_removed as f32 / self.pax_count as f32
    }

    fn part_one_fmt(&self) -> String {
        float_value_formatted(self.part_one())
    }

    fn part_two(&self) -> f32 {
        self.miles / calculation_consts::MILES_DIVIDED
    }

    fn part_two_fmt(&self) -> String {
        float_value_formatted(self.part_two())
    }

    fn part_three(&self) -> f32 {
        self.avg_hr / calculation_consts::HEART_RATE_DIVIDED
    }

    fn part_three_fmt(&self) -> String {
        float_value_formatted(self.part_three())
    }

    fn total(&self) -> f32 {
        self.part_one() + self.part_two() + self.part_three()
    }

    pub fn total_fmt(&self) -> String {
        float_value_formatted(self.total())
    }
}

impl From<HashMap<String, BasicValue>> for BlackDiamondRatingPost {
    fn from(value: HashMap<String, BasicValue>) -> Self {
        let pax_count =
            value_utils::get_value(&value, post_ids::PAX_COUNT, value_utils::get_single_usize)
                .unwrap_or_default();

        let vests_removed = value_utils::get_value(
            &value,
            post_ids::VESTS_REMOVED,
            value_utils::get_single_usize,
        )
        .unwrap_or_default();

        let miles = value_utils::get_value(&value, post_ids::MILES, value_utils::get_single_float)
            .unwrap_or_default();

        let avg_hr =
            value_utils::get_value(&value, post_ids::AVG_HR, value_utils::get_single_float)
                .unwrap_or_default();

        let post_where = value
            .get(post_ids::WHERE_POST)
            .map(value_utils::get_ao_value)
            .unwrap_or_else(|| AO::Unknown("Missing AO".to_string()));

        BlackDiamondRatingPost {
            pax_count,
            vests_removed,
            miles,
            avg_hr,
            post_where,
        }
    }
}

pub async fn convert_to_message(
    post: BlackDiamondRatingPost,
    db_pool: &PgPool,
    user_id: &str,
) -> PostMessageRequest {
    let channel_id = post.post_where.channel_id();

    let user = get_user_by_slack_id(db_pool, user_id)
        .await
        .unwrap_or_default();

    let desc = format!(
        "1. # vests removed / Pax\n\
2. # miles / {}\n\
3. Avg heart rate of Pax / {}",
        calculation_consts::MILES_DIVIDED,
        calculation_consts::HEART_RATE_DIVIDED
    );

    let break_down = format!(
        "1. {}/{} vests off = {}\n\
2. {} miles/{} = {}\n\
3. {} hr/{} = {}",
        post.vests_removed,
        post.pax_count,
        post.part_one_fmt(),
        post.miles_fmt(),
        calculation_consts::MILES_DIVIDED,
        post.part_two_fmt(),
        post.avg_hr_fmt(),
        calculation_consts::HEART_RATE_DIVIDED,
        post.part_three_fmt()
    );

    let total = format!("*Total*: {} :black-diamond-1:", post.total_fmt());

    let block_builder = BlockBuilder::new()
        .section_markdown("*BLACK DIAMOND GRADING*")
        .img_markdown(
            desc.as_str(),
            "https://img.freepik.com/free-vector/shiny-background-design_1415-103.jpg?w=100",
            "black-diamond",
        )
        .section_markdown(break_down.as_str())
        .divider()
        .section_markdown(total.as_str());

    if let Some(user) = user {
        PostMessageRequest::new_as_user(channel_id, block_builder.blocks, user)
    } else {
        PostMessageRequest::new(channel_id, block_builder.blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_calc() {
        let post = BlackDiamondRatingPost {
            pax_count: 8,
            vests_removed: 8,
            miles: 3.72,
            avg_hr: 140.5,
            post_where: AO::BlackDiamond,
        };
        assert_eq!(post.total_fmt(), 3.86.to_string());
    }
}
