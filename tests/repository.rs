mod test_utils;
use std::{env::temp_dir, fs};

use rit::{git_config::GitConfig, repository::GitRepository};
use uuid::Uuid;

#[test]
fn if_project_directory_is_empty_create_should_be_successful() {
    let project_dir = test_utils::general::generate_random_path();
    std::fs::create_dir_all(&project_dir).unwrap();

    let repo = GitRepository::create(&project_dir).unwrap();

    let dir_manager = &repo.directory_manager;
    assert!(dir_manager.dot_git_path.exists());
    assert!(dir_manager.branches_path.exists());
    assert!(dir_manager.objects_path.exists());
    assert!(dir_manager.refs_heads_path.exists());
    assert!(dir_manager.refs_tags_path.exists());

    assert_eq!(
        fs::read_to_string(&dir_manager.config_file).unwrap(),
        GitConfig::default_str()
    );
}

#[test]
fn if_dot_git_is_not_empty_create_should_fail() {
    let dir_manager = test_utils::directory_manager::create_directory_manager();

    // To create .git file
    dir_manager.create_directory_tree().unwrap();

    assert!(GitRepository::create(&dir_manager.work_tree).is_err())
}

#[test]
fn if_dot_git_is_empty_create_should_succeed() {
    let work_tree = temp_dir().join(Uuid::new_v4().to_string());
    let dot_git_path = work_tree.join(".git");
    fs::create_dir_all(dot_git_path).unwrap();

    assert!(GitRepository::create(&work_tree).is_ok())
}

#[test]
fn if_work_tree_exists_but_is_not_a_file_create_should_fail() {
    let work_tree = temp_dir().join(Uuid::new_v4().to_string());

    // Make work tree a file
    fs::write(&work_tree, "test").unwrap();

    assert!(GitRepository::create(&work_tree).is_err())
}
