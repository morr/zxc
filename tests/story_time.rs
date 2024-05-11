mod story_time {
    use zxc::{ElapsedTime, CONFIG};

    fn day_to_seconds(amount: f32) -> f32 {
        CONFIG.time.day_duration * amount
    }

    #[test]
    fn default_time() {
        let subject = ElapsedTime::default();

        assert_eq!(subject.total_days(), 1);
        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.day_hour(), CONFIG.starting_scene.day_hour);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn zero_time() {
        let subject = ElapsedTime(0.0);

        assert_eq!(subject.total_seconds(), 0.0);
        assert_eq!(subject.day_time(), 0.0);
        assert_eq!(subject.total_days(), 1);
        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.day_hour(), 0);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn hour_ten_minutes() {
        let subject = ElapsedTime(day_to_seconds(1. / 24.) + day_to_seconds(1. / 24. / 60. * 10.));

        assert_eq!(subject.total_days(), 1);
        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.day_hour(), 1);
        assert_eq!(subject.hour_minute(), 10);
    }

    #[test]
    fn noon() {
        let subject = ElapsedTime(day_to_seconds(1. / 2.));

        assert_eq!(subject.total_seconds(), day_to_seconds(1. / 2.));
        assert_eq!(subject.day_time(), 0.5);
        assert_eq!(subject.total_days(), 1);
        assert_eq!(subject.year_day(), 1);
        assert_eq!(subject.day_hour(), 12);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn day() {
        let subject = ElapsedTime(day_to_seconds(1.));

        assert_eq!(subject.total_seconds(), day_to_seconds(1.));
        assert_eq!(subject.day_time(), 0.0);
        assert_eq!(subject.total_days(), 2);
        assert_eq!(subject.year_day(), 2);
        assert_eq!(subject.day_hour(), 0);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn day_and_half() {
        let subject = ElapsedTime(day_to_seconds(1.5));

        assert_eq!(subject.day_time(), 0.5);
        assert_eq!(subject.total_days(), 2);
        assert_eq!(subject.year_day(), 2);
        assert_eq!(subject.day_hour(), 12);
        assert_eq!(subject.hour_minute(), 0);
    }

    #[test]
    fn year_day_and_half() {
        let subject = ElapsedTime(day_to_seconds(CONFIG.time.days_in_year as f32 + 1.5));

        assert_eq!(subject.day_time(), 0.5);
        assert_eq!(subject.total_days(), 2 + CONFIG.time.days_in_year);
        assert_eq!(subject.year_day(), 2);
        assert_eq!(subject.day_hour(), 12);
        assert_eq!(subject.hour_minute(), 0);
    }
}
