import discord
import asyncio
import logging
import leaderboard

ALLOWED_CHANNELS = ['453477821508091905']
PREFIX = '!'
LEADERBOARD_PAGE_SIZE = 20

with open('token') as f:
	TOKEN = f.read()

logging.basicConfig(level=logging.INFO)
client = discord.Client()

@client.event
async def on_ready():
	print('Logged in:', client.user.id, client.user.name)

@client.event
async def on_message(msg):
	if not msg.channel.is_private and msg.channel.id not in ALLOWED_CHANNELS: return
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
			page = 0
		elif len(parts) > 1 or not parts[0].isdigit():
			await client.send_message(msg.channel, "%s Usage: `%sleaderboard [page number]`. Default: page 1. Aliases: `%slb`" % (msg.author.mention, PREFIX, PREFIX))
		else:
			page = int(parts[0] - 1)

		for entry in leaderboard.get_top(page * LEADERBOARD_PAGE_SIZE, LEADERBOARD_PAGE_SIZE):
			print(repr(entry))
	else:
		await client.send_message(msg.channel, "%s No such command: `%s`. Type %shelp for help" % (msg.author.mention, command, PREFIX))

client.run(TOKEN)
