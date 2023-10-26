mod test_utils;
use std::{env::temp_dir, fs};

use rit::{git_config::GitConfig, repository::GitRepository};
use uuid::Uuid;

use crate::test_utils::{
    directory_manager::create_directory_manager, general::generate_random_path,
};

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
    let work_tree = generate_random_path();

    // Make work tree a file
    fs::write(&work_tree, "test").unwrap();

    assert!(GitRepository::create(&work_tree).is_err())
}

#[test]
fn try_from_a_directory_manager_should_be_fine_if_it_contains_a_dot_git() {
    // Arrange
    let dir_manager = create_directory_manager();

    // Act
    GitRepository::create(&dir_manager.work_tree).unwrap();

    // Assert
    assert!(GitRepository::try_from(dir_manager).is_ok());
}

#[test]
fn if_current_dir_is_a_git_repo_load_should_be_fine() {
    // Arrange
    let dir_manager = create_directory_manager();

    // Act, Create a real repo
    GitRepository::create(&dir_manager.work_tree).unwrap();

    // Assert
    assert!(GitRepository::find(&dir_manager.work_tree).is_ok());
}

#[test]
fn if_one_of_the_parent_directories_is_a_git_repo_load_should_be_fine() {
    // Arrange
    let dir_manager = create_directory_manager();

    // Act, Create a real repo
    GitRepository::create(&dir_manager.work_tree).unwrap();

    let sub_dir_in_repo = dir_manager
        .work_tree
        .join("sub_dir_in_repo")
        .join("another_sub_dir");
    fs::create_dir_all(&sub_dir_in_repo).unwrap();

    assert!(GitRepository::find(&sub_dir_in_repo).is_ok());
}

#[test]
fn if_none_of_the_parents_contains_a_dot_git_load_should_fail() {
    // Arrange
    let path = generate_random_path();

    assert!(GitRepository::find(&path).is_err());
}
