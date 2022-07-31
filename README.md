# Open Media Server

## How would it work

All files are located in an s3 bucket - staticly hosted.

The server provides a "configuration" file in predefined JSON schema
That includes everyting from info about the server, authentication, file structure, metadata, thumbnails, ...

To add a server to the client, you simply add the link to the configuration file or enter a domain with a TXT record that resolves to an configuration file url

The base url is specified at the root of the config file. Each movie / tv show, season and episode has a relative path based on the parent

For example, if the file is located in `https://media.com/tv_show/season_1/episode_1.mp4`, this would be the config file:

```json
{
  "base_url": "https://media.com",
  "media": [
    {
      "title": "TV Show",
      "path": "tv_show",
      "seasons": [
        {
          "title": "Season 1",
          "path": "season_1",
          "episodes": [
            {
              "title": "Episode 1",
              "path": "episode_1.mp4"
            }
          ]
        }
      ]
    }
  ]
}
```

If you don't specify the path of the movie / tvshow, season ..., it will be left out: `https://media.com/tv_show/episode_1.mp4`

```json
...
{
  "title": "Season 1",
  "episodes": [
    ...
  ],
}
...
```

Trailing slash will reset path back to base url: `https://media.com/episode_1.mp4`

```json
...
{
  "title": "Episode",
  "path": "/episode_1.mp4",
}
...
```

Paths can also be absolue if you want to link multiple hosting sites together

```json
...
{
  "title": "TV Show",
  "path": "https://other-media.com/tv_show",
  "seasons": [
    ...
  ]
}
...
```
