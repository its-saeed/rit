use std::{collections::BTreeMap, io::BufRead};

use crate::error::KvlParseError;

#[derive(Debug)]
pub struct Commit {
    pub kvl: KeyValueList,
}

impl Commit {
    pub fn serialize(&self) -> String {
        self.kvl.serialize()
    }

    pub fn deserialize(
        buf_reader: &mut impl std::io::BufRead,
        _object_header: super::Header,
    ) -> Result<Self, crate::error::ObjectParseError> {
        Ok(Self {
            kvl: KeyValueList::parse(buf_reader)?,
        })
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.kvl.map.get(key)
    }
}

#[derive(Debug)]
pub struct KeyValueList {
    map: BTreeMap<String, String>,
}

impl KeyValueList {
    pub fn new(map: BTreeMap<String, String>) -> Self {
        KeyValueList { map }
    }

    pub fn parse(buf_reader: impl BufRead) -> Result<Self, KvlParseError> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();

        let mut the_rest_is_message = false;
        let mut message = String::new();

        let mut prev_key = String::new();

        for line in buf_reader.lines() {
            let line = line.map_err(|_| KvlParseError::FailedToGetNextLine)?;
            if line.is_empty() {
                the_rest_is_message = true;
            } else if the_rest_is_message {
                message.push_str(line.as_str());
            } else if let Some(char) = line.chars().next() {
                if char == ' ' {
                    // Continuation of previous value
                    map.get_mut(&prev_key)
                        .unwrap()
                        .push_str(format!("\n{}", &line[1..]).as_str());
                } else {
                    // New key value
                    let index = line.find(' ').ok_or(KvlParseError::KeyDelimiterNotFound)?;
                    let key = &line[0..index];
                    let value = &line[index + 1..];
                    map.insert(key.to_string(), value.to_string());
                    prev_key = key.to_string();
                }
            }
        }

        if the_rest_is_message {
            map.insert("message".to_string(), message);
        }

        Ok(Self { map })
    }

    pub fn serialize(&self) -> String {
        let mut buffer = String::new();
        let mut message_value = None;
        for (key, value) in &self.map {
            if key == "message" {
                message_value = Some(value);
                continue;
            }
            if value.contains('\n') {
                let multi_line_value = value
                    .lines()
                    .take(1)
                    .map(|v| format!("{}\n", v))
                    .chain(value.lines().skip(1).map(|line| format!(" {}\n", line)))
                    .collect::<String>();
                buffer.push_str(format!("{} {}", key, &multi_line_value).as_str());
            } else {
                buffer.push_str(format!("{} {}\n", key, value).as_str());
            }
        }

        if let Some(message_value) = message_value {
            buffer.push_str(&format!("\n{}", &message_value));
        }
        buffer
    }
}

impl AsRef<BTreeMap<String, String>> for KeyValueList {
    fn as_ref(&self) -> &BTreeMap<String, String> {
        &self.map
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, io::BufReader};

    use super::KeyValueList;

