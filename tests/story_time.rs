mod story_time {
    use zxc::{ElapsedTime, CONFIG};

    fn day_duration() -> f32 {
        CONFIG.time.day_duration
    }

    #[test]
    fn zero_time() {
        let subject = ElapsedTime(0.0);

        assert_eq!(subject.total_seconds(), 0.0);
        assert_eq!(subject.game_time_of_day(), 0.0);
        assert_eq!(subject.game_day(), 0);
        assert_eq!(subject.game_hours(), 0);
        assert_eq!(subject.game_minutes(), 0);
    }

    #[test]
    fn hour_ten_minutes() {
        let subject = ElapsedTime(day_duration() / 24.0 + day_duration() / 24.0 / 60.0 * 10.0);

        assert_eq!(subject.game_day(), 0);
        assert_eq!(subject.game_hours(), 1);
        assert_eq!(subject.game_minutes(), 10);
    }

    #[test]
    fn noon() {
        let subject = ElapsedTime(day_duration() / 2.0);

        assert_eq!(subject.total_seconds(), day_duration() / 2.0);
        assert_eq!(subject.game_time_of_day(), 0.5);
        assert_eq!(subject.game_day(), 0);
        assert_eq!(subject.game_hours(), 12);
        assert_eq!(subject.game_minutes(), 0);
    }

    #[test]
    fn day() {
        let subject = ElapsedTime(day_duration());

        assert_eq!(subject.total_seconds(), day_duration());
        assert_eq!(subject.game_time_of_day(), 0.0);
        assert_eq!(subject.game_day(), 1);
        assert_eq!(subject.game_hours(), 0);
        assert_eq!(subject.game_minutes(), 0);
    }

    #[test]
    fn day_and_half() {
        let subject = ElapsedTime(day_duration() * 1.5);

        assert_eq!(subject.game_time_of_day(), 0.5);
        assert_eq!(subject.game_day(), 1);
        assert_eq!(subject.game_hours(), 12);
        assert_eq!(subject.game_minutes(), 0);
    }
}
