# Nightbot API for Age of Empires 2 info

## Resource endpoints
* `/elo/<user>[?ladder=...]`
	* Checks the Voobly elo of the specified user
	* If ladder is given, it looks up elo in the specified ladder
	* Ladder can be one of `rmtg`, `dm1v1`, `dmtg` and defaults to `rm1v1`
* `/tech/<tech>`
	* Returns info for the specified tech
* `/unit/<unit>`
	* Returns info for the specified unit
* `/building/<building>`
	* Returns info for the specified building
* `/available/<civ>/<tech,unit,building>`
	* Checks if the specified civ has a tech / unit / building

All resources require the Nightbot headers in production environment.

## Building / deploying
Run debug build on port 8000:
```
cargo run
```

Currently configured to be deployed on Heroku (see [Procfile](Procfile)).
Easiest way is to use the [Rust buildpack for Heroku](https://github.com/emk/heroku-buildpack-rust).

---

Contributions welcome!