    #[test]
    pub fn kvl_parse_should_parse_single_line_key_values_correctly() {
        let raw = r#"tree 29ff16c9c14e2652b22f8b78bb08a5a07930c147
parent 206941306e8a8af65b66eaaaea388a7ae24d49a0
author Thibault Polge <thibault@thb.lt> 1527025023 +0200
committer Thibault Polge <thibault@thb.lt> 1527025044 +0200
gpgsig -----BEGIN PGP SIGNATURE-----
 
 iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL
 kGQdcBAAqPP+ln4nGDd2gETXjvOpOxLzIMEw4A9gU6CzWzm+oB8mEIKyaH0UFIPh
 rNUZ1j7/ZGFNeBDtT55LPdPIQw4KKlcf6kC8MPWP3qSu3xHqx12C5zyai2duFZUU
 wqOt9iCFCscFQYqKs3xsHI+ncQb+PGjVZA8+jPw7nrPIkeSXQV2aZb1E68wa2YIL
 3eYgTUKz34cB6tAq9YwHnZpyPx8UJCZGkshpJmgtZ3mCbtQaO17LoihnqPn4UOMr
 V75R/7FjSuPLS8NaZF4wfi52btXMSxO/u7GuoJkzJscP3p4qtwe6Rl9dc1XC8P7k
 NIbGZ5Yg5cEPcfmhgXFOhQZkD0yxcJqBUcoFpnp2vu5XJl2E5I/quIyVxUXi6O6c
 /obspcvace4wy8uO0bdVhc4nJ+Rla4InVSJaUaBeiHTW8kReSFYyMmDCzLjGIu1q
 doU61OM3Zv1ptsLu3gUE6GU27iWYj2RWN3e3HE4Sbd89IFwLXNdSuM0ifDLZk7AQ
 WBhRhipCCgZhkj9g2NEk7jRVslti1NdN5zoQLaJNqSwO1MtxTmJ15Ksk3QP6kfLB
 Q52UWybBzpaP9HEd4XnR+HuQ4k2K0ns2KgNImsNvIyFwbpMUyUWLMPimaV1DWUXo
 5SBjDB/V/W2JBFR+XKHFJeFwYhj7DD/ocsGr4ZMx/lgc8rjIBkI=
 =lgTX
 -----END PGP SIGNATURE-----

Create first draft"#;

        let parsed_key_values = BTreeMap::from([
            (
                "tree".to_string(),
                "29ff16c9c14e2652b22f8b78bb08a5a07930c147".to_string(),
            ),
            (
                "parent".to_string(),
                "206941306e8a8af65b66eaaaea388a7ae24d49a0".to_string(),
            ),
            (
                "author".to_string(),
                "Thibault Polge <thibault@thb.lt> 1527025023 +0200".to_string(),
            ),
            (
                "committer".to_string(),
                "Thibault Polge <thibault@thb.lt> 1527025044 +0200".to_string(),
            ),
            (
                "gpgsig".to_string(),
                "-----BEGIN PGP SIGNATURE-----\n\
                \n\
            iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL\n\
            kGQdcBAAqPP+ln4nGDd2gETXjvOpOxLzIMEw4A9gU6CzWzm+oB8mEIKyaH0UFIPh\n\
            rNUZ1j7/ZGFNeBDtT55LPdPIQw4KKlcf6kC8MPWP3qSu3xHqx12C5zyai2duFZUU\n\
            wqOt9iCFCscFQYqKs3xsHI+ncQb+PGjVZA8+jPw7nrPIkeSXQV2aZb1E68wa2YIL\n\
            3eYgTUKz34cB6tAq9YwHnZpyPx8UJCZGkshpJmgtZ3mCbtQaO17LoihnqPn4UOMr\n\
            V75R/7FjSuPLS8NaZF4wfi52btXMSxO/u7GuoJkzJscP3p4qtwe6Rl9dc1XC8P7k\n\
            NIbGZ5Yg5cEPcfmhgXFOhQZkD0yxcJqBUcoFpnp2vu5XJl2E5I/quIyVxUXi6O6c\n\
            /obspcvace4wy8uO0bdVhc4nJ+Rla4InVSJaUaBeiHTW8kReSFYyMmDCzLjGIu1q\n\
            doU61OM3Zv1ptsLu3gUE6GU27iWYj2RWN3e3HE4Sbd89IFwLXNdSuM0ifDLZk7AQ\n\
            WBhRhipCCgZhkj9g2NEk7jRVslti1NdN5zoQLaJNqSwO1MtxTmJ15Ksk3QP6kfLB\n\
            Q52UWybBzpaP9HEd4XnR+HuQ4k2K0ns2KgNImsNvIyFwbpMUyUWLMPimaV1DWUXo\n\
            5SBjDB/V/W2JBFR+XKHFJeFwYhj7DD/ocsGr4ZMx/lgc8rjIBkI=\n\
            =lgTX\n\
            -----END PGP SIGNATURE-----"
                    .to_string(),
            ),
            ("message".to_string(), "Create first draft".to_string()),
        ]);

        let kvl = KeyValueList::parse(BufReader::new(raw.as_bytes())).unwrap();

        assert_eq!(parsed_key_values, *kvl.as_ref());
    }

    #[test]
    fn serialize_should_work_if_it_doesnt_contain_message() {
        let map = BTreeMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3\nvalue3-1\nvalue3-2".to_string()),
        ]);

        let kvl = KeyValueList::new(map);
        let serialized = kvl.serialize();

        let expected = r#"key1 value1
key2 value2
key3 value3
 value3-1
 value3-2
"#;

        assert_eq!(expected, serialized);
    }

    #[test]
    fn serialize_should_work_if_it_contains_message() {
        let map = BTreeMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3\nvalue3-1\nvalue3-2".to_string()),
            ("message".to_string(), "khar gav".to_string()),
        ]);

        let kvl = KeyValueList::new(map);
        let serialized = kvl.serialize();

        let expected = r#"key1 value1
key2 value2
key3 value3
 value3-1
 value3-2

khar gav"#;

        assert_eq!(expected, serialized);
    }
}
