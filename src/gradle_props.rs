use std::collections::HashMap;

pub struct GradleProperties {
    pub keys: Vec<String>,
    pub values: HashMap<String, String>,
}

impl GradleProperties {

    //TODO introduce methods load, save

    pub fn get(&self, key: &str) -> Option<&str> {
        return self.values.get(key)
            .map(|it|it.as_str())
    }

}