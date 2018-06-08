import discord
import asyncio
import logging
import random
import leaderboard
from bomb import Bomb
from modules.wires import Wires
from modules.the_button import TheButton

try:
	from config import *
except:
	raise Exception("Copy config.template to config.py and edit the values!")

VANILLA_MODULES = {
	"wires": Wires
}

MODDED_MODULES = {
	"theButton": TheButton
}

logging.basicConfig(level=logging.INFO)
client = discord.Client()
bombs = {}

@client.event
async def on_ready():
	print('Logged in:', client.user.id, client.user.name)

@client.event
async def on_message(msg):
	if type(msg.channel) is not discord.channel.DMChannel and msg.channel.id not in ALLOWED_CHANNELS: return
	if not msg.content.startswith(PREFIX): return
	parts = msg.content[len(PREFIX):].strip().split()
	command = parts.pop(0)
	if command == "run":
		if msg.channel.id in bombs:
			await msg.channel.send("{:s} A bomb is already running in this channel!".format(msg.author.mention))
			return

		usage =("{:s} Usage: `{prefix}run [hummus] <module count> <module distributon> [-<module 1> [-<module 2> [...]]]` or " +
			"`{prefix}run [hummus] <module 1> [<module 2> [...]]`.\n" +
			"For example:\n - `{prefix}run hummus 7 vanilla` - 7 vanilla modules that use the modified manual by LtHummus\n" +
			" - `{prefix}run 12 mixed -souvenir -forgetEverything -forgetMeNot` - 12 modules, each having a 50% chance of " +
			"being a vanilla module. Souvenir, Forget Everything and Forget Me Not modules will not be generated\n" +
			" - `{prefix}run marbleTumble` - a single Marble Tumble module and nothing else\n" +
			" - `{prefix}run hummus complicatedWires morseCode` - two modules: Complicated Wires and Morse Code. Both use LtHummus's manual\n" +
			"Use `!modules` to see the implemented modules.\nAvailable distributions:")

		distributions = {
			"vanilla": 1,
			"mods": 0,
			"mixed": 0.5,
			"lightmixed": 0.67,
			"mixedlight": 0.67,
			"heavymixed": 0.33,
			"mixedheavy": 0.33,
			"light": 0.8,
			"heavy": 0.2,
			"extralight": 0.9,
			"extraheavy": 0.1
		}

		for distribution in distributions:
			if distribution not in ["lightmixed", "heavymixed"]:
				vanilla = int(distributions[distribution] * 100)
				usage += "\n`{:s}`: {:d}% vanilla, {:d}% modded".format(distribution, vanilla, 100 - vanilla)

		if len(parts) < 1:
			await msg.channel.send(usage.format(msg.author.mention, prefix=PREFIX))
			return

		hummus = parts[0] == "hummus"
		if hummus: parts.pop(0)

		if parts[0].isdigit():
			if len(parts) < 2 or parts[1] not in distributions:
				await msg.channel.send(usage.format(msg.author.mention, prefix=PREFIX))
				return

			module_candidates_vanilla = VANILLA_MODULES
			module_candidates_modded = MODDED_MODULES
			module_count = int(parts[0])

			if module_count == 0:
				await msg.channel.send("{:s} What would it even mean for a bomb to have no modules? :thinking:".format(msg.author.mention))
				return

			for veto in parts[2:]:
				if not veto.startswith('-'):
					await msg.channel.send(usage.format(msg.author.mention, prefix=PREFIX))
					return

				veto = veto[1:]
				
				if veto in module_candidates_vanilla:
					del module_candidates_vanilla[veto]
				elif veto in module_candidates_modded:
					del module_candidates_modded[veto]
				else:
					await msg.channel.send("{:s} No such module: `{:s}`".format(msg.author.mention, veto))
					return

			modules = []

			for _ in range(module_count):
				if random.random() < distributions[parts[1]]:
					modules.append(random.choice(list(module_candidates_vanilla.values())))
				else:
					modules.append(random.choice(list(module_candidates_modded.values())))
		else:
			modules = []
			module_candidates = VANILLA_MODULES + MODDED_MODULES
			for module in parts:
				if module not in module_candidates:
					await msg.channel.send("{:s} No such module: `{:s}`".format(msg.author.mention, module))
					return
				modules.append(module_candidates[module])

		bomb = Bomb(modules, hummus)
		bombs[msg.channel.id] = bomb
		await msg.channel.send("A bomb with {:d} modules has been armed!\nEdgework: `{:s}`".format(len(bomb.modules), bomb.get_edgework()))
	elif command == "edgework":
		if msg.channel.id in bombs:
			await msg.channel.send("{:s} Edgework: `{:s}`".format(msg.author.mention, bombs[msg.channel.id].get_edgework()))
		else:
			await msg.channel.send("{:s} No bomb is currently running in this channel.".format(msg.author.mention))
	elif command == "modules":
		await msg.channel.send("Available modules:\nVanilla: `" + '`, `'.join(VANILLA_MODULES.keys()) + "`\nModded: `" + '`, `'.join(MODDED_MODULES.keys()) + '`')
	elif command == "unclaimed":
		if msg.channel.id in bombs:
			bomb = bombs[msg.channel.id]
			modules = filter(lambda module: not module.claim, bomb.modules)

			if len(modules) > MAX_UNCLAIMED_LIST_SIZE:
				reply = 'A random sample of unclaimed modules:'
				modules = random.sample(modules, MAX_UNCLAIMED_LIST_SIZE)
			reply = 'Modules:'
			for index, module in enumerate(bomb.modules):
				if module.solved:
					status = 'solved'
				elif module.claim:
					status = 'claimed by {:s}'.format(module.claim)
				else:
					status = 'unclaimed'
				reply += "\n{:d}. {:s} - {:s}".format(index + 1, module.display_name, status)
			await msg.channel.send(reply)
		else:
			await msg.channel.send("{:s} No bomb is currently running in this channel.".format(msg.author.mention))
	elif command in ["leaderboard", "lb"]:
		if not parts:
			page = 1
		elif len(parts) > 1 or not parts[0].isdigit() or parts[0] == '0':
			await msg.channel.send("{:s} Usage: `{prefix}leaderboard [<page number>]`. Default: page 1. Aliases: `{prefix}lb`".format(msg.author.mention, prefix=PREFIX))
			return
		else:
			page = int(parts[0])

		await msg.channel.send(leaderboard.format_page(page, LEADERBOARD_PAGE_SIZE))
	elif command == "rank":
		await msg.channel.send(leaderboard.format_rank(msg.author))
	elif command == "help":
		await msg.channel.send((
			"`{prefix}help`: Show this message\n" +
			"`{prefix}leaderboard [<page number>]`: Show the leaderboard, {:d} items per page. Defaults to the first page.\n" +
			"`{prefix}lb`: Alias for `{prefix}leaderboard`.\n" +
			"`{prefix}run ...`: Start a bomb. Pass no parameters for usage.\n" +
			"`{prefix}modules`: Show a list of implemented modules.\n" +
			"`{prefix}unclaimed`: Shows {:d} random unclaimed modules from the bomb.\n" +
			"`{prefix}edgework`: Show the edgework string of the bomb.\n" +
			"`{prefix}<module number> view`: Show the module and link its manual.\n" +
			"`{prefix}<module number> claim`: Claim the module so that only you can give it commands.\n" +
			"`{prefix}<module number> claimview`: `claim` and `view` combined.\n" +
			"`{prefix}<module number> cv`: Alias for `claimview`.\n" +
			"`{prefix}<module number> take`: Request that a module is unclaimed.\n" +
			"`{prefix}<module number> mine`: Confirm you are still working on the module, disarming the `take` countdown.\n" +
			"`{prefix}claimany`: Claim a randomly chosen module, except for a Souvenir, Forget Me Not or Forget Everything.\n" +
			"`{prefix}claimanyview`: `{prefix}claimany` and `view` combined.\n" +
			"`{prefix}cvany`: Alias for `{prefix}claimanyview`.\n" +
			"`{prefix}claims`: Show the list of modules you have claimed.\n" +
			"`{prefix}rank`: Shows your leaderboard entry.\n"
		).format(LEADERBOARD_PAGE_SIZE, MAX_UNCLAIMED_LIST_SIZE, prefix=PREFIX))
	else:
		await msg.channel.send("{:s} No such command: `{prefix}{:s}`. Try `{prefix}help` for help.".format(msg.author.mention, command, prefix=PREFIX))

client.run(TOKEN)
