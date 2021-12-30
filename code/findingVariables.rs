fn get_variable_clone(&self, key: &str) -> Result<String> {
    if key.len() >= 4 && &key[0..4] == "req." {
        let key = &key[4..];
        return self
            .request_map
            .get(key)
            .map(|it| it.clone())
            .ok_or(anyhow!("Request key {} not found", key));
    } else if key.len() >= 6 && &key[0..6] == "super." {
        let mut counter = 0_usize;
        let mut inner_key = key;

        while inner_key.len() >= 6 && &inner_key[0..6] == "super." {
            inner_key = &inner_key[6..];
            counter += 1;
        }

        if counter > self.execution_maps.len() {
            return Err(anyhow!(
                "Too many 'super.'s, reached negative index ({})",
                key
            ));
        }

        let vec_index = self.execution_maps.len() - counter;
        let map = &self.execution_maps[vec_index];
        return map
            .get(inner_key)
            .map(|it| it.clone())
            .ok_or(anyhow!("Execution key {} not found", key));
    } else {
        return Err(anyhow!(
            "Bad variable name ({}). Should begin with super. or req.",
            key
        ));
    }
}
