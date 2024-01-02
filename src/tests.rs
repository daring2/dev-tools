use super::*;

#[test]
fn test_build_gradle_command() {
    let args1 = ReleaseArgs { next_version: "0.1".to_string(), clean: false, build: false, };
    assert_eq!(
        build_gradle_command(&args1),
        "gradlew.bat --no-daemon"
    );
    let args2 = ReleaseArgs { next_version: "0.1".to_string(), clean: true, build: true, };
    assert_eq!(
        build_gradle_command(&args2),
        "gradlew.bat --no-daemon clean build"
    );
}
