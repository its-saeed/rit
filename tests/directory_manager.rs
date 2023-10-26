mod test_utils;

use std::env::temp_dir;
use test_utils::directory_manager::create_directory_manager;

use rit::DirectoryManager;
use uuid::Uuid;

#[test]
fn directory_tree_should_be_created_successfully() {
    let dir_manager = create_directory_manager();

    dir_manager.create_directory_tree().unwrap();

    assert!(dir_manager.dot_git_path.exists());
    assert!(dir_manager.branches_path.exists());
    assert!(dir_manager.objects_path.exists());
    assert!(dir_manager.refs_heads_path.exists());
    assert!(dir_manager.refs_tags_path.exists());
}

#[test]
fn if_work_tree_directory_is_empty_is_dot_git_empty_should_return_true() {
    let dir_manager = create_directory_manager();
    std::fs::create_dir_all(&dir_manager.work_tree).unwrap();

    assert!(dir_manager.is_dot_git_empty().unwrap());
}

#[test]
fn if_dot_git_is_empty_is_dot_git_empty_should_return_true() {
    let dir_manager = DirectoryManager::new(temp_dir().join(Uuid::new_v4().to_string()));

    std::fs::create_dir_all(&dir_manager.dot_git_path).unwrap();

    assert!(dir_manager.is_dot_git_empty().unwrap());
}

#[test]
fn if_dot_git_is_not_empty_is_dot_git_empty_should_return_false() {
    let dir_manager = create_directory_manager();

    // To create .git folder and all of its children.
    dir_manager.create_directory_tree().unwrap();

    assert!(!dir_manager.is_dot_git_empty().unwrap());
}
