use {
    anyhow::*,
    yaml_rust::{yaml::*},
};

pub fn apply_patch(base: &mut Yaml, mut patch: Vec<Yaml>) -> Result<()> {
    for item in patch.drain(..) {
        if let Yaml::Hash(hash) = item {
            apply_patch_hash(base, hash)?;
        } else {
            bail!("Unexpected patch root item: {:?}", item);
        }
    }
    Ok(())
}
pub fn apply_patch_hash(item: &mut Yaml, mut hash: Hash) -> Result<()> {
    while let Some((key, value)) = hash.pop_front() {
        if let Some(key) = key.as_str() {
            apply_change(item, key, value)?;
        } else {
            bail!("Unexpected key type in patch: {:?}", key);
        }
    }
    Ok(())
}

pub fn apply_change(mut item: &mut Yaml, path: &str, new_value: Yaml) -> Result<()> {
    let mut iter = path.split(' ').peekable();
    loop {
        if let Some(key) = iter.next() {
            if iter.peek().is_none() {
                // last : we assign
                match item {
                    Yaml::Hash(hash) => {
                        hash.insert(Yaml::String(key.to_string()), new_value);
                        return Ok(());
                    }
                    _ => {
                        bail!("map expected for assignation of {:?}", key);
                    }
                }
            } else {
                // not last: we go deeper
                let idx = key.parse::<usize>();
                match idx {
                    Ok(idx) => { // it's an array index
                        match item {
                            Yaml::Array(array) => {
                                item = array
                                    .get_mut(idx)
                                    .ok_or_else(|| anyhow!("Path can't be followed: {:?}", path))?;
                            }
                            _ => {
                                bail!("array expected in {:?}", key);
                            }
                        }
                    }
                    _ => { // it's a map key
                        match item {
                            Yaml::Hash(hash) => {
                                item = hash
                                    .get_mut(&Yaml::String(key.to_string()))
                                    .ok_or_else(|| anyhow!("Path can't be followed: {:?}", path))?;
                            }
                            _ => {
                                bail!("map expected in {:?}", key);
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    eprintln!("failed to assign {:?}", path);
    Ok(())
}


