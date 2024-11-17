use anyhow::{bail, Context, Result};
use bsky_sdk::{
    agent::config::{Config, FileStore},
    api::{
        app::bsky::feed::{defs::FeedViewPostData, get_list_feed},
        types::{LimitedNonZeroU8, Object},
    },
    BskyAgent,
};
use clap::Parser;
use ipld_core::ipld::Ipld;
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::create_dir_all, ops::Deref, path::PathBuf};
use url::Url;

const PROJECT_NAME: &str = "bsky-motd";
const AGENT_CONFIG_FILENAME: &str = "agentconfig.json";

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about)]
struct Arguments {
    /// The base URL of the service to communicate with.
    ///
    /// Note that that you might need to delete the agentconfig.json file in `${OS_CONFIG_LOCAL}/bsky-motd/agentconfig.json`
    #[clap(
        long = "service",
        env = "BSKY_MOTD_SERVICE",
        default_value = "https://bsky.social"
    )]
    service: Url,

    /// The username or email of the account.
    #[clap(long = "identifier", env = "BSKY_MOTD_IDENTIFIER")]
    identifier: String,

    /// The app password to use for authentication.
    #[clap(long = "app-password", env = "BSKY_MOTD_APP_PASSWORD")]
    app_password: String,

    /// The AT-URL to the feed to use for fetching posts.
    #[clap(long = "feed-at-url", env = "BSKY_MOTD_FEED_AT_URL")]
    feed_at_url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    let bsky = BskyHandler::new(args.service).await?;
    bsky.login(args.identifier, args.app_password).await?;

    // Fetch posts from list and filter down to just 1.
    let list_feed = bsky.get_list_feed(args.feed_at_url).await?;
    let filtered_posts = list_feed
        .feed
        .clone()
        .into_iter()
        .filter(|f| f.reply.is_none() && f.post.embed.is_none())
        .collect::<Vec<Object<FeedViewPostData>>>();
    let post = filtered_posts
        .choose(&mut thread_rng())
        .context("filtered posts returned an empty value")?;

    match &post.post.record {
        bsky_sdk::api::types::Unknown::Object(obj) => {
            let Some(data) = obj.get("text") else {
                bail!("Fetched post was malfored (No 'text' in record data)")
            };
            let text = match data.deref() {
                Ipld::String(txt) => txt,
                _ => bail!("Fetched post was malfored ('text' field was not of type Ipld::String)"),
            };
            println!(
                r#"🦋 "{text}" - {} (@{})"#,
                post.post.author.display_name.as_ref().map_or("", |v| v),
                post.post.author.handle.as_str()
            );
        }
        _ => bail!("Returned post record was not an object and could not be parsed"),
    }

    Ok(())
}

struct BskyHandler {
    agent: BskyAgent,
    config_file: PathBuf,
}

impl BskyHandler {
    pub async fn new(service: Url) -> Result<Self> {
        let agent_config_file = dirs::config_local_dir()
            .context("unable to find system local configuration directory")?
            .join(PROJECT_NAME)
            .join(AGENT_CONFIG_FILENAME);
        create_dir_all(
            agent_config_file
                .parent()
                .context("failed to find configuration file parent")?,
        )
        .context("failed to create configuration directory")?;

        let config = Config::load(&FileStore::new(&agent_config_file))
            .await
            .unwrap_or(Config {
                endpoint: service
                    .to_string()
                    .strip_suffix("/")
                    .map_or(service.into(), |s: &str| s.into()),
                ..Default::default()
            });
        let agent = BskyAgent::builder().config(config).build().await?;

        Ok(Self {
            agent,
            config_file: agent_config_file,
        })
    }

    pub async fn login(&self, identifier: String, password: String) -> Result<()> {
        self.agent.login(identifier, password).await?;
        self.agent
            .to_config()
            .await
            .save(&FileStore::new(&self.config_file))
            .await?;
        Ok(())
    }

    pub async fn get_list_feed(
        &self,
        feed_proto_url: String,
    ) -> Result<Object<get_list_feed::OutputData>> {
        let feed = self
            .agent
            .api
            .app
            .bsky
            .feed
            .get_list_feed(Object::from(
                bsky_sdk::api::app::bsky::feed::get_list_feed::ParametersData {
                    list: feed_proto_url,
                    cursor: None,
                    limit: Some(LimitedNonZeroU8::<100>::MAX),
                },
            ))
            .await?;
        Ok(feed)
    }
}
