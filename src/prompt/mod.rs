use dialoguer::Select;

pub struct Prompt {
    pub input: String,
}

impl Prompt {
    pub fn new(prompt: &str, items: Vec<&str>) -> Self {
        let selected = Select::new()
            .with_prompt(prompt)
            .items(&items)
            .interact()
            .unwrap();

        return Self {
            input: String::from(items[selected]),
        };
    }
}
