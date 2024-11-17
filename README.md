# Bluesky MOTD 

Get a message of the day in your terminal from a specified Bluesky list, made entirely as a joke.

## Installing

You can build and install this tool using Cargo by running the following

```
cargo install --git https://github.com/Blooym/bsky-motd.git
```

## Usage

If you want bsky-motd to show a message every time a new shell is created, you'll need to add it to your shells configuration file. It is up to you to figure out how to do this, however it is very simple.

You'll need to pass the configuration flags at shell startup or make sure the environment variables have been set beforehand.

## Configuration

All configuration is handled either by command line arguments or environment variables. See below for a full list. You can run `bsky-motd --help` for an update-to-date output.

```
  --service <SERVICE>
      The base URL of the service to communicate with.
      
      Note that that you might need to delete the agentconfig.json file in `${OS_CONFIG_LOCAL}/bsky-motd/agentconfig.json`
      
      [env: BSKY_MOTD_SERVICE=]
      [default: https://bsky.social]

  --identifier <IDENTIFIER>
      The username or email of the account
      
      [env: BSKY_MOTD_IDENTIFIER=]

  --app-password <APP_PASSWORD>
      The app password to use for authentication
      
      [env: BSKY_MOTD_APP_PASSWORD=]

  --feed-at-url <FEED_AT_URL>
      The AT-URL to the feed to use for fetching posts
      
      [env: BSKY_MOTD_FEED_AT_URL=]
```

## Support

No support will be offered for this project, it likely has many things broken with it and may just decide to break one day in the future.