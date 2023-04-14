use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendInfo {
    pub track_name: String,
    #[serde(rename = "TrackID")]
    pub track_id: i32,
    pub track_length: String,
    pub track_length_official: String,
    pub track_display_name: String,
    pub track_display_short_name: String,
    pub track_config_name: String,
    pub track_city: String,
    pub track_country: String,
    pub track_altitude: String,
    pub track_latitude: String,
    pub track_longitude: String,
    pub track_north_offset: String,
    pub track_num_turns: i32,
    pub track_pit_speed_limit: String,
    pub track_type: String,
    pub track_direction: String,
    pub track_weather_type: String,
    pub track_skies: String,
    pub track_surface_temp: String,
    pub track_air_temp: String,
    pub track_air_pressure: String,
    pub track_wind_vel: String,
    pub track_wind_dir: String,
    pub track_relative_humidity: String,
    pub track_fog_level: String,
    pub track_cleanup: i32,
    pub track_dynamic_track: i32,
    pub track_version: String,
    #[serde(rename = "SeriesID")]
    pub series_id: i32,
    #[serde(rename = "SeasonID")]
    pub season_id: i32,
    #[serde(rename = "SessionID")]
    pub session_id: i32,
    #[serde(rename = "SubSessionID")]
    pub sub_session_id: i32,
    #[serde(rename = "LeagueID")]
    pub league_id: i32,
    pub official: i32,
    pub race_week: i32,
    pub event_type: String,
    pub category: String,
    pub sim_mode: String,
    pub team_racing: i32,
    pub min_drivers: i32,
    pub max_drivers: i32,
    #[serde(rename = "DCRuleSet")]
    pub dc_rule_set: String,
    pub qualifier_must_start_race: i32,
    pub num_car_classes: i32,
    pub num_car_types: i32,
    pub heat_racing: i32,
    pub build_type: String,
    pub build_target: String,
    pub build_version: String,
    pub weekend_options: WeekendOptions,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendOptions {
    pub num_starters: i32,
    pub starting_grid: String,
    pub qualify_scoring: String,
    pub course_cautions: String,
    pub standing_start: i32,
    pub short_parade_lap: i32,
    pub restarts: String,
    pub weather_type: String,
    pub skies: String,
    pub wind_direction: String,
    pub wind_speed: String,
    pub weather_temp: String,
    pub relative_humidity: String,
    pub fog_level: String,
    pub time_of_day: String,
    pub date: String,
    pub earth_rotation_speedup_factor: i32,
    pub unofficial: i32,
    pub commercial_mode: String,
    pub night_mode: String,
    pub is_fixed_setup: i32,
    pub strict_laps_checking: String,
    pub has_open_registration: i32,
    pub hardcore_level: i32,
    pub num_joker_laps: i32,
    pub incident_limit: String,
    pub fast_repairs_limit: String,
    pub green_white_checkered_limit: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub weekend_info: WeekendInfo,
    pub session_info: Sessions,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sessions {
    pub sessions: Vec<Session>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    pub session_num: i32,
    pub session_laps: String,
    pub session_time: String,
    pub session_num_laps_to_avg: i32,
    pub session_type: String,
    pub session_track_rubber_state: String,
    pub session_name: String,
    pub session_sub_type: Option<String>,
    pub session_skipped: i32,
    pub session_run_groups_used: i32,
    pub session_enforce_tire_compound_change: i32,
    pub results_fastest_lap: Vec<FastestLap>,
    pub results_average_lap_time: f32,
    pub results_num_caution_flags: i32,
    pub results_num_caution_laps: i32,
    pub results_num_lead_changes: i32,
    pub results_laps_complete: i32,
    pub results_official: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FastestLap {
    #[serde(rename = "CarIdx")]
    pub car_idx: i32,
    pub fastes_lap: Option<i32>,
    pub fastest_time: f32,
}
