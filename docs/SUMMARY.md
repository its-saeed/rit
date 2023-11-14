# Table of contents

## Lets' start

* [Install rust](README.md)
* [Create the project structure](lets-start/readme.md)

## Git init command

* [Implement git init Command](git-init-command/implement-git-init-command.md)
* [Create a GitConfig struct](git-init-command/create-a-gitconfig-struct.md)
* [Create a GitRepository struct](git-init-command/create-a-gitrepository-struct.md)
* [Refactor!](git-init-command/refactor.md)
* [Implement GitRepository::create](git-init-command/implement-gitrepository-create.md)
* [Let's add some integration tests](git-init-command/lets-add-some-integration-tests.md)
* [A better approach to handling errors](git-init-command/a-better-approach-to-handling-errors.md)
* [Add find function to GitRepository](git-init-command/add-find-function-to-gitrepository.md)
* [Finish init command implementation](git-init-command/finish-init-command-implementation.md)

## Git cat-file command

* [Create a git object module](git-cat-file-command/create-a-git-object-module.md)
* [Add BlobObject](git-cat-file-command/add-blobobject.md)
* [Add ObjectParseError](git-cat-file-command/add-objectparseerror.md)
* [Implement read function](git-cat-file-command/implement-read-function.md)
* [Update argument parser](git-cat-file-command/update-argument-parser.md)
* [Finish cat-file command implementation](git-cat-file-command/finish-cat-file-command-implementation.md)
* [The hash-object command](git-cat-file-command/the-hash-object-command.md)

## Refactor

* [Split error types](refactor/split-error-types.md)
* [Split git\_object module](refactor/split-git\_object-module.md)
* [Add SerializedGitObject](refactor/add-serializedgitobject.md)
* [Add CompressedGitObject](refactor/add-compressedgitobject.md)
* [Move read and write objects to repository module](refactor/move-read-and-write-objects-to-repository-module.md)
* [Change GitObject to an enum](refactor/change-gitobject-to-an-enum.md)
* [Move each git object to separate file](refactor/move-each-git-object-to-separate-file.md)

## Reading commit history

* [Parsing commits](reading-commit-history/parsing-commits.md)
* [Add log command](reading-commit-history/add-log-command.md)
* [A few more refactors and fixes](reading-commit-history/a-few-more-refactors-and-fixes.md)
* [Start parsing tree objects](reading-commit-history/start-parsing-tree-objects.md)
* [Add ls-tree](reading-commit-history/add-ls-tree.md)
* [Add checkout command](reading-commit-history/add-checkout-command.md)

## Refs, tags and branches

* [Implement show-ref command](refs-tags-and-branches/implement-show-ref-command.md)
* [Implement tag command](refs-tags-and-branches/implement-tag-command.md)
* [Improve find\_object](refs-tags-and-branches/improve-find\_object.md)
