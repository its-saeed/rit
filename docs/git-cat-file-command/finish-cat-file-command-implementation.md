# Finish cat-file command implementation

Compiler is not happy yet. Because we added a new entry to `Command` enum but we didn't add it to match expression in `main.rs`.

Let's add it:

{% code title="src/main.rs" lineNumbers="true" %}
```rust
fn main() -> Result<()> {
    let command = parse_args().unwrap();
    match command {
        Command::Init { path } => {
            GitRepository::create(path)?;
        }
        Command::CatFile {
            object_type,
            object_hash,
        } => {
            cmd_cat_file(object_type, object_hash)?;
        }
    };

    Ok(())
}
```
{% endcode %}

If the parsed command is `CatFile` we call `cmd_cat_file`:

{% code title="src/main.rs" lineNumbers="true" %}
```rust
fn cmd_cat_file(object_type: git_object::GitObjectType, object_hash: String) -> Result<()> {
    let current_directory = std::env::current_dir()?;
    let repo = GitRepository::find(&current_directory)?;

    let object = git_object::read(&repo, repo.find_object(object_type, object_hash))?;
    println!("{}", object.serialize());
    Ok(())
}
```
{% endcode %}

First, we get the directory in which the command is executed. Then we create a new GitRepository object using this path. We use `find` function in order to find the repo.

Then in line 5 we read the object and print the result in line 6.  Here we introduced yet another auxiliary function named `find_object` in `GitRepository` struct:

{% code title="src/repository.rs" lineNumbers="true" %}
```rust
    pub fn find_object(&self, _object_type: git_object::GitObjectType, name: String) -> String {
        name
    }
```
{% endcode %}

&#x20;Why do we need this function?

The reason for this strange small function is that Git has a _lot_ of ways to refer to objects: full hash, short hash, tags… `object_find()` will be our name resolution function. We’ll only implement it [later](https://wyag.thb.lt/#object\_find), so this is just a temporary placeholder (so our code can run already, without the real version). This means that until we implement the real thing, the only way we can refer to an object will be by its full hash.\[[source](https://wyag.thb.lt/#cmd-cat-file)]

\
