use nu_path::make_absolute_and_clean_with;
use nu_test_support::fs::Stub::EmptyFile;
use nu_test_support::nu;
use nu_test_support::playground::Playground;
use pretty_assertions::assert_eq;
use std::path::Path;

#[test]
fn make_absolute_and_clean_path() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let mut spam = dirs.test().to_owned();
        spam.push("spam.txt");

        let cwd = std::env::current_dir().expect("Could not get current directory");
        let actual =
            make_absolute_and_clean_with(spam, cwd).expect("Failed to make absolute and clean");

        assert!(actual.ends_with("spam.txt"));
    });
}

#[test]
fn make_absolute_and_clean_unicode_path() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("🚒.txt")]);

        let mut spam = dirs.test().to_owned();
        spam.push("🚒.txt");

        let cwd = std::env::current_dir().expect("Could not get current directory");

        let actual =
            make_absolute_and_clean_with(spam, cwd).expect("Failed to make absolute and clean");

        assert!(actual.ends_with("🚒.txt"));
    });
}

#[ignore]
#[test]
fn make_absolute_and_clean_non_utf8_path() {
    // TODO
}

#[test]
fn make_absolute_and_clean_path_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_unicode_path_relative_to_unicode_path_with_spaces() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("e-$ èрт🚒♞中片-j");
        sandbox.with_files(&[EmptyFile("e-$ èрт🚒♞中片-j/🚒.txt")]);

        let mut relative_to = dirs.test().to_owned();
        relative_to.push("e-$ èрт🚒♞中片-j");

        let actual = make_absolute_and_clean_with("🚒.txt", relative_to)
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("e-$ èрт🚒♞中片-j/🚒.txt");

        assert_eq!(actual, expected);
    });
}

#[ignore]
#[test]
fn make_absolute_and_clean_non_utf8_path_relative_to_non_utf8_path_with_spaces() {
    // TODO
}

#[test]
fn make_absolute_and_clean_absolute_path_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let mut absolute_path = dirs.test().to_owned();
        absolute_path.push("spam.txt");

        let actual = make_absolute_and_clean_with(&absolute_path, "non/existent/directory")
            .expect("Failed to make absolute and clean");
        let expected = absolute_path;

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_dot() {
    let expected = std::env::current_dir().expect("Could not get current directory");

    let actual = make_absolute_and_clean_with(".", expected.as_path())
        .expect("Failed to make absolute and clean");

    assert_eq!(actual, expected);
}

#[test]
fn make_absolute_and_clean_many_dots() {
    let expected = std::env::current_dir().expect("Could not get current directory");

    let actual = make_absolute_and_clean_with("././/.//////./././//.///", expected.as_path())
        .expect("Failed to make absolute and clean");

    assert_eq!(actual, expected);
}

#[test]
fn make_absolute_and_clean_path_with_dot_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("./spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_many_dots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("././/.//////./././//.////spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_double_dot() {
    let cwd = std::env::current_dir().expect("Could not get current directory");
    let actual =
        make_absolute_and_clean_with("..", &cwd).expect("Failed to make absolute and clean");
    let expected = cwd
        .parent()
        .expect("Could not get parent of current directory");

    assert_eq!(actual, expected);
}

