# TODO FOR HANIBANA

- [] Finish the rust book
- [] Learn async
- [] Learn Serenity
- [] Learn Poise
- [] Develop hanibana 

# Development process

Important extras to learn:

- [] Error handling in rust 
- [] Tracing for logging&debugging
- [] Learn how to write tests
- [] Learn tokio internals
- [] clippy and rustfmt for formatting and linting
- [] Git branching


# QOL

- [] !! command to rerun a command
- [] React with a CherryBlossom emoji when a task is done successfully
- [] Hire an artist to make a custom pfp and banner
- [] Gracefiul fallback, if a command fails to avoid crashing and tell the user something went wrong.

## Phase 1

- [] Initialise the project
- [] Add dependencies and setup `Cargo.toml`
- [] Setup .env with the bot token
- [] Basic main.rs with a test command like ping using poise.

## Phase 2

- [] Help command
- [] About command (bot info, name, version, dev, uptime, prefix)
- [] invite command | sends the bots invite link
- [] Command cooldown/rate limits
- [] Graceful startup/shutdown logs

## Phase 3

- [] Develop slash commands 
- [] Error handling
- [] Purge
- [] Kick
- [] Ban
- [] Temp ban // Basically just bans them and unbans them straight away to clear their server messages.
- [] Warning system
- [] Unban
- [] Mute/Unmute
- [] Slowmode
- [] Logging system
- [] Checks in poise for permissions

## Phase 4

- [] Roll
- [] Dice
- [] Coinflip
- [] Avatar
- [] Whois
- [] Serverinfo

## Phase 5

- [] Leveling system
- [] Save to database, maybe postgress
- [] Rank command
- [] Leaderboard command
- [] Configuration
- [] Restrict channels command
- [] Level roles
- [] Prestige system with xp boosts

### optional

- [] XP decay
- [] Custom xp formula per server
- [] Double xp at the weekend
- [] set level/xp/prestige command (admin)
- [] Streak system
- [] Xp scaling to channels, 0.8x xp for active channels and 1.2x for inactive to encourage channel use.
- [] Anti-spam to detect if someone is spamming emojis or something trying to farm xp.

## Phase 6

- [] Create all the Donator perks

## Phase 7

- [] Rillrate dashboard
- [] Web dashboard

Release somewhere around phase 5.

### Future ideas

- [] Automod
- [] Ai integration/custom ai 
- [] Reaction roles
- [] Birthday system
- [] Reminders
- [] Mini-games
- [] Strike system
- [] Afk command
- [] Forms/Appeals from the dashboard
- [] Presence logging
- [] Afk bypass command
- [] Lock command
- [] Lockdown command
- [] Create the quote system
- [] Maintainance mode
- [] Track command usage on the owner dashboard
- [] Announcements to servers with the owner dashboard
- [] Restart/Shutdown on the owner dashboard
- [] Dynamic module support
- [] Talk as outer from the owner dashboard
- [] Bot strictness
- [] Search command, basically just a more advanced search than what discord gives, something like fzf but for all the server channels.
- [] Could make the bot message the server owner in dms or something with a help command when the bot first joins, would have to be careful tho.



- [] Setup the dashboard (example in serenity)
- [] Setup premium perks
# For the premium perks i have decided to do the following:

## Donator
Price: 5 GBP - 1 time purchase

perks:

- Donator badge on whois
- A shoutout in a channel like #supporters
- More options for the background rank card on the leveling system (presets they can pick)
- Custom personality for the bot (maybe presets and way later on custom Ai)
- Ability to customize their own level up message (it will be moderated)
- Ability to customize the embed colour on commands they run
- Add a custom flair/name to their whois

## Supporter 
Price: 10 GBP - Once per month recurring payment.

Perks:

- Eveything the Donator gets (they get it permenently same as Donator even if their subscription is cancelled.)
- A custom shoutout from me thanking you!

This is basically just to support me! >3

## Sponsor
Price: 50 GBP+ - Once per month recurring payment.

Perks:

- Eveything Supporter gets 
- Added into the credits of Hanibana!

There will be a command to see all the perks, something like /perks.
Could try and make Hanibana send a thankyou message in their DMS/a channel when they donate/subscribe.

- [] Release
- [] Patches
