try:
	from config import *
except:
	raise Exception("Copy config.template to config.py and edit the values!")

import discord
import asyncio
import logging
import random
import leaderboard
import bomb_manager

async def cmd_help(msg, parts):
	await msg.channel.send((
		"`{prefix}help`: Show this message\n" +
		"`{prefix}leaderboard [<page number>]`: Show the leaderboard, {:d} items per page. Defaults to the first page.\n" +
		"`{prefix}lb`: Alias for `{prefix}leaderboard`.\n" +
		"`{prefix}rank`: Shows your leaderboard entry.\n" +
		"`{prefix}run ...`: Start a bomb. Pass no parameters for usage.\n" +
		"`{prefix}modules`: Show a list of implemented modules.\n" +
		"`{prefix}unclaimed`: Shows {:d} random unclaimed modules from the bomb.\n" +
		"`{prefix}edgework`: Show the edgework string of the bomb.\n" +
		"`{prefix}status`: Show the bomb status.\n" +
		"`{prefix}<module number> view`: Show the module and link its manual.\n" +
		"`{prefix}<module number> claim`: Claim the module so that only you can give it commands.\n" +
		"`{prefix}<module number> unclaim`: Undo a `claim` command.\n" +
		"`{prefix}<module number> claimview`: `claim` and `view` combined.\n" +
		"`{prefix}<module number> cv`: Alias for `claimview`.\n" +
		"`{prefix}<module number> player`: Show the player who claimed the module.\n" +
		"`{prefix}<module number> take`: Request that a module is unclaimed.\n" +
		"`{prefix}<module number> mine`: Confirm you are still working on the module, disarming the `take` countdown.\n" +
		"`{prefix}claimany`: Claim a randomly chosen module, except for a Souvenir, Forget Me Not or Forget Everything.\n" +
		"`{prefix}claimanyview`: `{prefix}claimany` and `view` combined.\n" +
		"`{prefix}cvany`: Alias for `{prefix}claimanyview`.\n" +
		"`{prefix}claims`: Show the list of modules you have claimed.\n" +
		"Message the bot in DMs for solo play!"
	).format(LEADERBOARD_PAGE_SIZE, MAX_UNCLAIMED_LIST_SIZE, prefix=PREFIX))

COMMANDS = {
	"run": bomb_manager.cmd_run,
	"edgework": bomb_manager.cmd_edgework,
	"modules": bomb_manager.cmd_modules,
	"unclaimed": bomb_manager.cmd_unclaimed,
	"status": bomb_manager.cmd_status,
	"claims": bomb_manager.cmd_claims,
	"claimany": bomb_manager.cmd_claimany,
	"claimanyview": bomb_manager.cmd_claimanyview,
	"cvany": bomb_manager.cmd_claimanyview,
	"leaderboard": leaderboard.cmd_leaderboard,
	"lb": leaderboard.cmd_leaderboard,
	"rank": leaderboard.cmd_rank,
	"help": cmd_help
}

logging.basicConfig(level=logging.INFO)
client = discord.Client()
bomb_manager.client = client

@client.event
async def on_ready():
	print('Logged in:', client.user.id, client.user.name)
	await bomb_manager.update_presence()

@client.event
async def on_message(msg):
	if type(msg.channel) is not discord.channel.DMChannel and msg.channel.id not in ALLOWED_CHANNELS: return
	if not msg.content.startswith(PREFIX): return
	parts = msg.content[len(PREFIX):].strip().split()
	command = parts.pop(0)
	if command in COMMANDS:
		await COMMANDS[command](msg, parts)
	elif command.isdigit():
		await bomb_manager.handle_module_command(msg, int(command), parts)
	else:
		await msg.channel.send("{:s} No such command: `{prefix}{:s}`. Try `{prefix}help` for help.".format(msg.author.mention, command, prefix=PREFIX))

client.run(TOKEN)
