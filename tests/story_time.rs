mod apply_default_global_config;

mod story_time {
    use zxc::*;

    fn day_to_seconds(amount: f32) -> f32 {
        config().time.day_duration * amount
    }

    #[test]
    fn default_time() {
        let subject = ElapsedTime::default();

        assert_eq!(subject.total_days(), 0);

        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.season_day(), 1);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_hour(), config().starting_scene.day_hour);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn zero_time() {
        let subject = ElapsedTime(0.0);

        assert_eq!(subject.total_seconds(), 0.0);
        assert_eq!(subject.total_days(), 0);

        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.season_day(), 1);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_time(), 0.0);
        assert_eq!(subject.day_hour(), 0);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn hour_ten_minutes() {
        let subject = ElapsedTime(day_to_seconds(1. / 24.) + day_to_seconds(1. / 24. / 60. * 10.));

        assert_eq!(subject.total_days(), 0);

        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.season_day(), 1);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_hour(), 1);
        assert_eq!(subject.hour_minute(), 10);
    }

    #[test]
    fn noon() {
        let subject = ElapsedTime(day_to_seconds(1. / 2.));

        assert_eq!(subject.total_seconds(), day_to_seconds(1. / 2.));
        assert_eq!(subject.total_days(), 0);

        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.season_day(), 1);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_time(), 0.5);
        assert_eq!(subject.day_hour(), 12);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn day() {
        let subject = ElapsedTime(day_to_seconds(1.));

        assert_eq!(subject.total_seconds(), day_to_seconds(1.));
        assert_eq!(subject.total_days(), 1);

        assert_eq!(subject.year_day(), 2);
        assert_eq!(subject.season_day(), 2);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_time(), 0.0);
        assert_eq!(subject.day_hour(), 0);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn day_and_half() {
        let subject = ElapsedTime(day_to_seconds(1.5));

        assert_eq!(subject.total_days(), 1);

        assert_eq!(subject.year_day(), 2);
        assert_eq!(subject.season_day(), 2);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_time(), 0.5);
        assert_eq!(subject.day_hour(), 12);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn two_seasons() {
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 2.));

        assert_eq!(
            subject.total_days(),
            (config().time.days_in_season as f32 * 2.) as u32
        );

        assert_eq!(
            subject.year_day(),
            (config().time.days_in_season as f32 * 2.) as u32 + 1
        );
        assert_eq!(subject.season_day(), 1);

        assert_eq!(subject.year(), 1);
        assert_eq!(subject.year_season(), YearSeason::Fall);

        assert_eq!(subject.day_hour(), 0);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn year_day_and_half() {
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_year as f32 + 1.5));

        assert_eq!(subject.total_days(), 1 + config().time.days_in_year);

        assert_eq!(subject.year_day(), 2);
        assert_eq!(subject.season_day(), 2);

        assert_eq!(subject.year(), 2);
        assert_eq!(subject.year_season(), YearSeason::Spring);

        assert_eq!(subject.day_time(), 0.5);
        assert_eq!(subject.day_hour(), 12);
        assert_eq!(subject.hour_minute(), 0);
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
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 0.));

        assert_eq!(subject.year_season(), YearSeason::Spring);
    }

    #[test]
    fn summer() {
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 1.));

        assert_eq!(subject.year_season(), YearSeason::Summer);
    }

    #[test]
    fn fall() {
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 2.));

        assert_eq!(subject.year_season(), YearSeason::Fall);
    }

    #[test]
    fn winter() {
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 3.));

        assert_eq!(subject.year_season(), YearSeason::Winter);
    }

    #[test]
    fn next_year_spring() {
        let subject = ElapsedTime(day_to_seconds(config().time.days_in_season as f32 * 4.));

        assert_eq!(subject.year_season(), YearSeason::Spring);
    }

    #[test]
    fn total_day_to_year_day() {
        assert_eq!(ElapsedTime::total_day_to_year_day(0), 1);
        assert_eq!(ElapsedTime::total_day_to_year_day(1), 2);
        assert_eq!(
            ElapsedTime::total_day_to_year_day(config().time.days_in_year),
            1
        );
        assert_eq!(
            ElapsedTime::total_day_to_year_day(config().time.days_in_year * 10),
            1
        );
        assert_eq!(
            ElapsedTime::total_day_to_year_day(config().time.days_in_year * 10 + 1),
            2
        );
    }

    #[test]
    fn year_day_to_season() {
        assert_eq!(ElapsedTime::year_day_to_season(1), YearSeason::Spring);
        assert_eq!(
            ElapsedTime::year_day_to_season(config().time.days_in_season - 1),
            YearSeason::Spring
        );
        assert_eq!(
            ElapsedTime::year_day_to_season(config().time.days_in_season),
            YearSeason::Spring
        );
        assert_eq!(
            ElapsedTime::year_day_to_season(config().time.days_in_season + 1),
            YearSeason::Summer
        );
        assert_eq!(
            ElapsedTime::year_day_to_season(config().time.days_in_season * 2 + 1),
            YearSeason::Fall
        );
        assert_eq!(
            ElapsedTime::year_day_to_season(config().time.days_in_season * 3 + 1),
            YearSeason::Winter
        );
    }

    #[test]
    fn year_day_to_season_day() {
        assert_eq!(ElapsedTime::year_day_to_season_day(1), 1);
        assert_eq!(
            ElapsedTime::year_day_to_season_day(config().time.days_in_season),
            config().time.days_in_season
        );
        assert_eq!(
            ElapsedTime::year_day_to_season_day(config().time.days_in_season + 1),
            1
        );
    }

    #[test]
    fn year_day_to_season_day_label() {
        assert_eq!(
            ElapsedTime::year_day_to_season_day_label(1),
            "1st of Spring"
        );
        assert_eq!(
            ElapsedTime::year_day_to_season_day_label(config().time.days_in_season),
            format!("{}th of Spring", config().time.days_in_season)
        );
        assert_eq!(
            ElapsedTime::year_day_to_season_day_label(config().time.days_in_season + 1),
            "1st of Summer"
        );
    }
}
