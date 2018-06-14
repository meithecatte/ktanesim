try:
	from config import *
except:
	raise Exception("Copy config.template to config.py and edit the values!")

import discord
import asyncio
import logging
import random
import leaderboard
import modules
from bomb import Bomb

async def cmd_help(channel, author, parts):
	await channel.send(f"\u200b\n"
		f"`{PREFIX}help`: Show this message\n"
		f"`{PREFIX}leaderboard [<page number>]`, alias `{PREFIX}lb`: Show the leaderboard, {LEADERBOARD_PAGE_SIZE} items per page. Defaults to the first page.\n"
		f"`{PREFIX}rank`: Shows your leaderboard entry.\n"
		f"`{PREFIX}run ...`: Start a bomb. Pass no parameters for usage.\n"
		f"`{PREFIX}modules`: Show a list of implemented modules.\n"
		f"`{PREFIX}unclaimed`: Shows {MAX_UNCLAIMED_LIST_SIZE} random unclaimed modules from the bomb.\n"
		f"`{PREFIX}edgework`: Show the edgework string of the bomb.\n"
		f"`{PREFIX}status`: Show the bomb status.\n"
		f"`{PREFIX}<module number> view`: Show the module and link its manual.\n"
		f"`{PREFIX}<module number> claim`: Claim the module so that only you can give it commands.\n"
		f"`{PREFIX}<module number> unclaim`: Undo a `claim` command.\n"
		f"`{PREFIX}<module number> claimview`, alias `... cv`: `claim` and `view` combined.\n"
		f"`{PREFIX}<module number> player`: Show the player who claimed the module.\n"
		f"`{PREFIX}<module number> take`: If you think someone has abondoned the module, use this command to take it over.\n"
		f"`{PREFIX}claimany`: Claim a randomly chosen module, except for a Souvenir, Forget Me Not or Forget Everything.\n"
		f"`{PREFIX}claimanyview`, alias `{PREFIX}cvany`: `{PREFIX}claimany` and `view` combined.\n"
		f"`{PREFIX}claims`: Show the list of modules you have claimed.\n"
		f"Message the bot in DMs for solo play!")

logging.basicConfig(level=logging.INFO)
client = discord.Client()
Bomb.client = client

@client.event
async def on_ready():
	print('Logged in:', client.user.id, client.user.name)
	await Bomb.update_presence()

@client.event
async def on_message(msg):
	if type(msg.channel) is not discord.channel.DMChannel and msg.channel.id not in ALLOWED_CHANNELS: return
	if not msg.content.startswith(PREFIX): return
	parts = msg.content[len(PREFIX):].strip().split()
	command = parts.pop(0)
	channel = msg.channel
	author = msg.author

	GENERIC_COMMANDS = {
		"run": Bomb.cmd_run,
		"modules": modules.cmd_modules,
		"leaderboard": leaderboard.cmd_leaderboard,
		"lb": leaderboard.cmd_leaderboard,
		"rank": leaderboard.cmd_rank,
		"help": cmd_help,
		"shutdown": Bomb.cmd_shutdown
	}

	if command in GENERIC_COMMANDS:
		await GENERIC_COMMANDS[command](channel, author, parts)
	elif command.isdigit() or command in Bomb.COMMANDS:
		if channel.id in Bomb.bombs:
			await Bomb.bombs[channel.id].handle_command(command, author, parts)
		else:
			await channel.send(f"{author.mention} No bomb is currently ticking in this channel. Change this sad fact with `{PREFIX}run`.")
	else:
		await channel.send(f"{author.mention} No such command: `{PREFIX}{command}`. Try `{PREFIX}help` for help.")

client.run(TOKEN)
