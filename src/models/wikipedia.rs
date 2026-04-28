use anyhow::anyhow;
use serde::Deserialize;
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct PageResponse {
    pub source: String
}

impl PageResponse {
    pub fn get_names(self: &Self) -> anyhow::Result<Vec<String>> {
        let pat = Regex::new(r"(?:\[\[Imieniny\]\])([\s\S]*?)\n").unwrap();
        let names_source = pat
            .captures(&self.source).ok_or(anyhow!("Namedays not found"))?
            .get(0).ok_or(anyhow!("Invalid namedays format"))?
            .as_str();

        let pat = Regex::new(r"\[\[(?:[^\[\]\|]*\|)?(?<name>[^\[\]\|]+)\]\]").unwrap();
        let mut names: Vec<String> = Vec::new();
        for caps in pat.captures_iter(names_source) {
            let name = if let Some(x) = caps.name("name") {
                x.as_str().to_string()
            } else {
                continue
            };
            names.push(name);
        }
        return Ok(names);
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_weather_parsing() {
        let example_wiki = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example_wiki.json"));
        let parsed = serde_json::from_str::<PageResponse>(example_wiki);
        assert!(parsed.is_ok(), "example wiki page should parse");
        let names = parsed.unwrap().get_names();
        assert!(names.is_ok(), "example wiki names should parse");
    }
}
