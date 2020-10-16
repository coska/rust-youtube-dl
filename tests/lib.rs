#[cfg(test)]
mod tests {
    #[test]
    fn valid_url() {
        let test_valid_url = "https://youtu.be/test-id";
        let parsed_url = rust_youtube_dl::parse_url(&test_valid_url);
        assert_eq!(parsed_url.host_str(), Some("youtube.com"));
    }

    #[test]
    #[should_panic]
    fn invalid_url() {
        let test_invalid_url = "Test Fail Wrong";
        let _parsed_url = rust_youtube_dl::parse_url(&test_invalid_url);
    }

    #[test]
    fn invalid_youtube_url() {
        assert!(true);
    }

    #[test]
    fn argument_num() {
        assert_eq!(true, true);
    }

    #[test]
    fn resource_not_found() {
        assert_eq!(true, true);
    }

    #[test]
    fn network_not_connected() {
        assert_eq!(true, true);
    }

    #[test]
    fn network_disrupted() {
        assert_eq!(true, true);
    }

    #[test]
    fn channel_download_mulitple_video() {
        assert_eq!(true, true);
    }

    #[test]
    fn restricted_video_should_not_download() {
        assert_eq!(true, true);
    }

    #[test]
    fn youtube_clip_should_exist() {
        assert!(true);    
    }

    #[test]
    fn should_check_disk_space() {
        assert!(true);    
    }

    #[test]
    fn should_have_write_permission() {
        assert!(true);    
    }
}