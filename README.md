# onair-rs
ON AIR indicator sign service for remote workers. Uses a client on your local machine to monitor for an active video conference session and messages the sign controller to turn on and off an ON AIR sign.

Current supports monitoring for Zoom & WebEx conferences, and a Scroll pHAT HD display on Raspberry Pi. Tested on a Pi Zero W.

## Running
To build and run the server for local dev

```cargo run --bin server```

You may select which display mechinism to use by passing any of the following arguments:
- `--quiet` no sign output, useful for client dev/testing
- `--I2C` use the Scroll pHAT HD display over I2C bus for sign
- `--unicode` use terminal-based Unicode display for sign
- `--term` use terminal-based ASCII display for sign

Example:
```cargo run --bin server -- --term```

Set the the ROCKET_ENV to configure for non-localhost use. See the [Rocket documentation](https://rocket.rs/v0.4/guide/configuration/) for further details.

```ROCKET_ENV=stage cargo run --bin server -- --quiet```

## Usage

### Getting Status

Send a GET request to get the current status

```curl -X GET http://localhost:8000/```

### Changing Status

Turning on sign

```curl -X POST -H "Content-Type: application/json" -d '{"onair":true}' http://localhost:8000/```

Turning off sign:

```curl -X POST -H "Content-Type: application/json" -d '{"onair":false}' http://localhost:8000/```

Use client to automatically detect ON AIR status:

```cargo run --bin client -- http://localhost:8000/```


## TODO
- [x] Add Rust client.
- [ ] Add support for detecting GoToMeeting, Skype, Slack, and Hangouts calls.
- [ ] Refine support for WebEx.
- [ ] Remove Rocket and replace with basic Hyper implementation. Rocket is overkill and too heavy for an RPi build.
- [ ] Allow the displayed message to be configured.
- [ ] Add support for alternative displays and USB-powered signs/lights.

