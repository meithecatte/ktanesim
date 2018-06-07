import discord
import asyncio
import logging
import leaderboard

try:
	from config import *
except:
	raise Exception("Copy config.template to config.py and edit the values!")

logging.basicConfig(level=logging.INFO)
client = discord.Client()

@client.event
async def on_ready():
	print('Logged in:', client.user.id, client.user.name)

@client.event
async def on_message(msg):
	if type(msg.channel) is not discord.channel.DMChannel and msg.channel.id not in ALLOWED_CHANNELS: return
	if not msg.content.startswith(PREFIX): return
	parts = msg.content[1:].strip().split()
	command = parts.pop(0)
	if command == "solve":
		leaderboard.record_solve(msg.author, int(parts[0]))
	elif command == "strike":
		leaderboard.record_strike(msg.author, int(parts[0]))
	elif command == "penalty":
		leaderboard.record_penalty(msg.author, int(parts[0]))
	elif command in ["leaderboard", "lb"]:
		if not parts:
			page = 1
		elif len(parts) > 1 or not parts[0].isdigit() or parts[0] == '0':
			await msg.channel.send("%s Usage: `%sleaderboard [page number]`. Default: page 1. Aliases: `%slb`" % (msg.author.mention, PREFIX, PREFIX))
			return
		else:
			page = int(parts[0])

		await msg.channel.send(leaderboard.format_page(page, LEADERBOARD_PAGE_SIZE))
	else:
		await msg.channel.send("%s No such command: `%s`. Type %shelp for help" % (msg.author.mention, command, PREFIX))

client.run(TOKEN)