#[test]
fn make_absolute_and_clean_path_with_double_dot_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("foo/../spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_many_double_dots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar/baz");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("foo/bar/baz/../../../spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_ndots2() {
    // This test will fail if you have the nushell repo on the root partition
    // So, let's start in a nested folder before trying to make_absolute_and_clean_with "..."
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("aaa/bbb/ccc");
        let output = nu!( cwd: dirs.root(), "cd nu_path_test_1/aaa/bbb/ccc; $env.PWD");
        let cwd = Path::new(&output.out);

        let actual =
            make_absolute_and_clean_with("...", cwd).expect("Failed to make absolute and clean");
        let expected = cwd
            .parent()
            .expect("Could not get parent of current directory")
            .parent()
            .expect("Could not get parent of a parent of current directory");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_3_ndots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("foo/bar/.../spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_many_3_ndots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar/baz/eggs/sausage/bacon");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with(
            "foo/bar/baz/eggs/sausage/bacon/.../.../.../spam.txt",
            dirs.test(),
        )
        .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_4_ndots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar/baz");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with("foo/bar/baz/..../spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_many_4_ndots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar/baz/eggs/sausage/bacon");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let actual = make_absolute_and_clean_with(
            "foo/bar/baz/eggs/sausage/bacon/..../..../spam.txt",
            dirs.test(),
        )
        .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_path_with_way_too_many_dots_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar/baz/eggs/sausage/bacon/vikings");
        sandbox.with_files(&[EmptyFile("spam.txt")]);

        let mut relative_to = dirs.test().to_owned();
        relative_to.push("foo/bar/baz/eggs/sausage/bacon/vikings");

        let actual =
            make_absolute_and_clean_with("././..////././...///././.....///spam.txt", relative_to)
                .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_unicode_path_with_way_too_many_dots_relative_to_unicode_path_with_spaces(
) {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/áčěéí  +šř=é/baz/eggs/e-$ èрт🚒♞中片-j/bacon/öäöä öäöä");
        sandbox.with_files(&[EmptyFile("🚒.txt")]);

        let mut relative_to = dirs.test().to_owned();
        relative_to.push("foo/áčěéí  +šř=é/baz/eggs/e-$ èрт🚒♞中片-j/bacon/öäöä öäöä");

        let actual =
            make_absolute_and_clean_with("././..////././...///././.....///🚒.txt", relative_to)
                .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("🚒.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_tilde() {
    let tilde_path = "~";

    let cwd = std::env::current_dir().expect("Could not get current directory");
    let actual =
        make_absolute_and_clean_with(tilde_path, cwd).expect("Failed to make absolute and clean");

    assert!(actual.is_absolute());
    assert!(!actual.starts_with("~"));
}

#[test]
fn make_absolute_and_clean_tilde_relative_to() {
    let tilde_path = "~";

    let actual = make_absolute_and_clean_with(tilde_path, "non/existent/path")
        .expect("Failed to make absolute and clean");

    assert!(actual.is_absolute());
    assert!(!actual.starts_with("~"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn make_absolute_and_clean_symlink() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);
        sandbox.symlink("spam.txt", "link_to_spam.txt");

        let mut symlink_path = dirs.test().to_owned();
        symlink_path.push("link_to_spam.txt");

        let cwd = std::env::current_dir().expect("Could not get current directory");
        let actual = make_absolute_and_clean_with(symlink_path, cwd)
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("link_to_spam.txt");

        assert_eq!(actual, expected);
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn make_absolute_and_clean_symlink_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);
        sandbox.symlink("spam.txt", "link_to_spam.txt");

        let actual = make_absolute_and_clean_with("link_to_spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("link_to_spam.txt");

        assert_eq!(actual, expected);
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(windows))] // seems like Windows symlink requires existing file or dir
#[test]
fn make_absolute_and_clean_symlink_loop_relative_to_without_touching_filesystem() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        // sandbox.with_files(vec![EmptyFile("spam.txt")]);
        sandbox.symlink("spam.txt", "link_to_spam.txt");
        sandbox.symlink("link_to_spam.txt", "spam.txt");

        let actual = make_absolute_and_clean_with("link_to_spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("link_to_spam.txt");

        assert_eq!(actual, expected);
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn make_absolute_and_clean_nested_symlink_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.with_files(&[EmptyFile("spam.txt")]);
        sandbox.symlink("spam.txt", "link_to_spam.txt");
        sandbox.symlink("link_to_spam.txt", "link_to_link_to_spam.txt");

        let actual = make_absolute_and_clean_with("link_to_link_to_spam.txt", dirs.test())
            .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("link_to_link_to_spam.txt");

        assert_eq!(actual, expected);
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn make_absolute_and_clean_nested_symlink_within_symlink_dir_relative_to() {
    Playground::setup("nu_path_test_1", |dirs, sandbox| {
        sandbox.mkdir("foo/bar/baz");
        sandbox.with_files(&[EmptyFile("foo/bar/baz/spam.txt")]);
        sandbox.symlink("foo/bar/baz/spam.txt", "foo/bar/link_to_spam.txt");
        sandbox.symlink("foo/bar/link_to_spam.txt", "foo/link_to_link_to_spam.txt");
        sandbox.symlink("foo", "link_to_foo");

        let actual =
            make_absolute_and_clean_with("link_to_foo/link_to_link_to_spam.txt", dirs.test())
                .expect("Failed to make absolute and clean");
        let mut expected = dirs.test().to_owned();
        expected.push("link_to_foo/link_to_link_to_spam.txt");

        assert_eq!(actual, expected);
    });
}

#[test]
fn make_absolute_and_clean_without_touching_filesystem() {
    let path = Path::new("/foo/bar/baz"); // hopefully, this path does not exist

    let cwd = std::env::current_dir().expect("Could not get current directory");

    let actual =
        make_absolute_and_clean_with(path, cwd).expect("failed to make absolute and clean");
    let expected = Path::new("/foo/bar/baz").to_path_buf();

    assert_eq!(actual, expected);
}

#[test]
fn make_absolute_and_clean_relative_to_without_touching_filesystem() {
    let relative_to = "/foo";
    let path = "bar/baz";

    let actual =
        make_absolute_and_clean_with(path, relative_to).expect("failed to make absolute and clean");
    let expected = Path::new("/foo/bar/baz").to_path_buf();

    assert_eq!(actual, expected);
}

#[cfg(windows)]
#[test]
fn make_absolute_and_clean_unc() {
    // Ensure that canonicalizing UNC paths does not turn them verbatim.
    // Assumes the C drive exists and that the `localhost` UNC path works.
    let actual = nu_path::make_absolute_and_clean_with(r"\\localhost\c$", ".")
        .expect("failed to make_absolute_and_clean");
    let expected = Path::new(r"\\localhost\c$");
    assert_eq!(actual, expected);
}
