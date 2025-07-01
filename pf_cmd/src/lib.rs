use clap::Parser;
use futures_util::pin_mut;
use futures_util::StreamExt;
use regex::Regex;

fn validate_date(val: &str) -> Result<String, String> {
    let datetime_regex = Regex::new(
        r"^\d{4}-\d{2}-\d{2}[Tt ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}(?::\d{2})?)?$",
    )
    .unwrap();

    if datetime_regex.is_match(val) {
        Ok(val.to_string())
    } else {
        Err(String::from("Invalid date format. Use YYYY-MM-DDTHH:MM:SS plus optional fractional seconds plus an optional timezone specifier in the form Z, +XX, -XX, +XX:XX, or -XX:XX (cf. <https://core.trac.wordpress.org/ticket/41032>)."))
    }
}

/// Scans WordPress websites to find videos.
///
/// Supported MIME types: video/mp4 and video/quicktime (.mov).
#[derive(Parser)]
#[command(name = "pf", author, version, about)]
pub struct Opt {
    /// WordPress base URL (e.g. <http://example.com>).
    pub url: String,

    /// Result set published before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long, value_parser=validate_date)]
    pub before: Option<String>,

    /// Result set modified before a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long, value_parser=validate_date)]
    pub modified_before: Option<String>,

    /// Result set published after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long, value_parser=validate_date)]
    pub after: Option<String>,

    /// Result set modified after a given date (cf. <https://core.trac.wordpress.org/ticket/41032>).
    #[arg(long, value_parser=validate_date)]
    pub modified_after: Option<String>,

    /// Ensures result set excludes specific IDs.
    #[arg(long)]
    pub exclude: Vec<u16>,

    /// Ensures result set excludes specific category IDs.
    #[arg(long)]
    pub categories_exclude: Vec<u16>,

    /// Ensures result set excludes to specific tag IDs.
    #[arg(long)]
    pub tags_exclude: Vec<u16>,
}

impl Opt {
    /// Converts the `Opt` struct to a `FinderConfig` struct.
    fn to_finder_config(&self) -> pf_lib::FinderConfig {
        pf_lib::FinderConfig {
            url: self.url.clone(),
            target: pf_lib::FinderTarget::Posts {
                categories_exclude: self.categories_exclude.clone(),
                tags_exclude: self.tags_exclude.clone(),
            },
            before: self.before.clone(),
            modified_before: self.modified_before.clone(),
            after: self.after.clone(),
            modified_after: self.modified_after.clone(),
            exclude: self.exclude.clone(),
        }
    }
}

/// Runs the `pf` command.
pub async fn run(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = opt.to_finder_config();
    print_stream(&config).await?;
    config.target = pf_lib::FinderTarget::Media;
    print_stream(&config).await?;
    Ok(())
}

/// Consumes and prints the `find` stream.
async fn print_stream(config: &pf_lib::FinderConfig) -> Result<(), Box<dyn std::error::Error>> {
    let stream = pf_lib::find(config);
    pin_mut!(stream);
    while let Some(res) = stream.next().await {
        match res {
            Ok(url) => println!("{url}"),
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_date() {
        assert!(validate_date("2023-01-01T00:00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00Z").is_ok());
        assert!(validate_date("2023-01-01T00:00:00+00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00-00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00+00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00-00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00+00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00-00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123Z").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123+00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123-00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123+00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123-00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123+00:00").is_ok());
        assert!(validate_date("2023-01-01T00:00:00.123-00:00").is_ok());
        assert!(validate_date("2023-01-01").is_err());
        assert!(validate_date("2023-01-01T00:00:00+").is_err());
        assert!(validate_date("2023-01-01T00:00:00-").is_err());
        assert!(validate_date("2023-01-01T00:00:00+00:").is_err());
        assert!(validate_date("2023-01-01T00:00:00-00:").is_err());
        assert!(validate_date("2023-01-01T00:00:00+00:00:00").is_err());
    }

    #[test]
    fn test_opt_parsing() {
        let args = vec![
            "pf",
            "http://example.com",
            "--before",
            "2023-01-01T00:00:00",
            "--modified-before",
            "2023-01-01T00:00:00",
            "--after",
            "2023-01-01T00:00:00",
            "--modified-after",
            "2023-01-01T00:00:00",
            "--exclude",
            "1",
            "--exclude",
            "2",
            "--categories-exclude",
            "3",
            "--tags-exclude",
            "4",
        ];
        let opt = Opt::parse_from(args);
        assert_eq!(opt.url, "http://example.com");
        assert_eq!(opt.before, Some("2023-01-01T00:00:00".to_string()));
        assert_eq!(opt.modified_before, Some("2023-01-01T00:00:00".to_string()));
        assert_eq!(opt.after, Some("2023-01-01T00:00:00".to_string()));
        assert_eq!(opt.modified_after, Some("2023-01-01T00:00:00".to_string()));
        assert_eq!(opt.exclude, vec![1, 2]);
        assert_eq!(opt.categories_exclude, vec![3]);
        assert_eq!(opt.tags_exclude, vec![4]);
    }

    #[test]
    fn test_to_finder_config() {
        let opt = Opt {
            url: "http://example.com".to_string(),
            before: Some("2023-01-01T00:00:00".to_string()),
            modified_before: Some("2023-01-01T00:00:00".to_string()),
            after: Some("2023-01-01T00:00:00".to_string()),
            modified_after: Some("2023-01-01T00:00:00".to_string()),
            exclude: vec![1, 2],
            categories_exclude: vec![3],
            tags_exclude: vec![4],
        };
        let config = opt.to_finder_config();
        assert_eq!(config.url, "http://example.com");
        assert_eq!(config.before, Some("2023-01-01T00:00:00".to_string()));
        assert_eq!(
            config.modified_before,
            Some("2023-01-01T00:00:00".to_string())
        );
        assert_eq!(config.after, Some("2023-01-01T00:00:00".to_string()));
        assert_eq!(
            config.modified_after,
            Some("2023-01-01T00:00:00".to_string())
        );
        assert_eq!(config.exclude, vec![1, 2]);
        assert_eq!(
            config.target,
            pf_lib::FinderTarget::Posts {
                categories_exclude: vec![3],
                tags_exclude: vec![4],
            }
        );
    }

    #[tokio::test]
    async fn test_print_stream() {
        let config = pf_lib::FinderConfig {
            url: "http://example.com".to_string(),
            target: pf_lib::FinderTarget::Posts {
                categories_exclude: vec![],
                tags_exclude: vec![],
            },
            before: None,
            modified_before: None,
            after: None,
            modified_after: None,
            exclude: vec![],
        };
        let result = print_stream(&config).await;
        assert!(result.is_ok());
    }
}
