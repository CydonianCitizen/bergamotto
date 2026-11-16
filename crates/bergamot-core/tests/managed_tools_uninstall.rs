use bergamot_core::managed_tools::uninstall_tool;
use std::fs;
use std::io::ErrorKind;
use tempfile::tempdir;

#[test]
fn rejects_parent_path_without_removing_a_sibling() {
    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    let sibling = temp.path().join("outside");
    fs::create_dir_all(&tools_dir).unwrap();
    fs::create_dir_all(&sibling).unwrap();
    fs::write(sibling.join("keep.txt"), "keep me").unwrap();

    let error = uninstall_tool(&tools_dir, "../outside").unwrap_err();

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(
        fs::read_to_string(sibling.join("keep.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn rejects_absolute_path_without_removing_its_target() {
    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    let outside = temp.path().join("outside");
    fs::create_dir_all(&tools_dir).unwrap();
    fs::create_dir_all(&outside).unwrap();
    fs::write(outside.join("keep.txt"), "keep me").unwrap();

    let error = uninstall_tool(&tools_dir, outside.to_str().unwrap()).unwrap_err();

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(
        fs::read_to_string(outside.join("keep.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn rejects_paths_with_separators() {
    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    let nested_tool = tools_dir.join("nested").join("tool");
    let trailing_separator_tool = tools_dir.join("ffmpeg");
    fs::create_dir_all(&nested_tool).unwrap();
    fs::create_dir_all(&trailing_separator_tool).unwrap();
    fs::write(nested_tool.join("keep.txt"), "keep me").unwrap();
    fs::write(trailing_separator_tool.join("keep.txt"), "keep me").unwrap();

    for invalid_name in ["nested/tool", "ffmpeg/"] {
        let error = uninstall_tool(&tools_dir, invalid_name).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidInput, "{invalid_name}");
    }

    assert_eq!(
        fs::read_to_string(nested_tool.join("keep.txt")).unwrap(),
        "keep me"
    );
    assert_eq!(
        fs::read_to_string(trailing_separator_tool.join("keep.txt")).unwrap(),
        "keep me"
    );
}

#[cfg(unix)]
#[test]
fn rejects_backslash_as_a_separator() {
    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    let tool_with_backslash = tools_dir.join("nested\\tool");
    fs::create_dir_all(&tool_with_backslash).unwrap();
    fs::write(tool_with_backslash.join("keep.txt"), "keep me").unwrap();

    let error = uninstall_tool(&tools_dir, "nested\\tool").unwrap_err();

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(
        fs::read_to_string(tool_with_backslash.join("keep.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn removes_only_the_named_managed_tool_child() {
    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    let managed_child = tools_dir.join("ffmpeg");
    let sibling = tools_dir.join("python3");
    fs::create_dir_all(&managed_child).unwrap();
    fs::create_dir_all(&sibling).unwrap();
    fs::write(managed_child.join("binary"), "managed").unwrap();
    fs::write(sibling.join("binary"), "keep me").unwrap();

    uninstall_tool(&tools_dir, "ffmpeg").unwrap();

    assert!(!managed_child.exists());
    assert_eq!(
        fs::read_to_string(sibling.join("binary")).unwrap(),
        "keep me"
    );
}

#[cfg(unix)]
#[test]
fn removes_a_symlink_child_without_touching_its_target() {
    use std::os::unix::fs::symlink;

    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    let outside = temp.path().join("outside");
    fs::create_dir_all(&tools_dir).unwrap();
    fs::create_dir_all(&outside).unwrap();
    fs::write(outside.join("keep.txt"), "keep me").unwrap();
    symlink(&outside, tools_dir.join("ffmpeg")).unwrap();

    uninstall_tool(&tools_dir, "ffmpeg").unwrap();

    assert!(!tools_dir.join("ffmpeg").exists());
    assert_eq!(
        fs::read_to_string(outside.join("keep.txt")).unwrap(),
        "keep me"
    );
}

#[test]
fn rejects_dot_components_without_removing_the_tools_root() {
    let temp = tempdir().unwrap();
    let tools_dir = temp.path().join("tools");
    fs::create_dir_all(&tools_dir).unwrap();
    fs::write(tools_dir.join("keep.txt"), "keep me").unwrap();

    let error = uninstall_tool(&tools_dir, ".").unwrap_err();

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(
        fs::read_to_string(tools_dir.join("keep.txt")).unwrap(),
        "keep me"
    );
}
