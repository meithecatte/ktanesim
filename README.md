# KTaNE Simulator

This repository contains the source code for KTaNE Simulator, the Discord bot that simulates
[Keep Talking and Nobody Explodes] bombs.

[Keep Talking and Nobody Explodes]: https://keeptalkinggame.com/

## Project organization

The code is split into a few separate crates.

 - `ktanesim` is the main bot binary. Everything specific to Discord or image rendering can be
   found here.
 - `ktane-utils` is a semi-independent library for ruleset, module instance and edgework generation
   that encapsulates the business logic of the modules.
 - `ktanesim-logging` handles the user-facing logging.
 - `ktanesim-website` (TODO) handles the web-facing leaderboard and logs, as well as serves the
   static help.

## Licensing

The code in the `ktane-utils` directory is licensed under either of
 - [Apache License, Version 2.0][apache]
 - [MIT License](ktane-utils/LICENSE-MIT)

at your option.

The image files found in `ktanesim/src/edgework/` are licensed under
[The Keep Talking and Nobody Explodes ModKit License](ktanesim/src/edgework/LICENSE-ART).

All other code is licensed under the terms of the [GNU Affero General Public License](LICENSE).

As a notable consequence of this fact, should you run a modified version of the bot, **you must
make the modified source code available to everyone who interacts with the bot**. A good way to
do this is to set up the `!source` command to link to your code.

**TODO**: explain how.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the KTaNE Simulator project by you, as defined in the [Apache-2.0 license][apache], shall be
licensed as above, without any additional terms or conditions.

[apache]: ktane-utils/LICENSE-APACHE
