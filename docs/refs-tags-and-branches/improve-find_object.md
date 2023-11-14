# Improve find\_object

{% code title="src/repository/mod.rs" lineNumbers="true" %}
```rust
impl GitRepository {
    fn resolve_object(&self, name: &str) -> Result<Vec<String>, anyhow::Error> {
        let mut candidates = vec![];

        if name == "HEAD" {
            let ref_entry = refs::resolve_ref(
                &self.directory_manager.dot_git_path,
                &self.directory_manager.dot_git_path.join("HEAD"),
            )?;
            return Ok(vec![ref_entry]);
        }

        let regex = regex::Regex::new("^[0-9A-Fa-f]{4,40}$").unwrap();
        if regex.is_match(&name) {
            let directory = &name[0..2].to_lowercase();
            let path = self.directory_manager.objects_path.join(directory);
            for entry in path.read_dir()? {
                let entry = entry?.path();
                let filename = entry
                    .file_name()
                    .ok_or(anyhow::anyhow!("Failed to get filename"))?
                    .to_str()
                    .ok_or(anyhow::anyhow!("Failed to get the filename"))?;
                if filename.starts_with(&name[2..]) {
                    candidates.push(format!("{}{}", directory, filename))
                }
            }
        }

        if let Ok(tag) = self.resolve_ref(&format!("refs/tags/{}", name)) {
            candidates.push(tag);
        }

        if let Ok(branch) = self.resolve_ref(&format!("refs/heads/{}", name)) {
            candidates.push(branch);
        }

        Ok(candidates)
    }

    pub fn find_object(&self, name: &str) -> Result<String, anyhow::Error> {
        let candidates = self.resolve_object(name)?;
        match candidates.len() {
            1 => Ok(candidates[0].clone()),
            0 => Err(anyhow::anyhow!("Not a hash")),
            _ => Err(anyhow::anyhow!("Ambiguous object name")),
        }
    }

```
{% endcode %}

<pre class="language-toml" data-title="" data-line-numbers><code class="lang-toml">[dependencies]
anyhow = "1.0.75"
clap = { version = "4.4.6", features = ["cargo"] }
colored = "2.0.4"
configparser = "3.0.2"
flate2 = "1.0.28"
sha1_smol = { version = "1.0.0", features = ["std"] }
thiserror = "1.0.50"
uuid = { version = "1.5.0", features = ["v4"] }
hex = "0.4.3"
<strong>regex = "1.10.2"
</strong></code></pre>
