+++
title = "Help"
insert_anchor_links = "right"
+++

# Introduction

[KTaNE Simulator][invite] is a Discord bot that simulates [Keep Talking and Nobody Explodes][ktane]
bombs right in your Discord client. Message the bot in a *Direct Message* for solo play or
collaborate in server channels (the bot may only be allowed to talk in some of them).

<!-- TODO: example embed? -->

The modules on the bomb are numbered, starting from 1. To send a command to a specific module,
prefix the command with its number. For example, `!3 view` (or `!3 v` for short) will show you a
picture of the third module on a bomb. The message will also contain a handy reference for
module-specific commands, such as `!3 cut 4` for [Wires] or `!5 hold` for [The Button].

As a shorthand, you can use a double exclamation mark to refer to the module most recently viewed
by you: `!!cut 5` will be equivalent to `!2 cut 5` if just issued a `!2 view`.

[ktane]: https://keeptalkinggame.com "Official game website"
[invite]: https://discord.gg/3DkqfZv "Discord server invite"
[Wires]: https://ktane.timwi.de/HTML/Wires.html
[The Button]: https://ktane.timwi.de/HTML/The%20Button.html

# Arming a bomb

There are two ways to start a bomb. Firstly, there are many [missions] with varying difficulty
available. However, if you prefer, you can specify a module list and defuse a truly unique bomb.

[missions]: /missions/ "Mission list"

## Starting a mission with `!mission`

The mission command will start a bomb in the channel it is issued. In its elementary form, it's
only followed by a [mission name][missions]: `!mission pick up the pace IV`

### Named parameters

It is possible to add named parameters before the mission name to control some aspects of the bomb:

- **ruleseed** (also: **seed**, **rules**) &mdash; for modules that support it, generate the rules
  procedurally with the specified seed.
  - Examples:
    - `!mission seed=7 blinkenlights`
    - `!mission ruleseed=6502 I am hardcore`
  - A seed is a positive number smaller than 2,147,483,648.
  - The bot will show you the link to the appropriate manuals.
  - This is equivalent to the [Rule Seed Modifier] mod.
- **timer** &mdash; change the behavior of the timer.
  - `!mission timer=1m30s the first bomb` &mdash; change the amount of time on the timer. This will
    make the bomb either easier or harder to defuse, and will be reflected in how much [Leaderboard]
    points will be awarded.
  - `!mission timer=zen centurion` &mdash; make the timer run up and remove the strike limit, making
    the bomb never explode. Useful if you want some chilled out puzzlesolving experience.
  - `!mission timer=time fiendish` &mdash; enable [Time Mode] <s>(explanation aped from there)</s>:
    - The timer starts at 5 minutes.
    - When a strike is earned, the timer does not speed up; instead a fixed *proportion* of the
      remaining time is deducted.
    - When a moddule is solved, additional time is added to the timer. The amount of time depends
      on the difficulty of the module solved. This way, time lost from strikes can be regained.
    - There is a "multiplier" value. With a high multiplier, the amount of time earned from solved
      module only increases it slightly. This way, strikes are still very costly and the challenge
      of solving modules reliably is maintained.

These parameters can be combined like so: `!mission timer=zen seed=7 double trouble`. Their order
does not matter.

[Rule Seed Modifier]: https://steamcommunity.com/sharedfiles/filedetails/?id=1224413364
[Time Mode]: https://ktane.timwi.de/More/FAQs.html#time-mode

## Creating custom bombs using `!run`

Should you be unsatisfied with the missions available, you can use the `!run` command to start a
custom bomb. Apart from the same [Named Parameters] as available for `!mission`, you should specify
a list of [modules] you want on the bomb.
