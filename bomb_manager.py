import random
import discord
import aiohttp
import traceback
import async_timeout
from config import *
from bomb import Bomb

async def unclaimed_command(msg, cmd):
	unclaimed = bombs[msg.channel.id].get_unclaimed()
	if unclaimed:
		await handle_module_command(msg, random.choice(unclaimed).ident, cmd)
	else:
		await msg.channel.send("{:s} Sorry, there are no unclaimed modules.".format(msg.author.mention))

@bomb_present
async def cmd_claimany(msg, parts):
	await unclaimed_command(msg, "claim")

@bomb_present
async def cmd_claimanyview(msg, parts):
	await unclaimed_command(msg, "claimview")

@bomb_present
async def handle_module_command(msg, ident, parts):
	bomb = bombs[msg.channel.id]
	if ident > len(bomb.modules):
		await msg.channel.send("{:s} The bomb has only {:d} modules!".format(msg.author.mention, len(bomb.modules)))
		return
	module = bomb.modules[ident - 1]
	if parts[0] == "claim":
		if await module.cmd_claim(msg):
			await msg.channel.send("{:s} {:s} is yours now.".format(msg.author.mention, str(module)))
	elif parts[0] == "unclaim":
		await module.cmd_unclaim(msg)
	elif parts[0] == "view":
		await module.cmd_view(msg, '')
	elif parts[0] in ["cv", "claimview"]:
		if await module.cmd_claim(msg):
			await module.cmd_view(msg, "{:s} {:s} is yours now.".format(msg.author.mention, str(module)))
	elif parts[0] == "player":
		if module.claim:
			await msg.channel.send("{:s} {:s} has been claimed by {:s}".format(msg.author.mention, str(module), str(module.claim)))
		else:
			await msg.channel.send("{:s} {:s} has not been claimed by anybody".format(msg.author.mention, str(module)))
	elif parts[0] == "take":
		await module.take(msg)
	elif parts[0] == "mine":
		await module.mine(msg)
	else:
		if module.solved:
			await msg.channel.send("{:s} {:s} has already been solved.".format(msg.author.mention, str(module)))
		elif module.claim and module.claim.id != msg.author.id:
			await msg.channel.send("{:s} Sorry, {:s} has been claimed by {:s}.".format(msg.author.mention, str(module), str(module.claim)))
		else:
			await module.command(msg, parts)

hastebin_session = None
async def defused(channel):
	bomb = bombs[channel.id]
	log = bomb.get_log()
	global hastebin_session
	if hastebin_session is None:
		hastebin_session = aiohttp.ClientSession()
	try:
		with async_timeout.timeout(5):
			async with hastebin_session.post('https://hastebin.com/documents', data=log.encode('utf-8')) as resp:
				decoded = await resp.json()
				if 'key' in decoded:
					logurl = 'Log: https://hastebin.com/{:s}.txt'.format(decoded['key'])
				elif 'message' in decoded:
					logurl = 'Log upload failed with error message: `{:s}`'.format(decoded['message'])
				else:
					logurl = 'Log upload failed with malformed JSON: `{:s}`'.format(repr(decoded))
	except Exception:
		logurl = 'Log upload failed with exception: ```\n{:s}```'.format(traceback.format_exc())
	await channel.send("The bomb has been defused after {:s} and {:d} strikes. {:s}".format(bomb.get_time_formatted(), bomb.strikes, logurl))
	del bombs[channel.id]
	await update_presence()

async def cmd_run(msg, parts):
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
		module_candidates = {**VANILLA_MODULES, **MODDED_MODULES}
		for module in parts:
			if module not in module_candidates:
				await msg.channel.send("{:s} No such module: `{:s}`".format(msg.author.mention, module))
				return
			modules.append(module_candidates[module])

	bomb = Bomb(modules, hummus)
	bombs[msg.channel.id] = bomb
	await msg.channel.send("A bomb with {:d} modules has been armed!\nEdgework: `{:s}`".format(len(bomb.modules), bomb.get_edgework()))
	await update_presence()
