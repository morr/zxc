mod apply_default_global_config;

mod story_time {
    use zxc::*;

    fn day_to_seconds(amount: f32) -> f32 {
        config().time.day_duration * amount
    }

    #[test]
    fn default_time() {
        let subject = 0.0;

        assert_eq!(total_seconds(subject), total_seconds(0.0));
        assert_eq!(total_days(subject), 0);

        assert_eq!(current_year_day(subject), 1);
        assert_eq!(current_season_day(subject), 1);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_hour(subject), config().starting_scene.day_hour);
        assert_eq!(current_hour_minute(subject), 0);
    }

    #[test]
    fn zero_time() {
        let subject = -total_seconds(0.0);

        assert_eq!(total_seconds(subject), 0.0);
        assert_eq!(total_days(subject), 0);

        assert_eq!(current_year_day(subject), 1);
        assert_eq!(current_season_day(subject), 1);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_normalized_time(subject), 0.0);
        assert_eq!(current_day_hour(subject), 0);
        assert_eq!(current_hour_minute(subject), 0);
    }

    #[test]
    fn hour_ten_minutes() {
        let subject = day_to_seconds(1. / 24. + 1. / 24. / 60. * 10.) - total_seconds(0.0);

        assert_eq!(total_days(subject), 0);

        assert_eq!(current_year_day(subject), 1);
        assert_eq!(current_season_day(subject), 1);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_hour(subject), 1);
        assert_eq!(current_hour_minute(subject), 10);
    }

    #[test]
    fn noon() {
        let subject = day_to_seconds(1. / 2.) - total_seconds(0.0);

        assert_eq!(total_seconds(subject), day_to_seconds(1. / 2.));
        assert_eq!(total_days(subject), 0);

        assert_eq!(current_year_day(subject), 1);
        assert_eq!(current_season_day(subject), 1);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_normalized_time(subject), 0.5);
        assert_eq!(current_day_hour(subject), 12);
        assert_eq!(current_hour_minute(subject), 0);
    }

    #[test]
    fn day() {
        let subject = day_to_seconds(1.) - total_seconds(0.0);

        assert_eq!(total_seconds(subject), day_to_seconds(1.));
        assert_eq!(total_days(subject), 1);

        assert_eq!(current_year_day(subject), 2);
        assert_eq!(current_season_day(subject), 2);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_normalized_time(subject), 0.0);
        assert_eq!(current_day_hour(subject), 0);
        assert_eq!(current_hour_minute(subject), 0);
    }

    #[test]
    fn day_and_half() {
        let subject = day_to_seconds(1.5) - total_seconds(0.0);

        assert_eq!(total_days(subject), 1);

        assert_eq!(current_year_day(subject), 2);
        assert_eq!(current_season_day(subject), 2);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_normalized_time(subject), 0.5);
        assert_eq!(current_day_hour(subject), 12);
        assert_eq!(current_hour_minute(subject), 0);
    }

    #[test]
    fn two_seasons() {
        let subject = day_to_seconds(config().time.days_in_season as f32 * 2.) - total_seconds(0.0);

        assert_eq!(
            total_days(subject),
            (config().time.days_in_season as f32 * 2.) as u32
        );

        assert_eq!(
            current_year_day(subject),
            (config().time.days_in_season as f32 * 2.) as u32 + 1
        );
        assert_eq!(current_season_day(subject), 1);

        assert_eq!(current_year(subject), 1);
        assert_eq!(current_year_season(subject), YearSeason::Fall);

        assert_eq!(current_day_hour(subject), 0);
        assert_eq!(current_hour_minute(subject), 0);
    }

    #[test]
    fn current_year_day_and_half() {
        let subject = day_to_seconds(config().time.days_in_year as f32 + 1.5) - total_seconds(0.0);

        assert_eq!(total_days(subject), 1 + config().time.days_in_year);

        assert_eq!(current_year_day(subject), 2);
        assert_eq!(current_season_day(subject), 2);

        assert_eq!(current_year(subject), 2);
        assert_eq!(current_year_season(subject), YearSeason::Spring);

        assert_eq!(current_day_normalized_time(subject), 0.5);
        assert_eq!(current_day_hour(subject), 12);
        assert_eq!(current_hour_minute(subject), 0);
    }

    // #[test]
    // fn season_index() {
    //     assert_eq!(
    //         ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 0.)).season_index(),
    //         0
    //     );
    //     assert_eq!(
    //         ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 1.) - 1.0)
    //             .season_index(),
    //         0
    //     );
    //     assert_eq!(
    //         ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 1.)).season_index(),
    //         1
    //     );
    //     assert_eq!(
    //         ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 1.) + 1.0)
    //             .season_index(),
    //         1
    //     );
    // }

    #[test]
    fn spring() {
        let subject = day_to_seconds(config().time.days_in_season as f32 * 0.) - total_seconds(0.0);

        assert_eq!(current_year_season(subject), YearSeason::Spring);
    }

    #[test]
    fn summer() {
        let subject = day_to_seconds(config().time.days_in_season as f32 * 1.) - total_seconds(0.0);

        assert_eq!(current_year_season(subject), YearSeason::Summer);
    }

    #[test]
    fn fall() {
        let subject = day_to_seconds(config().time.days_in_season as f32 * 2.) - total_seconds(0.0);

        assert_eq!(current_year_season(subject), YearSeason::Fall);
    }

    #[test]
    fn winter() {
        let subject = day_to_seconds(config().time.days_in_season as f32 * 3.) - total_seconds(0.0);

        assert_eq!(current_year_season(subject), YearSeason::Winter);
    }

    #[test]
    fn next_year_spring() {
        let subject = day_to_seconds(config().time.days_in_season as f32 * 4.) - total_seconds(0.0);

        assert_eq!(current_year_season(subject), YearSeason::Spring);
    }

    #[test]
    fn total_day_to_year_day() {
        assert_eq!(story_time::total_day_to_year_day(0), 1);
        assert_eq!(story_time::total_day_to_year_day(1), 2);
        assert_eq!(
            story_time::total_day_to_year_day(config().time.days_in_year),
            1
        );
        assert_eq!(
            story_time::total_day_to_year_day(config().time.days_in_year * 10),
            1
        );
        assert_eq!(
            story_time::total_day_to_year_day(config().time.days_in_year * 10 + 1),
            2
        );
    }

    #[test]
    fn year_day_to_season() {
        assert_eq!(story_time::year_day_to_season(1), YearSeason::Spring);
        assert_eq!(
            story_time::year_day_to_season(config().time.days_in_season - 1),
            YearSeason::Spring
        );
        assert_eq!(
            story_time::year_day_to_season(config().time.days_in_season),
            YearSeason::Spring
        );
        assert_eq!(
            story_time::year_day_to_season(config().time.days_in_season + 1),
            YearSeason::Summer
        );
        assert_eq!(
            story_time::year_day_to_season(config().time.days_in_season * 2 + 1),
            YearSeason::Fall
        );
        assert_eq!(
            story_time::year_day_to_season(config().time.days_in_season * 3 + 1),
            YearSeason::Winter
        );
    }

    #[test]
    fn year_day_to_season_day() {
        assert_eq!(story_time::year_day_to_season_day(1), 1);
        assert_eq!(
            story_time::year_day_to_season_day(config().time.days_in_season),
            config().time.days_in_season
        );
        assert_eq!(
            story_time::year_day_to_season_day(config().time.days_in_season + 1),
            1
        );
    }

    #[test]
    fn year_day_to_season_day_label() {
        assert_eq!(story_time::year_day_to_season_day_label(1), "1st of Spring");
        assert_eq!(
            story_time::year_day_to_season_day_label(config().time.days_in_season),
            format!("{}th of Spring", config().time.days_in_season)
        );
        assert_eq!(
            story_time::year_day_to_season_day_label(config().time.days_in_season + 1),
            "1st of Summer"
        );
    }
}
