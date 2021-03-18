# productivity-bot

A binary crate for generating Windows 10 toast notifications on upcoming google calendar events

# Motivation

One of my biggest issues from adapting to the work from home environemnt has been getting distracted with my work and showing up a couple minutes late to zoom meetings. Due to the fairly complicated nature of my work dev environment, the standard chrome toast notifications that google calendar uses are incredibly unreliable and insufficient for reminding me about upcoming meetings. It would be much easier if I could leverage the windows notification system on my local machine and provide it credentials that have access to my work calendar. This crate will poll the google calendar API every 60 seconds and look for any new events it has not already generated notifications for. For any new events it will create a windows 10 toast notification containing the event summary

# Requirements

- Windows 10 environment
- [Google OAuth2 Credentials](https://www.daimto.com/google-3-legged-oauth2-flow/) for the account of interest:
    - Client id
    - Client secret
    - Refresh token

# Usage

```
USAGE:
    bot.exe [OPTIONS] --client_id <client_id> --client_secret <client_secret> --refresh_token <refresh_token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --client_id <client_id>            Google client id
    -s, --client_secret <client_secret>    Google client secret
    -r, --refresh_token <refresh_token>    Google refresh token
    -t, --reminder_time <reminder_time>    How many seconds in advance you want to be reminded (Default 60s)
```