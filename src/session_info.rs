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
    pub camera_info: CameraInfo,
    pub radio_info: RadioInfo,
    pub driver_info: DriverInfo,
    pub split_time_info: SplitTimeInfo,
    pub car_setup: CarSetup,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarSetup {
    pub tires: Tires,
    pub chassis: Chassis,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Chassis {
    pub front: ChassisFront,
    pub left_front: ChassisLeftFront,
    pub left_rear: ChassisLeftRear,
    pub in_car_dials: InCarDials,
    pub right_front: ChassisRightFront,
    pub right_rear: ChassisRightRear,
    pub rear: ChassisRear,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InCarDials {
    pub dash_display_page: String,
    pub brake_pressure_bias: String,
    pub brake_pads: String,
    pub abs_setting: String,
    pub tc_setting: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisRightFront {
    pub corner_weight: String,
    pub ride_height: String,
    pub spring_rate: String,
    pub spring_perch_offset: String,
    pub bump_stiffness: String,
    pub rebound_stiffness: String,
    pub camber: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisRightRear {
    pub corner_weight: String,
    pub ride_height: String,
    pub spring_perch_offset: String,
    pub bump_stiffness: String,
    pub rebound_stiffness: String,
    pub camber: String,
    pub toe_in: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisLeftFront {
    pub corner_weight: String,
    pub ride_height: String,
    pub spring_rate: String,
    pub spring_perch_offset: String,
    pub bump_stiffness: String,
    pub rebound_stiffness: String,
    pub camber: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisLeftRear {
    pub corner_weight: String,
    pub ride_height: String,
    pub spring_perch_offset: String,
    pub bump_stiffness: String,
    pub rebound_stiffness: String,
    pub camber: String,
    pub toe_in: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisFront {
    pub arb_setting: i32,
    pub toe_in: String,
    pub cross_weight: String,
    pub nose_weight: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChassisRear {
    pub fuel_level: String,
    pub arb_setting: i32,
    pub wing_setting: i32,
    pub diff_clutches: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tires {
    pub left_front: LeftTire,
    pub left_rear: LeftTire,
    pub right_front: RightTire,
    pub right_rear: RightTire,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeftTire {
    pub starting_pressure: String,
    pub last_hot_pressure: String,
    #[serde(rename = "LastTempsOMI")]
    pub last_temps_omi: String,
    pub tread_remaining: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RightTire {
    pub starting_pressure: String,
    pub last_hot_pressure: String,
    #[serde(rename = "LastTempsIMO")]
    pub last_temps_imo: String,
    pub tread_remaining: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SplitTimeInfo {
    pub sectors: Vec<Sector>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sector {
    pub sector_num: i32,
    pub sector_start_pct: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverInfo {
    pub driver_car_idx: i32,
    #[serde(rename = "DriverUserID")]
    pub driver_user_id: i32,
    pub pace_car_idx: i32,
    pub driver_head_pos_x: f32,
    pub driver_head_pos_y: f32,
    pub driver_head_pos_z: f32,
    pub driver_car_is_electric: i32,
    #[serde(rename = "DriverCarIdleRPM")]
    pub driver_car_idle_rpm: f32,
    pub driver_car_red_line: f32,
    pub driver_car_eng_cylinder_count: i32,
    pub driver_car_fuel_kg_per_ltr: f32,
    pub driver_car_fuel_max_ltr: f32,
    pub driver_car_max_fuel_pct: f32,
    pub driver_car_gear_num_forward: i32,
    pub driver_car_gear_neutral: i32,
    pub driver_car_gear_reverse: i32,
    #[serde(rename = "DriverCarSLFirstRPM")]
    pub driver_car_sl_first_rpm: f32,
    #[serde(rename = "DriverCarSLShiftRPM")]
    pub driver_car_sl_shift_rpm: f32,
    #[serde(rename = "DriverCarSLLastRPM")]
    pub driver_car_sl_last_rpm: f32,
    #[serde(rename = "DriverCarSLBlinkRPM")]
    pub driver_car_sl_blink_rpm: f32,
    pub driver_car_version: String,
    pub driver_pit_trk_pct: f32,
    pub driver_car_est_lap_time: f32,
    pub driver_setup_name: String,
    pub driver_setup_is_modified: f32,
    pub driver_setup_load_type_name: String,
    pub driver_setup_passed_tech: i32,
    pub driver_incident_count: i32,
    pub drivers: Vec<Driver>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub car_idx: i32,
    pub user_name: String,
    pub abbrev_name: Option<String>,
    pub initials: Option<String>,
    #[serde(rename = "UserID")]
    pub user_id: i32,
    #[serde(rename = "TeamID")]
    pub team_id: i32,
    pub team_name: String,
    pub car_number: String,
    pub car_number_raw: i32,
    pub car_path: String,
    #[serde(rename = "CarClassID")]
    pub car_class_id: i32,
    #[serde(rename = "CarID")]
    pub car_id: i32,
    pub car_is_pace_car: i32,
    #[serde(rename = "CarIsAI")]
    pub car_is_ai: i32,
    pub car_is_electric: i32,
    pub car_screen_name: String,
    pub car_screen_name_short: String,
    pub car_class_short_name: Option<String>,
    pub car_class_rel_speed: i32,
    pub car_class_license_level: i32,
    pub car_class_max_fuel_pct: String,
    pub car_class_weight_penalty: String,
    pub car_class_power_adjust: String,
    pub car_class_dry_tire_set_limit: String,
    pub car_class_color: i32,
    pub car_class_est_lap_time: f32,
    #[serde(rename = "IRating")]
    pub i_rating: i32,
    pub lic_level: i32,
    pub lic_sub_level: i32,
    pub lic_string: String,
    pub lic_color: String,
    pub is_spectator: i32,
    pub car_design_str: String,
    pub helmet_design_str: String,
    pub suit_design_str: String,
    pub car_number_design_str: String,
    #[serde(rename = "CarSponsor_1")]
    pub car_sponsor_1: i32,
    #[serde(rename = "CarSponsor_2")]
    pub car_sponsor_2: i32,
    pub cur_driver_incident_count: i32,
    pub team_incident_count: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioInfo {
    pub selected_radio_num: i32,
    pub radios: Vec<Radio>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Radio {
    pub radio_num: i32,
    pub hop_count: i32,
    pub num_frequencies: i32,
    pub tuned_to_frequency_num: i32,
    pub scanning_is_on: i32,
    pub frequencies: Vec<RadioFrequency>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioFrequency {
    pub frequency_num: i32,
    pub frequency_name: String,
    pub priority: i32,
    pub car_idx: i32,
    pub entry_idx: i32,
    #[serde(rename = "ClubID")]
    pub club_id: i32,
    pub can_scan: i32,
    pub can_squawk: i32,
    pub muted: i32,
    pub is_mutable: i32,
    pub is_deletable: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraInfo {
    pub groups: Vec<CameraGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CameraGroup {
    pub group_num: i32,
    pub group_name: String,
    pub cameras: Vec<Camera>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Camera {
    pub camera_num: i32,
    pub camera_name: String,
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
